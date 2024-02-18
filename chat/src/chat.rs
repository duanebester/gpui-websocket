use crate::websocket::Websocket;
use gpui::*;

pub static WIDTH: f64 = 600.0;
pub static HEIGHT: f64 = 400.0;

pub struct ChatUpdated {
    pub message: String,
}

pub struct Chat {
    pub message: Option<SharedString>,
    _subscriber: Option<Subscription>,
}

impl EventEmitter<ChatUpdated> for Chat {}

impl Chat {
    pub fn build_view(chat_model: Model<Chat>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let subscription = cx.subscribe(&chat_model, |this: &mut Chat, _model, event, cx| {
                this.message = Some(event.message.clone().into());
                cx.notify();
            });

            Self {
                message: None,
                _subscriber: Some(subscription),
            }
        })
    }

    pub fn build_model(cx: &mut WindowContext) -> Model<Chat> {
        cx.new_model(|_cx| Chat {
            message: None,
            _subscriber: None,
        })
    }

    // Lovingly "lifted" from https://github.com/MatthiasGrandl/Loungy
    pub fn options(bounds: Bounds<GlobalPixels>) -> WindowOptions {
        let mut options = WindowOptions::default();
        let center = bounds.center();

        options.focus = true;
        let width = GlobalPixels::from(WIDTH);
        let height = GlobalPixels::from(HEIGHT);
        let x: GlobalPixels = center.x - width / 2.0;
        let y: GlobalPixels = center.y - height / 2.0;

        let bounds: Bounds<GlobalPixels> = Bounds::new(Point { x, y }, Size { width, height });
        options.bounds = WindowBounds::Fixed(bounds);
        options.titlebar = None;
        options.is_movable = true;
        options.kind = WindowKind::PopUp;
        options
    }
}

impl Render for Chat {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let message = self.message.clone().unwrap_or("Nothing yet.".into());
        div()
            .flex()
            .bg(rgb(0x333333))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Message: {}", message))
    }
}

pub fn run_app(app: gpui::App) {
    app.run(|cx: &mut AppContext| {
        let bounds = cx.displays().last().expect("No Display found").bounds();
        let options = Chat::options(bounds);
        cx.open_window(options, |cx| {
            let chat_model = Chat::build_model(cx);
            Websocket::listen(chat_model.clone(), cx);
            Chat::build_view(chat_model, cx)
        });
    });
}
