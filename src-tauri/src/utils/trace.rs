use tracing::Level;
use tracing_subscriber::FmtSubscriber;

/// 只在第一次调用时完成全局初始化，后续调用无开销
pub fn init() {
    // 运行时再判断：debug 模式看 DEBUG，release 只看 WARN+
    let max_level = if cfg!(debug_assertions) {
        Level::DEBUG
    } else {
        Level::WARN
    };

    // 最简文本格式，输出到 stderr
    let subscriber = FmtSubscriber::builder().with_max_level(max_level).finish();

    // 全局安装，多次调用会自动忽略
    tracing::subscriber::set_global_default(subscriber)
        .expect("tracing should only be initialized once");
}
