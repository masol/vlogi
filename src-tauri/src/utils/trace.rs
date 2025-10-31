use crate::state::args::Args;
use crate::state::GlobalState;
use std::io;
use tracing::Level;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

/// 初始化全局 tracing 订阅器
///
/// 特性：
/// - 命令行参数优先级最高，其次是环境变量
/// - 控制台输出（stderr）+ 可选文件输出
/// - 根据日志级别自动调整详细程度
///
/// 优先级顺序：
/// 1. 命令行 --debug 标志
/// 2. 命令行 --log-level 参数
/// 3. RUST_LOG 环境变量
/// 4. 默认值（debug 构建: DEBUG, release 构建: WARN）
///
/// # Errors
/// 如果无法创建日志文件，会返回错误
///
/// # Panics
/// 如果 tracing 已经初始化过，会 panic
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let state = GlobalState::get();
    let args = &state.args;

    // 获取有效的日志级别（命令行参数已经处理了优先级）
    let log_level = args.effective_log_level();

    // 创建环境过滤器：命令行参数优先于环境变量
    let env_filter = if args.debug || args.log_level != Args::default_log_level() {
        // 用户明确指定了命令行参数，使用命令行参数
        EnvFilter::new(log_level.as_str())
    } else {
        // 没有指定命令行参数，尝试使用环境变量，否则使用默认值
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level.as_str()))
    };

    let registry = tracing_subscriber::registry().with(env_filter);

    // 判断是否需要文件和行号信息
    let show_file_info = log_level <= Level::DEBUG;

    if let Some(log_file_path) = &args.log_file {
        // 同时输出到控制台和文件
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)?;

        // 控制台输出层
        let console_layer = fmt::layer()
            .with_target(true)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(show_file_info)
            .with_line_number(show_file_info)
            .with_span_events(if log_level <= Level::DEBUG {
                FmtSpan::CLOSE
            } else {
                FmtSpan::NONE
            })
            .compact()
            .with_ansi(true)
            .with_writer(io::stderr);

        // 文件输出层
        let file_layer = fmt::layer()
            .with_target(true)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(show_file_info)
            .with_line_number(show_file_info)
            .with_span_events(if log_level <= Level::DEBUG {
                FmtSpan::CLOSE
            } else {
                FmtSpan::NONE
            })
            .with_ansi(false)
            .with_writer(file);

        registry.with(console_layer).with(file_layer).init();
    } else {
        // 仅控制台输出
        let console_layer = fmt::layer()
            .with_target(true)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(show_file_info)
            .with_line_number(show_file_info)
            .with_span_events(if log_level <= Level::DEBUG {
                FmtSpan::CLOSE
            } else {
                FmtSpan::NONE
            })
            .compact()
            .with_ansi(true)
            .with_writer(io::stderr);

        registry.with(console_layer).init();
    }

    // 记录初始化信息
    tracing::info!(
        log_level = ?log_level,
        log_file = ?args.log_file,
        "Tracing initialized"
    );

    Ok(())
}