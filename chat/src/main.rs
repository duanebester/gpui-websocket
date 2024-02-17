mod chat;
mod websocket;
use gpui::*;

fn main() {
    let app = App::new();
    chat::run_app(app);
}
