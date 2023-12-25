#![allow(non_snake_case)]

mod parse;

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use parse::java_parser::context;

fn main() {
    // launch the dioxus app in a webview
    // dioxus_desktop::launch(App);

    let string = context::read();

    println!("{}", string)
}
