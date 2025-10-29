use std::sync::Arc;

use crate::state::{self, GlobalState};

pub mod sql;
mod trace;
// mod webview;

/// 统一入口：以后 utils 里再有别的初始化，都放这里
pub fn init() {
    trace::init(); // 初始化 tracing

    let global_state = Arc::new(GlobalState::new());
    state::GLOBAL_STATE
        .set(Arc::clone(&global_state))
        .expect("GLOBAL_STATE 已被初始化");

    // webview::init(&global_state.args.config_dir().unwrap());
}
