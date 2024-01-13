use std::{process::{Command, Child}, path::Path};

const ARIA2C_PATH: &str = "/opt/homebrew/bin/aria2c";

static mut ARIA2C_PROCESS: Option<Child> = None;

pub fn startup() {
	if Path::new(ARIA2C_PATH).exists() {
		let mut cmd = Command::new(ARIA2C_PATH);
		cmd.arg("--enable-rpc");
		let process = match cmd.spawn() {
			Ok(process) => process,
			Err(err) => {
				panic!("{}", err)
			},
		};
		unsafe { ARIA2C_PROCESS = Some(process) };
	} else {
		println!("Cannot find aria2c");
	}
}

pub fn stop() {
	println!("Stop Aria2c");
	unsafe {
		if let Some(process) = &mut ARIA2C_PROCESS {
			process.kill().unwrap();
		}
	}
}
