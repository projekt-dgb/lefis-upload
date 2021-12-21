#![windows_subsystem = "windows"]

use azul::{
    app::{App, AppConfig, LayoutSolver},
    css::Css,
    style::StyledDom,
    callbacks::{RefAny, LayoutCallbackInfo},
    window::{WindowCreateOptions, WindowFrame},
};

pub mod ui;

#[derive(Default)]
#[repr(C)]
struct Data { _min: u8 }

extern "C" fn render(_: &mut RefAny, _: &mut LayoutCallbackInfo) -> StyledDom {
    crate::ui::render()
    .style(Css::empty()) // styles are applied inline
}

fn main() {
    let app = App::new(RefAny::new(Data::default()), AppConfig::new(LayoutSolver::Default));
    let mut window = WindowCreateOptions::new(render);
    window.state.flags.frame = WindowFrame::Maximized;
    app.run(window);
}
