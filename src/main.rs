mod button;

use std::sync::{Arc, Mutex};

use button::Button;
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
        .child("Sidebar")
}

fn content(text: &str, score: Arc<Mutex<i32>>) -> Div {
    let s = *score.lock().unwrap();
    let button = Button::new().on_click(move |_cx| {
        let mut score = score.lock().unwrap();
        *score += 1;
        println!("Score: {}", *score);
    });

    let children = div()
        .child(format!("Hello, {}!", text))
        .child(button)
        .child(format!("Score: {}", s));

    div()
        .flex()
        .bg(rgb(0x2e7d32))
        .size_full()
        .justify_center()
        .items_center()
        .text_xl()
        .text_color(rgb(0xffffff))
        .child(children)
}

struct Root {
    text: SharedString,
    score: Arc<Mutex<i32>>,
}

impl Render for Root {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0xffffff))
            .size_full()
            .min_w(Pixels(360.))
            .min_h(DefiniteLength::Fraction(1.))
            .children(vec![sidebar(), content(&self.text, self.score.clone())])
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| Root {
                score: Arc::new(Mutex::new(0)),
                text: "World".into(),
            })
        });
    });
}
