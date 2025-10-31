pub mod store;
pub mod info;

#[macro_export]
macro_rules! register_all_commands {
    () => {
        tauri::generate_handler![
            crate::commands::info::get_soft_info,
            crate::commands::store::emit_cfg_changed,
            crate::commands::store::emit_focus,
        ]
    };
}
