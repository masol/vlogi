mod state;
mod utils;

use state::GlobalState;
use std::sync::Arc;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet(name: String) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    utils::init();
    tauri::Builder::default()
        // ✅ 1. 初始化 GLOBAL_STATE 并获取 Arc
        .manage(Arc::clone(GlobalState::instance().unwrap()))
        .setup(|app| {
            // 2. 验证一致性（可选，用于调试）
            let state_from_manage: tauri::State<Arc<GlobalState>> = app.state();
            let state_from_global = GlobalState::instance().expect("GLOBAL_STATE 应该已初始化");

            assert!(
                Arc::ptr_eq(&*state_from_manage, state_from_global),
                "State 不一致！"
            );
            tracing::debug!("✅ State 一致性验证通过");

            tracing::info!("配置目录, {:?}!", state_from_global.args.config_dir());
            // 3. 克隆 AppHandle 和 State
            let app_handle = app.handle().clone();
            let state_clone = Arc::clone(state_from_global);

            // 4. 在后台线程中初始化
            tauri::async_runtime::spawn(async move {
                if let Err(e) = state_clone.init(&app_handle).await {
                    eprintln!("❌ GlobalState 初始化失败: {}", e);
                    std::process::exit(1);
                }
                tracing::info!("✅ GlobalState 初始化成功");
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
