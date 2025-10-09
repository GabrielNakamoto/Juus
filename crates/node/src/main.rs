use iroh::{Endpoint, NodeAddr, SecretKey, RelayMode, RelayUrl,};
use std::collections::HashMap;
use n0_snafu::ResultExt;
use std::env;
use rand::rngs::OsRng;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

const JUUS_P2P_V0_ALPN: &[u8] = b"juus-p2p-v0";

#[derive(Debug, Clone)]
enum PkarrRelay {
	Disabled,
	Iroh
}

struct JuPeer {
	username: String,
	pubkey: iroh::PublicKey
}

struct JuNode {
	secret_key: iroh::SecretKey,
	endpoint: iroh::Endpoint,
}

#[derive(Serialize, Deserialize)]
struct MemberStub {
	name: String
}

#[derive(Serialize, Deserialize)]
struct Member {
	name: String,
	#[serde(with = "serde_bytes")]
	pubkey: [u8;32]
}

impl JuNode {
	fn build_discovery(relay: PkarrRelay) -> iroh::discovery::pkarr::dht::Builder {
		use iroh::discovery::pkarr::dht::DhtDiscovery;

		let builder = DhtDiscovery::builder();
		match relay {
			PkarrRelay::Disabled => builder,
			PkarrRelay::Iroh => builder.n0_dns_pkarr_relay()
		}
	}

	fn get_peer_publickey(username: String) -> iroh::PublicKey {
		const REGISTRY_EP: &str = "http://localhost:8081/get";

		println!("Fetching {}'s public key from: {}", username, REGISTRY_EP);
		let client = reqwest::blocking::Client::new();
		let body = MemberStub { name: username };
		let res = client.get(REGISTRY_EP)
			.json(&body)
			.send()
			.unwrap();

		// match res statement
		let bytes = res.json::<[u8;32]>().unwrap();
		let pubkey = iroh::PublicKey::from_bytes(&bytes).unwrap();
		println!("Got: {}", pubkey);

		pubkey
	}

	fn publish_publickey(secret_key: &iroh::SecretKey, username: String) {
		const REGISTRY_EP: &str = "http://localhost:8081/set";

		println!("Publish public key to: {}", REGISTRY_EP);
		let binding = secret_key.public();
		let pubkey = binding.as_bytes();
		let body = Member { name: username, pubkey: pubkey.clone() };
		let client = reqwest::blocking::Client::new();
		let res = client.post(REGISTRY_EP)
			.json(&body)
			.send()
			.unwrap();

		println!("Response status: {}", res.status());
		println!("Response json: {}", res.text().unwrap());
	}

	fn generate_secretkey(path: &Path, display: String) -> std::io::Result<iroh::SecretKey> {
		println!("Generating new secret key...");
		let secret_key = SecretKey::generate(&mut OsRng);
		let mut file = match File::create(&path) {
			Err(why) => panic!("couldn't create {}: {}", display, why),
			Ok(file) => file
		};
		println!("Serializing secret key...");
		file.write_all(&secret_key.to_bytes())?;

		Self::publish_publickey(&secret_key, String::from("Gabriel"));
		Self::get_peer_publickey(String::from("Test")); // TESTING

		return Ok(secret_key)
	}

	fn deserialize_or_gen_secretkey() -> std::io::Result<iroh::SecretKey> {
		let path = Path::new("secret");
		let display = path.display();

		if ! path.exists() {
			return Self::generate_secretkey(&path, display.to_string());
		}

		let mut secret_file = match File::open(&path) {
			Err(why) => panic!("couldn't open {}: {}", display, why),
			Ok(file) => file
		};

		println!("Found secret key file...");
		println!("Deserializing secret key...");

		let mut buffer = [0; 32];
		let n = secret_file.read(&mut buffer[..])?;

		if n != 32 {
			println!("Corrupted or invalid secret key");
			return Self::generate_secretkey(&path, display.to_string());
		}

		Ok(iroh::SecretKey::from_bytes(&buffer))
	}

	async fn new(relay: PkarrRelay) -> n0_snafu::Result<Self> {
		let secret_key = tokio::task::spawn_blocking(move || {
			Self::deserialize_or_gen_secretkey().unwrap()
		}).await.unwrap();

		println!("Your public key is: {}", secret_key.public());

		let endpoint = Endpoint::builder()
			.alpns(vec![JUUS_P2P_V0_ALPN.to_vec()])
			.secret_key(secret_key.clone())
			.discovery(Self::build_discovery(relay))
			.bind()
			.await?;

		Ok(Self {
			secret_key,
			endpoint
		})
	}

	async fn create_connection(&self, remote_id: iroh::NodeId) -> n0_snafu::Result<()> {
		let connection = self.endpoint.connect(remote_id, JUUS_P2P_V0_ALPN).await?;
		let (mut writer, mut reader) = connection.open_bi().await.e()?;
		Ok(())
	}

	async fn handle_connections(&self) {
		while let Some(incoming) = self.endpoint.accept().await {
			let connecting = match incoming.accept() {
				Ok(connecting) => connecting,
				Err(err) => {
					println!("incoming connection failed: {err:#}");
					// we can carry on in these cases:
					// this can be caused by retransmitted datagrams
					continue;
				}
			};
			tokio::spawn(async move {
				let connection = connecting.await.e()?;
				let remote = connection.remote_node_id()?;

				let (mut writer, mut reader) = connection.accept_bi().await.e()?;
				Ok::<_, n0_snafu::Error>(())
			});
		}
	}
}

#[tokio::main]
async fn main() {
	let node = JuNode::new(PkarrRelay::Disabled).await.unwrap();
	// node.handle_connections().await;
}
