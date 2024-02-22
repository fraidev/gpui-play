use gpui::*;

fn sidebar() -> Div {
    div()
        .flex()
        .bg(rgb(0x000000))
        .size_full()
        .justify_center()
        .items_center()
        .text_xl()
        .text_color(rgb(0xffffff))
        .max_w(DefiniteLength::Fraction(0.2))
        .child(format!("Sidebar"))
}

fn content(text: &str) -> Div {
    div()
        .flex()
        .bg(rgb(0x2e7d32))
        .size_full()
        .justify_center()
        .items_center()
        .text_xl()
        .text_color(rgb(0xffffff))
        .child(format!("Hello, {}!", text))
}

struct Root {
    text: SharedString,
}

impl Render for Root {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0xffffff))
            .size_full()
            .min_w(Pixels(360.))
            .min_h(DefiniteLength::Fraction(1.))
            .children(vec![sidebar(), content(&self.text)])
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| Root {
                text: "World".into(),
            })
        });
    });
}
