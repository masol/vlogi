mod commands;
mod state;
mod utils; // 引入 commands 模块

use state::GlobalState;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    utils::init();
    tauri::Builder::default()
        // ✅ 使用 utils::sql 中的初始化函数
        .plugin(utils::sql::init_sql_plugin().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
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

            tracing::info!("打开项目, {:?}!", state_from_global.args.project_dir());
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

                // ✅ 执行 SQL 后置初始化
                if let Err(e) = utils::sql::post_init_sql(&app_handle).await {
                    eprintln!("❌ SQL 后置初始化失败: {}", e);
                }
            });

            Ok(())
        })
        .invoke_handler(register_all_commands!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
