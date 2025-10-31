use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct AppHandleState {
    inner: Arc<Mutex<Option<AppHandle>>>,
}

impl AppHandleState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::const_new(None)),
        }
    }

    /// 初始化 AppHandle（只能调用一次）
    pub async fn init(&self, handle: AppHandle) -> Result<(), String> {
        let mut guard = self.inner.lock().await;
        if guard.is_some() {
            return Err("AppHandle 已初始化".to_string());
        }
        *guard = Some(handle);
        Ok(())
    }

    // /// 检查是否已初始化
    // pub async fn is_initialized(&self) -> bool {
    //     let guard = self.inner.lock().await;
    //     guard.is_some()
    // }
}

impl Default for AppHandleState {
    fn default() -> Self {
        Self::new()
    }
}
