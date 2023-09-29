#![allow(non_snake_case)]

mod parse;

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use parse::java_parser::context;

fn main() {
    // launch the dioxus app in a webview
    // dioxus_desktop::launch(App);

    context::open();
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            button {
                "Insert Test"
            }
        }
    })
}
