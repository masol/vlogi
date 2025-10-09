// src/state/mod.rs

pub mod app_handle;
pub mod args;

use self::app_handle::AppHandleState;
use std::sync::{Arc, OnceLock};

/// 全局状态结构体
#[derive(Clone, Debug)]
pub struct GlobalState {
    pub app_handle: AppHandleState,
    pub args: crate::state::args::Args,
    // pub db: DbPool,
    // pub config: Config,
    // pub cache: CacheLayer,
}

/// 全局单例访问点
pub static GLOBAL_STATE: OnceLock<Arc<GlobalState>> = OnceLock::new();

#[allow(dead_code)]
impl GlobalState {
    /// 创建全局状态（仅构造，不同步初始化）
    /// 所有字段应在此创建
    pub fn new() -> Self {
        Self {
            app_handle: AppHandleState::new(),
            args: args::Args::new(),
            // db: DbPool::new(),        // 如果构造是同步的
            // config: Config::default(),
        }
    }

    /// 异步初始化所有组件
    /// 由 setup 调用
    pub async fn init(&self, handle: &tauri::AppHandle) -> Result<(), String> {
        // 并行初始化（可扩展）
        tokio::try_join!(
            self.app_handle.init(handle.clone()),
            // self.db.migrate().map_err(|e| e.to_string()),
            // self.cache.warm_up().map_err(|e| e.to_string()),
        )?;

        Ok(())
    }

    /// 获取全局单例（用于非 State 上下文）
    pub fn instance() -> Option<&'static Arc<GlobalState>> {
        GLOBAL_STATE.get()
    }

    /// 检查全局状态是否已**逻辑初始化**（不仅仅是单例设置）
    /// 即：app_handle 是否已注入 AppHandle
    pub fn is_initialized(&self) -> bool {
        // 检查每个组件的初始化状态（同步快照）
        self.app_handle.is_initialized_sync()
        // && self.db.is_connected_sync()
        // && self.config.is_loaded()
    }
}
