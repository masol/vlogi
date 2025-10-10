use std::path::PathBuf;

/// 初始化 WebView 语言和用户数据目录，必须在 tauri::Builder 构建前调用
pub fn init(config_dir: &PathBuf) {
    // 1. 确保子目录存在
    let webview_data = config_dir.join("webview_data");
    let _ = std::fs::create_dir_all(&webview_data);

    // 2. 跟随系统语言
    let locale = sys_locale::get_locale().unwrap_or_else(|| "en-US".into());

    tracing::info!("系统语言, {:?}!", locale);


    // 3. 各平台注入
    #[cfg(target_os = "macos")]
    std::env::set_var("WEBKIT_USER_DATA_DIRECTORY", &webview_data);

    #[cfg(all(not(target_os = "macos"), target_family = "unix"))]
    {
        std::env::set_var("LANGUAGE", &locale);
        // std::env::set_var("LC_ALL", &locale);
        std::env::set_var("WEBKIT_USER_DATA_DIRECTORY", &webview_data);
    }

    #[cfg(target_os = "windows")]
    {
        std::env::set_var(
            "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
            format!("--lang={}", locale),
        );
        std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &webview_data);
    }
}
