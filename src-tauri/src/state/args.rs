use clap::Parser;
use std::fs;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[derive(clap::Parser, Debug, Clone)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = r#"vlogi.cc is an all-in-one multi-agent automation platform that integrates both usage and editing. It's built around the core model of "Input / Output / Process," transforming prompts into structured, orchestratable task chains—called PromptFlows. Users can either directly run shared PromptFlows created by others to accomplish specific tasks or use a visual, n8n-like interface to compose, modify, and build new ones. Through a recursive prompt-generation mechanism, vlogi.cc enables multiple agents to collaborate intelligently without relying on complex context engineering, while seamlessly leveraging each user's personal context to create adaptive, reconfigurable multi-agent workflows."#
)]
pub struct Args {
    /// Project directory path (optional)
    #[arg(value_parser = parse_writable_dir)]
    pub project: Option<PathBuf>,

    /// Enable debug mode
    #[arg(short, long)]
    pub debug: bool,
}

/// 验证目录存在性、类型和可写性
fn validate_directory(path: &PathBuf) -> Result<(), String> {
    // 1️⃣ 存在性和类型检查
    if !path.exists() {
        let msg = format!("The path '{}' does not exist.", path.display());
        tracing::error!("{}", msg);
        return Err(msg);
    }
    if !path.is_dir() {
        let msg = format!("'{}' is not a directory.", path.display());
        tracing::error!("{}", msg);
        return Err(msg);
    }

    // 2️⃣ 检查是否可写：尝试创建一个临时文件
    let writable = match NamedTempFile::new_in(path) {
        Ok(temp_file) => {
            // 文件创建成功，立即删除以免污染目录
            let _ = fs::remove_file(temp_file.path());
            true
        }
        Err(e) => {
            tracing::warn!(
                path = %path.display(),
                error = %e,
                "Failed to create temporary file for write test"
            );
            false
        }
    };

    if !writable {
        let msg = format!(
            "The directory '{}' is not writable (failed to create a temporary file).",
            path.display()
        );
        tracing::error!("{}", msg);
        return Err(msg);
    }

    tracing::info!(path = %path.display(), "Directory validated successfully");
    Ok(())
}

/// 检查并解析可写目录
pub fn parse_writable_dir(path_str: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path_str);
    tracing::info!(path = %path.display(), "Parsing writable directory");

    // 验证目录
    validate_directory(&path)?;

    // 返回 canonicalized 版本路径
    fs::canonicalize(&path).map_err(|e| {
        let msg = format!("Failed to canonicalize path '{}': {}", path.display(), e);
        tracing::error!("{}", msg);
        msg
    })
}

/// 规范化路径并记录日志
fn canonicalize_with_logging(path: &PathBuf, dir_type: &str) -> Result<PathBuf, String> {
    let canon = fs::canonicalize(path).map_err(|e| {
        let msg = format!("Failed to canonicalize path '{}': {}", path.display(), e);
        tracing::error!(
            path = %path.display(),
            dir_type = dir_type,
            error = %e,
            "Canonicalization failed"
        );
        msg
    })?;

    tracing::info!(
        original = %path.display(),
        canonical = %canon.display(),
        dir_type = dir_type,
        "Path canonicalized successfully"
    );

    Ok(canon)
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }

    /// 返回项目目录
    /// 可能为 None，表示没有指定项目目录
    pub fn project_dir(&self) -> Result<Option<PathBuf>, String> {
        match &self.project {
            Some(project_path) => {
                tracing::info!(path = %project_path.display(), "Processing project directory");

                // 项目路径已在 parse_writable_dir 中验证过
                // 这里只需要规范化
                let canon = canonicalize_with_logging(project_path, "project")?;

                eprintln!(
                    "Note: Using '{}' as the project directory.",
                    canon.display()
                );

                Ok(Some(canon))
            }
            None => {
                tracing::info!("No project directory specified");
                Ok(None)
            }
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}
