use dioxus::prelude::*;

pub struct Message {
	username: String,
	content: String
}

impl Message {
	pub fn new(username: String, content: String) -> Self {
		Self {
			username,
			content
		}
	}

	pub fn view(&self) -> Element {
		rsx! {
			div {
				class: "message",
				p {
					class: "message-sender",
					"{self.username}"
				},
				p {
					class: "message-content",
					"{self.content}"
				}
			}
		}
	}
}
