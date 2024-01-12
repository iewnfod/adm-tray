use std::{process::Command, net::TcpListener};

use futures::executor::block_on;

pub const BASE_URL: &str  = "http://127.0.0.1:63318";
pub const APP_BIND: &str = "127.0.0.1:63318";
pub const APP_NAME: &str = "aria-download-manager";

pub fn is_opened() -> bool {
	!TcpListener::bind(APP_BIND).is_ok()
}

pub fn open_app() {
	if is_opened() {
		println!("Has Opened");
		return;
	}
	let exe_path = std::env::current_exe().unwrap();
	let running_dir = std::env::current_dir().unwrap();
	let app_dir = exe_path.parent().unwrap();
	let mut possible_app_path = vec![];
	possible_app_path.push(running_dir.join(APP_NAME));
	possible_app_path.push(app_dir.join(APP_NAME));
	for p in possible_app_path {
		if p.exists() {
			let mut command = Command::new(p);
			command.spawn().unwrap();
			break;
		} else {
			println!("{} not found", p.display());
		}
	}
}

pub fn quit_app() {
	block_on(
		reqwest::Client::new().get(format!("{}/quit", BASE_URL)).send()
	).unwrap();
}
