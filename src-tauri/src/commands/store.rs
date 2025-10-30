use crate::state::GlobalState;

/// 触发数据库配置变更信号
///
/// 通知配置文件监听器数据库配置已发生变更，
/// 不执行实际的数据库操作，仅用于信号通知。
pub fn touch_db_config() -> Result<(), String> {
    GlobalState::get()
        .app_states
        .touch_config_file()
        .map_err(|e| format!("触发配置通知失败: {}", e))?;

    tracing::debug!("📢 数据库配置变更信号已触发");
    Ok(())
}

/// Tauri Command: 触发数据库配置变更信号
///
/// 用于前端在完成数据库操作后，通知其它进程重新加载配置(如果有)
#[tauri::command]
pub fn emit_cfg_changed() -> Result<(), String> {
    touch_db_config()
}
