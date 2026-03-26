// services/websocket.rs
use yew::prelude::*;
use web_sys::{WebSocket, MessageEvent};
use shared::websocket::ServerMessage; // from shared crate

pub struct WebSocketService {
    ws: WebSocket,
    on_message: Callback<ServerMessage>,
}

impl WebSocketService {
    pub fn new(on_message: Callback<ServerMessage>) -> Self {
        let ws = WebSocket::new("ws://localhost:8080/ws").unwrap();
        let cloned_cb = on_message.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            let msg: ServerMessage = e.data().as_string().unwrap().into(); // deserialize
            cloned_cb.emit(msg);
        }) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget(); // leak for now (or store in struct)

        Self { ws, on_message }
    }
}