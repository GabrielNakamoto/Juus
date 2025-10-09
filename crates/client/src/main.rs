use dioxus::prelude::*;
use std::time::{SystemTime};
use std::vec;


static CSS: Asset = asset!("/assets/style.css");

#[derive(Clone, PartialEq)]
struct Member {
	name: String
}
// Need display name + unique user id

impl Member {
	fn new(name: impl Into<String>) -> Self { 
		Self {
			name: name.into()
		}
	}
	fn send_message(&self, msg: impl Into<String>) -> Message {
		Message {
			sender: self.clone(),
			content: msg.into()
		}
	}
}

#[derive(Clone, PartialEq)]
struct Message {
	sender: Member,
	content: String
}

impl Message {
	fn view(&self) -> Element {
		rsx! {
			div {
				class: "message",
				p {
					class: "message-sender",
					"{self.sender.name}"
				},
				p {
					class: "message-content",
					"{self.content}"
				}
			}
		}
	}
}

#[derive(Clone, PartialEq)]
struct State {
	user: Member,
	messages: Vec<Message>
}

fn main() {
	#[cfg(feature="server")]
	{
		tokio::runtime::Runtime::new()
			.unwrap()
			.block_on(async move {
				launch_server().await;
			});
	}
	#[cfg(feature="web")]
    dioxus::launch(App);
}

#[cfg(feature = "server")]
async fn launch_server() {
    use dioxus::fullstack::prelude::*;

    // Connect to dioxus' logging infrastructure
    dioxus::logger::initialize_default();

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile)
    let socket_addr = dioxus::cli_config::fullstack_address_or_localhost();

    // Build a custom axum router
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::new(), App)
        .into_make_service();

    // And launch it!
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[component]
fn App() -> Element {
	let user1 = Member::new("Nolan");
	let user2 = Member::new("Christian");

    let mut state = use_signal(|| State {
		user: Member::new("Gabriel"),
        messages: vec![
			/*
			user1.send_message("sams definitely dating meaghan."),
			user2.send_message("yall seen markelys vsco?")*/
        ],
    });

    rsx! {
		document::Stylesheet { href: CSS }
		style { "@import url('https://fonts.googleapis.com/css2?family=Rubik:ital,wght@0,300..900;1,300..900&display=swap');" }
        h1 { "Juus" }
        ChatHistory { state }
    }
}

#[component]
fn ChatHistory(state: Signal<State>) -> Element {
	let mut message = use_signal(|| String::new());
	rsx! {
		div {
			class: "messages hide-scrollbar",
			div {
				class: "new-message",
				textarea {
					oninput: move |event| {
						message.set(event.value());
					},
					value: "{message.read()}"
				}
				button {
					onclick: move |_|  {
						let msg = state.read().user.send_message(message.read().to_string());
						state.write().messages.push(msg);
						message.set(String::new());
					},
					class: "submit-message",
					"Send"
				}
			},
			for item in state.read().messages.iter().rev() {
				{item.view()}
			}
		}
	}
}
