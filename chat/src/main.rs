mod app;
mod state;
mod websocket;
mod workspace;
use gpui::*;

fn main() {
    let app = App::new();
    app::run_app(app);
}
