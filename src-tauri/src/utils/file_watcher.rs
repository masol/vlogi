use crate::state::GlobalState;
use crate::utils::message::ConfigAction;
use notify::{event::ModifyKind, EventKind};
use notify_debouncer_full::{new_debouncer, notify::*, DebounceEventResult};
use parking_lot::Mutex;
use std::fs;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tauri::{AppHandle, Emitter, Manager};

/// 文件元数据缓存
#[derive(Debug, Clone, Copy, PartialEq)]
struct FileMetadata {
    size: u64,
    modified: SystemTime,
}

impl FileMetadata {
    fn from_path(path: &std::path::Path) -> std::io::Result<Self> {
        let metadata = fs::metadata(path)?;
        Ok(Self {
            size: metadata.len(),
            modified: metadata.modified()?,
        })
    }
}

/// 设置配置文件监听器
///
/// # 生命周期
/// - 监听器会持续运行，直到 `GlobalState.debouncer` 被 drop
/// - 通过存储到全局状态,确保监听器在应用运行期间一直存活
/// - 应用退出时,Rust 会自动清理资源并停止监听
///
/// # 去重机制
/// - 缓存文件大小和修改时间--读文件内容会触发access time change,导致循环.并且部分平台无法区分内容变动和meta变动．
/// - 只有这两个属性的任意一个或全部发生变化时，才触发处理
/// - 避免重复处理相同的文件变更事件
/// - @todo: 使用两个文件来处理，一个是信号，一个是内容.
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

    state.app_states.set_config_watch_path(sig_file.clone());
    tracing::info!("开始监听配置文件: {:?}", sig_file);

    // ✅ 初始化文件元数据缓存
    let last_metadata = Arc::new(Mutex::new(FileMetadata::from_path(&sig_file).ok()));

    let app_handle = app.clone();
    let sig_file_clone = sig_file.clone();

    let mut debouncer = new_debouncer(
        Duration::from_millis(10), // ✅ 0.01秒防抖
        None,                       // ✅ 默认缓存就够了
        move |result: DebounceEventResult| match result {
            Ok(events) => {
                // ✅ 只处理数据修改事件
                let valid = events
                    .iter()
                    .any(|e| matches!(e.event.kind, EventKind::Modify(ModifyKind::Data(_))));

                if !valid {
                    return;
                }

                // ✅ 读取当前文件元数据
                let current_metadata = match FileMetadata::from_path(&sig_file_clone) {
                    Ok(meta) => meta,
                    Err(e) => {
                        tracing::warn!("无法读取文件元数据: {}", e);
                        return;
                    }
                };

                // ✅ 比较元数据是否变化
                let mut last = last_metadata.lock();
                let has_changed = match *last {
                    Some(ref prev) => {
                        prev.size != current_metadata.size
                            || prev.modified != current_metadata.modified
                    }
                    None => true, // 首次读取，视为变化
                };

                if !has_changed {
                    tracing::debug!("文件元数据未变化，跳过处理");
                    return;
                }

                // ✅ 更新缓存
                *last = Some(current_metadata);
                drop(last); // 提前释放锁

                tracing::debug!(
                    "检测到配置文件变化 [size: {}, modified: {:?}]",
                    current_metadata.size,
                    current_metadata.modified
                );

                match handle_config_message(&app_handle) {
                    Ok(true) => tracing::info!("✅ 配置消息处理成功"),
                    Ok(false) | Err(_) => emit_db_changed(&app_handle),
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
    *state.config_watcher.lock().unwrap() = Some(debouncer);

    Ok(())
}

/// 发送数据库变化事件
///
/// # 说明
/// 提取为独立函数以减少代码重复，统一错误处理
#[inline]
fn emit_db_changed(app_handle: &AppHandle) {
    tracing::debug!("触发数据库更新事件");
    if let Err(e) = app_handle.emit("tauri//cfgchanged", ()) {
        tracing::error!("发送配置变化事件失败: {}", e);
    }
}

/// 处理配置消息
///
/// # 返回值
/// * `Ok(true)` - 成功处理了有效的配置消息
/// * `Ok(false)` - 没有有效的配置消息（过期或不存在）
/// * `Err(_)` - 读取或处理失败
fn handle_config_message(app_handle: &AppHandle) -> std::io::Result<bool> {
    let state = GlobalState::get();

    // 读取并验证配置消息（10秒有效期）
    match state.app_states.read_config_message(Some(10))? {
        Some(message) => {
            tracing::info!(
                "收到有效配置消息: action={:?}, age={}s",
                message.action,
                message.age_secs()
            );

            // 根据 action 类型分发处理
            dispatch_action(app_handle, message.action)?;
            Ok(true)
        }
        None => {
            // 消息过期或无效
            tracing::debug!("配置消息已过期或无效");
            Ok(false)
        }
    }
}

/// 分发并执行配置操作
///
/// # 扩展指南
/// 添加新的 action 类型时：
/// 1. 在 `ConfigAction` 枚举中添加新变体
/// 2. 在此函数的 match 分支中添加对应处理逻辑
/// 3. 如果需要，创建专门的处理函数（如 `handle_focus_action`）
fn dispatch_action(app_handle: &AppHandle, action: ConfigAction) -> std::io::Result<()> {
    match action {
        ConfigAction::Focus { target } => handle_focus_action(app_handle, target),
        // 未来可以在此添加更多 action 处理
        // ConfigAction::Reload { module } => handle_reload_action(app_handle, &module),
        // ConfigAction::Update { key, value } => handle_update_action(app_handle, &key, &value),
    }
}

/// 处理 Focus 操作 - 激活指定 PID 的 Tauri 窗口
///
/// # 参数
/// * `app_handle` - Tauri 应用句柄
/// * `target_pid` - 目标进程 PID
///
/// # 逻辑
/// 1. 获取当前进程 PID
/// 2. 验证目标 PID 是否与当前 PID 匹配
/// 3. **只有 PID 匹配时才执行激活操作**
/// 4. PID 不匹配时直接返回（可能是其他实例的请求）
fn handle_focus_action(app_handle: &AppHandle, target_pid: u32) -> std::io::Result<()> {
    let current_pid = std::process::id();

    // ⚠️ 关键检查：只处理发给自己的请求
    if target_pid != current_pid {
        tracing::debug!(
            "目标 PID ({}) 不是当前进程 ({}), 忽略此请求",
            target_pid,
            current_pid
        );
        return Ok(());
    }

    tracing::info!("✅ PID 匹配，开始激活窗口 (PID: {})", current_pid);

    // 获取主窗口并激活
    let window = app_handle.get_webview_window("main").ok_or_else(|| {
        let err_msg = "未找到主窗口 'main'";
        tracing::error!("{}", err_msg);
        std::io::Error::new(std::io::ErrorKind::NotFound, err_msg)
    })?;

    // 取消最小化（如果窗口被最小化）
    if let Err(e) = window.unminimize() {
        tracing::warn!("取消最小化失败（窗口可能未最小化）: {}", e);
    }

    // 显示窗口
    window.show().map_err(|e| {
        tracing::error!("显示窗口失败: {}", e);
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to show window: {}", e),
        )
    })?;

    // 将窗口置于前台
    window.set_focus().map_err(|e| {
        tracing::error!("设置窗口焦点失败: {}", e);
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to focus window: {}", e),
        )
    })?;

    tracing::info!("✅ 成功激活主窗口 (PID: {})", current_pid);
    Ok(())
}
