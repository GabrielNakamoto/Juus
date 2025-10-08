use dioxus::prelude::*;

struct State {
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
		h1 { "Hello, juus!" }
		p { "This is the start of something great" }
    }
}
