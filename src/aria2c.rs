use std::process::{Command, Child};

static mut ARIA2C_PROCESS: Option<Child> = None;

pub fn startup() {
	let mut cmd = Command::new("aria2c");
	cmd.arg("--enable-rpc");
	let process = match cmd.spawn() {
		Ok(process) => process,
		Err(err) => {
			panic!("{}", err)
		},
	};
	unsafe { ARIA2C_PROCESS = Some(process) };
}

pub fn stop() {
	println!("Stop Aria2c");
	unsafe {
		if let Some(process) = &mut ARIA2C_PROCESS {
			process.kill().unwrap();
		}
	}
}
