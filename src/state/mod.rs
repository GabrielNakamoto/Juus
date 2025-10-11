pub mod types;

#[cfg(feature = "p2p")]
use crate::p2p::node::JuNode;
use types::message::Message;

pub struct Client {
	#[cfg(feature = "p2p")]
	node: Option<JuNode>,
	messages: Vec<Message>
}

impl Client {
	pub fn new() -> Self {
		Self {
			#[cfg(feature = "p2p")]
			node: None,
			messages: Vec::new()
		}
	}

	pub fn request_message(&mut self, msg: Message) {
		self.messages.push(msg);
	}

	pub fn messages(&self) -> &Vec<Message> {
		&self.messages
	}
}

