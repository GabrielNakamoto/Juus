use redb::{Database, ReadableTable, TableDefinition};
use redb::Error as RedbError;
use thiserror::Error;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, ResponseError,
	http::{
		StatusCode,
		header::ContentType
	}
};
use serde::{Deserialize, Serialize};
// use std::sync::Mutex;

const TABLE: TableDefinition<&str, [u8;32]> = TableDefinition::new("global-members");

#[derive(Error, Debug)]
enum AppError {
	#[error("Db op failed: {0}")]
	Database(#[from] RedbError),
	#[error("User not found: {0}")]
	NotFound(String)
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
	pubkey: [u8; 32]
}

// TODO: error handling, custom error type? -> remove all `.unwrap()` calls
async fn get_pubkey(stub: web::Json<MemberStub>, data: web::Data<State>) -> actix_web::Result<web::Json<[u8;32]>> {
	let db = &data.db;

	let txn = db.begin_read().map_err(AppError::Database)?;
	let table = txn.open_table(TABLE).unwrap();
	let value = table.get(stub.name.as_str()).unwrap().unwrap().value();

	Ok(web::Json(value))
}

async fn set_pubkey(member: web::Json<Member>, data: web::Data<State>) -> actix_web::Result<()> {
	let db = &data.db;
	let txn = db.begin_write().unwrap();
	{
		let mut table = txn.open_table(TABLE).unwrap();
		table.insert(member.name.as_str(), &member.pubkey).expect("Table should insert");
	}
	txn.commit().unwrap();
	Ok(())
}

#[actix_web::main]
async fn main() {
	let file = tempfile::NamedTempFile::new().unwrap();
	let state = web::Data::new(State {
		db: Database::create(file.path()).expect("Db create should work")
	});

	HttpServer::new(move || {
		App::new()
			.app_data(state.clone())
			.route("/get", web::get().to(get_pubkey))
			.route("/set", web::post().to(set_pubkey))
	})
	.bind(("127.0.0.1", 8081)).expect("Server should bind")
	.run()
	.await;
}
