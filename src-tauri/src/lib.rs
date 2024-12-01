use log::{error, debug};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;

        debug!("初始化数据库文件");
        use tauri::Manager;
        let db_path = app
          .path().app_data_dir().unwrap_or_else(|e| {
            error!("无法获取数据目录: {e}");
            panic!("缺失关键文件");
          }).join("video.db");
        create_db(&db_path);
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .unwrap_or_else(|e| {
        error!("tauri运行时错误: {e}");
        panic!("tauri运行时错误");
      });
}

struct Video {
    hash: String,
    path: std::path::PathBuf,
    record: String,
}

fn create_db<P: AsRef<std::path::Path>>(db_path: P) {
    use rusqlite::Connection;
    if !db_path.as_ref().exists() {
        // 如果数据库文件不存在，则创建
        let conn = Connection::open(db_path).unwrap_or_else(|err| {
            error!("无法创建数据库文件: {}", err);
            panic!("缺失关键文件");
        });
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Video (
                    hash TEXT PRIMARY KEY,
                    path TEXT NOT NULL,
                    record TEXT
                );",
            [],
        ).unwrap_or_else(|err| {
            error!("无法处理数据库文件: {}", err);
            panic!("缺失关键文件");
        });
    }
}