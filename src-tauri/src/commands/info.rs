// src/commands/info.rs
use crate::state::GlobalState;
use serde::Serialize;

#[derive(Serialize)]
pub struct PkgInfoResponse {
    pub version: String,
    pub initialized: bool,
    pub locale: String,
    pub pid: u32,
}

#[tauri::command]
pub async fn get_soft_info() -> Result<PkgInfoResponse, String> {
    let state = GlobalState::get();

    Ok(PkgInfoResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        initialized: state.app_states.is_initialized(),
        locale: sys_locale::get_locale().unwrap_or_else(|| "en".into()),
        pid: std::process::id(),
    })
}


// use sysinfo::{System, SystemExt};

// /// 检查指定 PID 对应的进程是否仍然有效（存在）
// ///
// /// # 参数
// /// * `pid` - 要检查的进程 ID
// ///
// /// # 返回值
// /// * `true` - 进程存在且有效
// /// * `false` - 进程不存在或已退出
// pub fn is_pid_alive(pid: u32) -> bool {
//     let mut sys = System::new();
//     // 刷新部分信息，避免过慢
//     sys.refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, true);
//     sys.process(pid as i32).is_some()
// }

// // 示例使用
// fn main() {
//     let pid = 12345;
//     if is_pid_alive(pid) {
//         println!("✅ PID {} 有效", pid);
//     } else {
//         println!("❌ PID {} 无效或已退出", pid);
//     }
// }