use clap::Parser;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use tempfile::NamedTempFile;

static GLOBAL_CONFIG_DIR: OnceLock<Result<PathBuf, String>> = OnceLock::new();

#[derive(clap::Parser, Debug, Clone)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = r#"webgen is an intelligent, AI-enhanced tool for bootstrapping modern web applications.
Leveraging advanced understanding of web technologies, design patterns, and best practices,
it generates full-stack web applications with semantic awareness, optimized architecture,
and production-ready configurations—tailored to your natural language description."#
)]
pub struct Args {
    /// Enable debug mode
    #[arg(short, long)]
    pub debug: bool,

    /// Configuration directory path
    #[arg(short, long, value_parser = parse_config_dir)]
    pub config: Option<PathBuf>,
}
/// 检查并解析配置目录
pub fn parse_config_dir(path_str: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path_str);

    // 1️⃣ 存在性和类型检查
    if !path.exists() {
        return Err(format!("The path '{}' does not exist.", path.display()));
    }
    if !path.is_dir() {
        return Err(format!("'{}' is not a directory.", path.display()));
    }

    // 2️⃣ 检查是否可写：尝试创建一个临时文件
    // 如果目录 readonly，这一步会返回错误。
    let writable = match NamedTempFile::new_in(&path) {
        Ok(temp_file) => {
            // 文件创建成功，立即删除以免污染目录
            let _ = fs::remove_file(temp_file.path());
            true
        }
        Err(_) => false,
    };

    if !writable {
        return Err(format!(
            "The directory '{}' is not writable (failed to create a temporary file).",
            path.display()
        ));
    }

    // 3️⃣ 返回 canonicalized 版本路径
    fs::canonicalize(&path)
        .map_err(|e| format!("Failed to canonicalize path '{}': {}", path.display(), e))
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }

    /// 返回配置目录（线程安全、惰性求值）
    pub fn config_dir(&self) -> Result<&'static PathBuf, &'static str> {
        GLOBAL_CONFIG_DIR
            .get_or_init(|| {
                let dir = match &self.config {
                    Some(config_path) => config_path.clone(),
                    None => {
                        let proj_dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
                            .ok_or_else(|| "Could not find project directories")?;
                        proj_dirs.config_dir().to_path_buf()
                    }
                };

                if !dir.exists() {
                    fs::create_dir_all(&dir)
                        .map_err(|e| format!("Failed to create config directory: {}", e))?;
                }

                if !dir.is_dir() {
                    return Err(format!("'{}' is not a directory", dir.display()));
                }

                let canon = fs::canonicalize(&dir).map_err(|e| {
                    tracing::debug!("canonicalize failed for {}", dir.display()); // <-- ①
                    Box::leak(
                        format!("Failed to canonicalize path '{}': {}", dir.display(), e)
                            .into_boxed_str(),
                    )
                })?;

                tracing::debug!(dir = %dir.display(), canon = %canon.display(), "canonicalize finished");

                // 4. 若用了用户指定路径，提醒
                if self.config.is_some() {
                    tracing::debug!("self.config is_some=true, going to print note");
                    eprintln!(
                        "Note: Using '{}' as the configuration directory to start.",
                        canon.display()
                    );
                }

                Ok(canon)
            })
            .as_ref()
            .map_err(|e| e.as_str())
    }
}
