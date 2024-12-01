use log::{debug, error};

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
                    .path()
                    .app_data_dir()
                    .unwrap_or_else(|e| {
                        error!("无法获取数据目录: {e}");
                        panic!("缺失关键文件");
                    })
                    .join("video.db");
                let db = create_db(&db_path);
                let db = std::sync::Arc::new(tokio::sync::Mutex::new(db));

                debug!("初始化观看记录处理线程");
                let (tx, rx) = tokio::sync::mpsc::channel(32);
                let db_update = db.clone();
                tokio::spawn(async move {
                    handle_record(rx, db_update).await;
                });
                app.manage(AppState {
                    tx: std::sync::Arc::new(tx),
                });
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            error!("tauri运行时错误: {e}");
            panic!("tauri运行时错误");
        });
}

struct AppState {
    tx: std::sync::Arc<tokio::sync::mpsc::Sender<(String, String)>>,
}
#[tauri::command]
async fn update_record(
    state: tauri::State<'_, AppState>,
    hash: String,
    recode: String,
) -> Result<(), String> {
    let tx = state.tx.clone();
    tx.send((hash, recode)).await.map_err(|e| {
        error!("发送观看记录信息失败: {e}");
        e.to_string()
    })?;
    Ok(())
}

/// 更新数据库内视频的观看记录，格式为时间戳，这个等前端写出来看看什么格式
async fn handle_record(
    mut rx: tokio::sync::mpsc::Receiver<(String, String)>,
    db: std::sync::Arc<tokio::sync::Mutex<rusqlite::Connection>>,
) {
    while let Some((hash, record)) = rx.recv().await {
        let db = db.lock().await;
        db.execute(
            "UPDATE Video SET record = ?2 WHERE hash = ?1;",
            rusqlite::params![hash, record],
        )
        .unwrap_or_else(|e| {
            error!("更新视频观看记录失败: {}", e);
            panic!("缺失关键文件");
        });
    }
}

struct Video {
    hash: String,
    path: std::path::PathBuf,
    record: String,
}

fn create_db<P: AsRef<std::path::Path>>(db_path: P) -> rusqlite::Connection {
    use rusqlite::Connection;
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
    )
    .unwrap_or_else(|err| {
        error!("无法处理数据库文件: {}", err);
        panic!("缺失关键文件");
    });
    conn
}
