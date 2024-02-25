use std::net::TcpStream;

use gpui::*;
use tungstenite::{stream::MaybeTlsStream, WebSocket};
use url::Url;

use crate::state::*;

const CONNECTION: &str = "ws://127.0.0.1:3030/chat";

pub struct Websocket;

impl Websocket {
    pub fn connect() -> WebSocket<MaybeTlsStream<TcpStream>> {
        let (socket, _response) =
            tungstenite::connect(Url::parse(CONNECTION).unwrap()).expect("Can't connect");

        socket
    }

    pub fn listen(cx: &mut WindowContext) {
        let state = cx.global::<StateModel>();
        let model_clone = state.inner.clone();
        let client_clone = state.client.clone();

        cx.spawn(|mut cx| async move {
            loop {
                // heartbeat
                let _ = client_clone
                    .lock()
                    .unwrap()
                    .send(tungstenite::Message::Ping(vec![0x00]));

                let msg = client_clone
                    .lock()
                    .unwrap()
                    .read()
                    .expect("Error reading message");

                match msg {
                    tungstenite::Message::Text(text) => {
                        println!("Received Text: {}", text);
                        let _ = model_clone.update(&mut cx, |_chat, cx| {
                            cx.emit(IncomingMessage {
                                message: text.clone(),
                            });
                        });
                    }
                    tungstenite::Message::Close(_) => {
                        println!("Connection closed");
                        break;
                    }
                    _ => {}
                }

                cx.background_executor()
                    .timer(std::time::Duration::from_millis(50))
                    .await;
            }

            // TODO: Handle close, reset state?
        })
        .detach();
    }
}
