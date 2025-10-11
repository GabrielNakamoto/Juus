use dioxus::prelude::*;
use std::vec;

#[cfg(feature = "p2p")]
mod p2p;
mod components;
mod state;

static CSS: Asset = asset!("/assets/style.css");

pub fn main() {
    dioxus::launch(App);
}

use crate::state::Client;
#[component]
fn App() -> Element {
    let mut state = use_signal(|| Client::new());

    rsx! {
		document::Stylesheet { href: CSS }
		crate::components::loggedin::LoggedInApp { state  }
    }
}
