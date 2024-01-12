use std::thread;

mod tray;
mod aria2c;
mod api;
mod server;

#[tokio::main]
async fn main() {
    // 启动 aria2 服务
    thread::spawn(|| {
        aria2c::startup();
    });
    // 启动 api 服务
    tokio::spawn(server::listen());
    // 启动应用
    let mut app = tray::DownloadManagerSystemTray::new();
    app.startup();
}
