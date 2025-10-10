mod trace;

/// 统一入口：以后 utils 里再有别的初始化，都放这里
pub fn init() {
    trace::init(); // 初始化 tracing
}
