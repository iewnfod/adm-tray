use std::process;

use tray_item::{TrayItem, IconSource};

use crate::{aria2c, api::{quit_app, open_app}};

pub struct DownloadManagerSystemTray {
	app: TrayItem,
}

impl DownloadManagerSystemTray {
	pub fn new() -> Self {
		let app = TrayItem::new("ADM", IconSource::Resource("")).unwrap();
		Self {
			app,
		}
	}

	pub fn startup(&mut self) {
		self.app.add_menu_item("Open Window", || {
			println!("Open");
			open_app();
		}).unwrap();

		self.app.add_menu_item("Quit", || {
			println!("Quit");
			quit_app();
			aria2c::stop();
			process::exit(0);
		}).unwrap();

		let inner = self.app.inner_mut();
		inner.display();
	}
}
