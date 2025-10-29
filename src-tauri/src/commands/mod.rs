pub mod info;

#[macro_export]
macro_rules! register_all_commands {
    () => {
        tauri::generate_handler![crate::commands::info::get_version]
    };
}
