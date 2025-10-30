use crate::state::GlobalState;
use notify_debouncer_full::{new_debouncer, notify::*, DebounceEventResult};
use std::fs;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

/// 设置配置文件监听器
///
/// # 生命周期
/// - 监听器会持续运行，直到 `GlobalState.debouncer` 被 drop
/// - 通过存储到全局状态，确保监听器在应用运行期间一直存活
/// - 应用退出时，Rust 会自动清理资源并停止监听
pub fn setup_config_watcher(app: AppHandle) -> notify::Result<()> {
    let state = GlobalState::get();

    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|e| notify::Error::generic(&format!("Failed to get app config dir: {}", e)))?;

    fs::create_dir_all(&config_dir).map_err(|e| notify::Error::io(e))?;

    let sig_file = config_dir.join("db.sig");

    if !sig_file.exists() {
        fs::write(&sig_file, "").map_err(|e| notify::Error::io(e))?;
    }

    // ✅ 保存监听文件路径到全局状态（初始化后只读）
    state.app_states.set_config_watch_path(sig_file.clone());

    tracing::info!("开始监听配置文件: {:?}", sig_file);

    let app_handle = app.clone();

    // 创建防抖监听器
    let mut debouncer = new_debouncer(
        Duration::from_millis(500),
        None,
        move |result: DebounceEventResult| match result {
            Ok(events) => {
                let has_valid_event = events
                    .iter()
                    .any(|event| matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)));

                if has_valid_event {
                    tracing::debug!("检测到配置文件变化，触发事件");
                    if let Err(e) = app_handle.emit("tauri//cfgchanged", ()) {
                        tracing::error!("发送配置变化事件失败: {}", e);
                    }
                }
            }
            Err(errors) => {
                for e in errors {
                    tracing::error!("文件监听错误: {:?}", e);
                }
            }
        },
    )?;

    debouncer.watch(&sig_file, RecursiveMode::NonRecursive)?;
    *state.debouncer.lock().unwrap() = Some(debouncer);

    tracing::info!("✅ 配置文件监听器已启动");

    Ok(())
}
