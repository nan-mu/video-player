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
                app.manage(UpdateState {
                    tx: std::sync::Arc::new(tx),
                });
                app.manage(GetState { db });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![update_record, get_list])
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            error!("tauri运行时错误: {e}");
            panic!("tauri运行时错误");
        });
}

struct UpdateState {
    tx: std::sync::Arc<tokio::sync::mpsc::Sender<(String, String, String)>>,
}
#[tauri::command]
async fn update_record(
    state: tauri::State<'_, UpdateState>,
    hash: String,
    path: String,
    recode: String,
) -> Result<(), String> {
    let tx = state.tx.clone();
    tx.send((hash, path, recode)).await.map_err(|e| {
        error!("发送观看记录信息失败: {e}");
        e.to_string()
    })?;
    Ok(())
}

/// 更新数据库内视频的观看记录，格式为时间戳，这个等前端写出来看看什么格式
async fn handle_record(
    mut rx: tokio::sync::mpsc::Receiver<(String, String, String)>,
    db: std::sync::Arc<tokio::sync::Mutex<rusqlite::Connection>>,
) {
    while let Some((hash, path, record)) = rx.recv().await {
        if hash.is_empty() {
            let hash = calculate_file_hash(&path).await.unwrap_or_else(|e| {
                error!("计算文件哈希失败: {}", e);
                "NOHASH".to_string()
            });
            let db = db.lock().await;
            db.execute(
                "INSERT INTO Video (hash, path, record) VALUES (?1, ?2, ?3) \
                ON CONFLICT(hash) DO UPDATE SET path = excluded.path, record = excluded.record;",
                rusqlite::params![hash, path, record],
            )
            .unwrap_or_else(|e| {
                error!("插入新视频观看记录失败: {}", e);
                0
            });
        } else {
            let db = db.lock().await;
            db.execute(
                "UPDATE Video SET record = ?2 WHERE hash = ?1;",
                rusqlite::params![hash, record],
            )
            .unwrap_or_else(|e| {
                error!("更新视频观看记录失败: {}", e);
                0
            });
        }
    }
}

async fn calculate_file_hash(file_path: &str) -> std::io::Result<String> {
    use sha2::Digest;
    use std::io::Read;

    let mut file = std::fs::File::open(file_path)?;
    let mut hasher = sha2::Sha256::new();
    // 读取文件的前 64 KB
    const HASH_SIZE: usize = 64 * 1024;
    let mut buffer = vec![0u8; HASH_SIZE];
    let bytes_read = file.read(&mut buffer)?;
    hasher.update(&buffer[..bytes_read]);

    // 获取文件的大小
    let file_size = file.metadata()?.len();

    // 如果文件大于 64 KB，读取最后 64 KB
    if file_size > HASH_SIZE as u64 {
        let mut buffer = vec![0u8; HASH_SIZE];
        std::io::Seek::seek(&mut file, std::io::SeekFrom::End(-(HASH_SIZE as i64)))?;
        let bytes_read = file.read(&mut buffer)?;
        hasher.update(&buffer[..bytes_read]);
    }

    // 完成哈希计算
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
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

#[derive(serde::Serialize)]
struct Video {
    hash: String,
    path: String,
    record: String,
    size: String,
}

struct GetState {
    db: std::sync::Arc<tokio::sync::Mutex<rusqlite::Connection>>,
}
#[tauri::command]
async fn get_list(state: tauri::State<'_, GetState>, offset: usize) -> Result<Vec<Video>, String> {
    let db = state.db.lock().await;
    let mut stmt = db
        .prepare(
            "SELECT hash, path, record FROM Video ORDER BY modified_time DESC LIMIT 5 OFFSET ?1",
        )
        .map_err(|e| e.to_string())?;

    let video_iter = stmt
        .query_map([offset as i64], |row| {
            let hash = row.get(0)?;
            let path = row.get(1)?;
            let record = row.get(2)?;
            let size = std::fs::metadata(&path)
                .map(|metadata| {
                    let size = metadata.len();
                    if size < 1024 {
                        format!("{} B", size)
                    } else if size < 1024 * 1024 {
                        format!("{:.2} KB", size as f64 / 1024.0)
                    } else if size < 1024 * 1024 * 1024 {
                        format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
                    } else if size < 1024 * 1024 * 1024 * 1024 {
                        format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
                    } else {
                        format!(
                            "{:.2} TB",
                            size as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0)
                        )
                    }
                })
                .unwrap_or_else(|_| "UNKNOWN".to_string());

            Ok(Video {
                hash,
                path,
                size,
                record,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut videos = Vec::new();
    for video in video_iter {
        videos.push(video.map_err(|e| e.to_string())?);
    }

    Ok(videos)
}
