// use std::sync::Arc;

// use crate::state::{self, GlobalState};

pub mod sql;
pub mod file_watcher;
mod trace;
// mod webview;

/// 统一入口：以后 utils 里再有别的初始化，都放这里
pub fn init() {
    trace::init(); // 初始化 tracing

    // webview::init(&global_state.args.config_dir().unwrap());
}
