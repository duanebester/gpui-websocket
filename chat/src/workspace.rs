use crate::state::*;
use gpui::*;

pub struct Workspace {
    message_view: View<MessageView>,
    update_view: View<UpdateView>,
}

impl Render for Workspace {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x333333))
            .justify_center()
            .items_center()
            .child(self.message_view.clone())
            .child(self.update_view.clone())
    }
}

pub struct MessageView {
    message: SharedString,
}

impl Render for MessageView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let message = self.message.clone();
        div()
            .flex()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Latest Message: {}", message))
    }
}

impl MessageView {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let state = cx.global::<StateModel>().clone();
            let message = state.inner.read(cx).message.clone();
            cx.observe(&state.inner, |this: &mut MessageView, model, cx| {
                this.message = model.read(cx).message.clone();
                cx.notify();
            })
            .detach();

            Self { message }
        })
    }
}

pub struct UpdateView;

impl Render for UpdateView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2a2a2a))
            .text_color(rgb(0xffffff))
            .py_2()
            .px_4()
            .child("Send Message!")
            .cursor(CursorStyle::PointingHand)
            .on_mouse_down(MouseButton::Left, |_mde, cx| {
                StateModel::update(
                    |model, cx| {
                        cx.update_model(&model.inner, |_model, cx| {
                            let message = OutgoingMessage {
                                message: "Hello from the other side".into(),
                            };
                            cx.emit(message);
                        })
                    },
                    cx,
                );
            })
    }
}

impl UpdateView {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self {})
    }
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let message_view = MessageView::build(cx);
        let update_view = UpdateView::build(cx);

        cx.new_view(|_cx| Self {
            message_view,
            update_view,
        })
    }
}
