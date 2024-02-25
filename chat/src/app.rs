use crate::{state::*, websocket::*, workspace::*};
use gpui::*;

pub static WIDTH: f64 = 600.0;
pub static HEIGHT: f64 = 400.0;

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

pub fn run_app(app: gpui::App) {
    app.run(|cx: &mut AppContext| {
        let bounds = cx.displays().first().expect("No Display found").bounds();
        cx.open_window(options(bounds), |cx| {
            StateModel::init(cx);
            let workspace = Workspace::build(cx);
            Websocket::listen(cx);
            workspace
        });
    });
}
