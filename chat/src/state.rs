use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

use gpui::*;
use tungstenite::{stream::MaybeTlsStream, WebSocket};

use crate::websocket::Websocket;

pub struct IncomingMessage {
    pub message: String,
}

pub struct OutgoingMessage {
    pub message: String,
}

pub struct State {
    pub message: SharedString,
}

#[derive(Clone)]
pub struct StateModel {
    pub inner: Model<State>,
    pub client: Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>,
}

impl EventEmitter<IncomingMessage> for State {}
impl EventEmitter<OutgoingMessage> for State {}

impl StateModel {
    pub fn init(cx: &mut WindowContext) {
        let socket = Websocket::connect();
        let socket_client = Arc::new(Mutex::new(socket));
        let model = cx.new_model(|_| State {
            message: "No messages".into(),
        });

        let client_clone = socket_client.clone();
        cx.subscribe(&model, move |_model, event: &OutgoingMessage, _cx| {
            println!("Outgoing message: {}", event.message);
            client_clone
                .lock()
                .unwrap()
                .send(tungstenite::Message::Text(event.message.clone()))
                .unwrap()
        })
        .detach();

        cx.subscribe(&model, |model, event: &IncomingMessage, cx| {
            println!("Incoming message: {}", event.message);
            let _ = cx.update_model(&model, |model, cx| {
                model.message = event.message.clone().into();
                cx.notify();
            });
        })
        .detach();

        let this = Self {
            inner: model,
            client: socket_client,
        };

        cx.set_global::<StateModel>(this);
    }

    pub fn update(f: impl FnOnce(&mut Self, &mut WindowContext), cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|mut this, cx| {
            f(&mut this, cx);
        });
    }
}

impl Global for StateModel {}
