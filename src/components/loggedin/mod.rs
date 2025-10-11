use dioxus::prelude::*;
use crate::state::{Client, types::message::Message};

#[component]
pub fn LoggedInApp (state: Signal<Client>) -> Element{
	rsx! {
		h1 { "Juus" }
		ChatHistory { state }
	}
}

#[component]
fn ChatHistory(state: Signal<Client>) -> Element {
	let mut message = use_signal(|| String::new());
	rsx! {
		div {
			class: "messages hide-scrollbar",
			div {
				class: "new-message",
				textarea {
					class: "no-resize",
					oninput: move |event| {
						message.set(event.value());
					},
					value: "{message.read()}"
				}
				button {
					onclick: move |_|  {
						let msg = message.read().to_string();
						state.write().request_message(Message::new(String::from("You"), msg));
						message.set(String::new());
					},
					class: "submit-message",
					"Send"
				}
			},
			for msg in state.read().messages().iter().rev() {
				{msg.view()}
			}
		}
	}
}

