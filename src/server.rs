use std::{thread, time::Duration};

use actix_web::{HttpServer, App, web};
use serde::{Serialize, Deserialize};

use crate::api;

#[derive(Debug, Serialize, Deserialize)]
struct Info {
	download_id: usize,
	size: usize,
	webpage_url: String,
	download_url: String,
	resume_state: bool,
}

async fn index(data: web::Json<Info>) -> actix_web::Result<String> {
	if !api::is_opened() {
		api::open_app();
		thread::sleep(Duration::from_secs(1));
	}
	match reqwest::Client::new()
	.post(format!("{}/api", api::BASE_URL))
	.json(&data)
	.send().await {
		Ok(response) => Ok(response.text().await.unwrap()),
		Err(e) => Err(actix_web::error::ErrorInternalServerError(e))
	}
}

async fn state() -> actix_web::Result<String> {
	Ok("{\"status\": 0}".to_string())
}

pub async fn listen() {
	println!("Start Server");
	HttpServer::new(|| {
		App::new()
			.route("/api", web::post().to(index))
			.route("/state", web::get().to(state))
	})
	.bind("127.0.0.1:63319").unwrap()
	.run().await
	.unwrap();
}
