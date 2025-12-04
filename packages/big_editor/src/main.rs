mod ui;

use gpui::{
    App, AppContext, Application, ClickEvent, Context, IntoElement, ParentElement, Render,
    SharedString, Window, WindowOptions, div,
};
use gpui_component::{
    Root, StyledExt, button::{Button, ButtonVariants}, list::ListDelegate, table::{Table, TableState}
};

pub struct HelloWorld {
    big_file_paths: Vec<SharedString>,
}

impl HelloWorld {
    fn on_increment(&mut self, _: &ClickEvent, _window: &mut Window, _cx: &mut Context<Self>) {
        self.count += 1;
    }
}

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().h_flex().child(Table::new())
    }
}

struct AssetList {}

impl ListDelegate for AssetList {}

fn main() {
    TableState::ne

    Application::new().run(|cx: &mut App| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| HelloWorld { count: 0 });
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
