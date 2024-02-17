use crate::chat::{Chat, ChatUpdated};
use gpui::*;
use tungstenite::connect;
use url::Url;

const CONNECTION: &'static str = "ws://127.0.0.1:3030/chat";

pub struct Websocket;

impl Websocket {
    pub fn listen(chat_model: Model<Chat>, cx: &mut WindowContext) {
        let (mut socket, response) =
            connect(Url::parse(CONNECTION).unwrap()).expect("Can't connect");
        println!("Connected to the server: {:?}", response.status());

        cx.spawn(|mut cx| async move {
            loop {
                let msg = socket.read().expect("Error reading message");
                match msg {
                    tungstenite::Message::Text(text) => {
                        println!("Received Text: {}", text);
                        let _ = chat_model.update(&mut cx, |_chat, cx| {
                            cx.emit(ChatUpdated {
                                message: text.clone(),
                            });
                        });
                    }
                    tungstenite::Message::Close(_) => {
                        println!("Connection closed");
                        break;
                    }
                    tungstenite::Message::Ping(_) => {
                        println!("Received Ping");
                    }
                    tungstenite::Message::Pong(_) => {
                        println!("Received Pong");
                    }
                    tungstenite::Message::Frame(_) => {
                        println!("Received Frame");
                    }
                    tungstenite::Message::Binary(_) => {
                        println!("Received Binary");
                    }
                }

                cx.background_executor()
                    .timer(std::time::Duration::from_millis(50))
                    .await;
            }

            println!("Closing socket");
            let res = socket.close(None);
            if let Err(e) = res {
                println!("Error: {:?}", e);
            }
        })
        .detach();
    }
}
