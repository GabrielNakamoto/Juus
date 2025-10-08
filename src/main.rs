use dioxus::prelude::*;
use std::time::{SystemTime};
use std::vec;


static CSS: Asset = asset!("/assets/style.css");


#[derive(Clone, PartialEq)]
struct Message {
	sender: String,
	content: String
}

impl Message {
	fn from(sender: impl Into<String>, content: impl Into<String>) -> Self {
		Message {
			sender: sender.into(),
			content: content.into()
		}
	}
}

#[derive(Clone, PartialEq)]
struct State {
	messages: Vec<Message>
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut state = use_signal(|| State {
        messages: vec![
            Message::from("Nolan", "sams definitely dating meaghan."),
            Message::from("Christian", "yall seen marleys vsco?"),
        ],
    });

    rsx! {
		document::Stylesheet { href: CSS }
		style { "@import url('https://fonts.googleapis.com/css2?family=Rubik:ital,wght@0,300..900;1,300..900&display=swap');" }
        h1 { "Hello, juus!" }
        p { "This is the start of something great" }
        ChatHistory { state }
    }
}

#[component]
fn ChatHistory(state: Signal<State>) -> Element {
	rsx! {
		div {
			class: "messages",
			for item in state.read().messages.iter() {
				div {
					class: "message",
					p {
						class: "message-sender",
						"{item.sender}"
					},
					p {
						class: "message-content",
						"{item.content}"
					}
				}
			}
		}
	}
}
