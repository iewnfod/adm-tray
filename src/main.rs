use std::thread;

mod tray;
mod aria2c;
mod api;

#[tokio::main]
async fn main() {
    // 启动 aria2 服务
    thread::spawn(|| {
        aria2c::startup();
    });
    let mut app = tray::DownloadManagerSystemTray::new();
    app.startup();
}
