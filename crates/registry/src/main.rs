use actix_web::middleware::Logger;
use redb::{Database, TableDefinition};
use redb::Error as RedbError;
use thiserror::Error;
use actix_web::{web, App, HttpServer, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use env_logger::Env;

type PubKey = [u8; 32];
const TABLE: TableDefinition<&str, PubKey> = TableDefinition::new("global-members");

#[derive(Error, Debug)]
enum AppError {
	#[error("Db op failed: {0}")]
	Database(#[from] RedbError),

	#[error("User not found: {0}")]
	NotFound(String),
}

impl ResponseError for AppError {
	fn error_response(&self) -> HttpResponse {
		match self {
			AppError::Database(e) => {
				HttpResponse::InternalServerError().json(serde_json::json!({
					"error" : "Internal server error",
					"details" : e.to_string()
				}))
			},
			AppError::NotFound(why) => HttpResponse::NotFound().json(serde_json::json!({
				"error" : "Not found",
				"details" : why
			}))
		}
	}
}

struct State {
	db: Database
}

#[derive(Deserialize)]
struct MemberStub {
	name: String
}

#[derive(Serialize, Deserialize)]
struct Member {
	name: String,
	#[serde(with = "serde_bytes")]
	pubkey: PubKey
}

async fn get_pubkey(stub: web::Json<MemberStub>, data: web::Data<State>) -> Result<web::Json<PubKey>, AppError> {
	let db = &data.db;
	let txn = db.begin_read().map_err(RedbError::from)?;
	let table = txn.open_table(TABLE).map_err(RedbError::from)?;

	let value = table.get(stub.name.as_str())
		.map_err(RedbError::from)?
		.map(|v| v.value());

	match value {
		Some(v) => Ok(web::Json(v)),
		None => Err(AppError::NotFound(stub.name.clone()))
	}
}

async fn set_pubkey(member: web::Json<Member>, data: web::Data<State>) -> Result<(), AppError> {
	let db = &data.db;
	let txn = db.begin_write().map_err(RedbError::from)?;
	{
		let mut table = txn.open_table(TABLE).map_err(RedbError::from)?;
		table.insert(member.name.as_str(), &member.pubkey).map_err(RedbError::from)?;
	}
	txn.commit().map_err(RedbError::from)?;
	Ok(())
}

#[actix_web::main]
async fn main() {
	env_logger::init_from_env(Env::default().default_filter_or("info"));

	let file = tempfile::NamedTempFile::new().unwrap();
	let state = web::Data::new(State {
		db: Database::create(file.path()).expect("Db create should work")
	});

	HttpServer::new(move || {
		App::new()
			.wrap(Logger::default())
			.app_data(state.clone())
			.route("/get", web::get().to(get_pubkey))
			.route("/set", web::post().to(set_pubkey))
	})
	.bind(("127.0.0.1", 8081)).expect("Server should bind")
	.run()
	.await;
}
