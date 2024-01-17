use std::{process::Command, net::TcpListener, path::PathBuf};

use futures::executor::block_on;

pub const BASE_URL: &str  = "http://127.0.0.1:63318";
pub const APP_BIND: &str = "127.0.0.1:63318";
pub const APP_NAME: &str = "aria-download-manager";

static mut APP_PID: u32 = 0;

pub fn is_opened() -> bool {
	!TcpListener::bind(APP_BIND).is_ok()
}

pub fn get_env() -> PathBuf {
	let exe_path = std::env::current_exe().unwrap();
	exe_path.parent().unwrap().to_path_buf()
}

pub fn open_app() {
	if is_opened() {
		println!("Try to Focus");
		block_on(
			reqwest::get(format!("{}/focus", BASE_URL))
		).unwrap();
		return;
	}
	let app_path = get_env().join(APP_NAME);
	if app_path.exists() {
		let mut command = Command::new(app_path);
		let child = command.spawn().unwrap();
		println!("Start app with pid {}", child.id());
		unsafe {
			APP_PID = child.id();
		}
	} else {
		println!("{} not found", app_path.display());
	}
}

pub fn quit_app() {
	if unsafe { APP_PID != 0 } {
		let mut command = Command::new("kill");
		command.arg("-9").arg(unsafe {APP_PID}.to_string());
		let _ = command.spawn().unwrap();
	}
}
