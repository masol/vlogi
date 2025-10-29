use std::future::Future;
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

    /// 便捷方法：安全地访问 AppHandle
    ///
    /// # 示例（异步）：
    /// ```rust
    /// state.with_handle(|handle| async move {
    ///     handle.emit_all("event", "hello").map_err(|e| e.to_string())?;
    ///     Ok::<_, String>("emitted")
    /// }).await?;
    /// ```
    #[allow(dead_code)]
    pub async fn with_handle<R, F, Fut>(&self, f: F) -> Result<R, String>
    where
        R: Send + 'static,
        F: FnOnce(AppHandle) -> Fut,
        Fut: Future<Output = Result<R, String>> + Send,
    {
        // 持锁获取 AppHandle 的克隆
        let handle = {
            let guard = self.inner.lock().await;
            guard.clone()
        }; // 🔓 锁已释放

        // 检查是否已初始化
        let handle = handle.ok_or("AppHandle 未初始化")?;

        // 调用用户闭包
        f(handle).await
    }
}

impl Default for AppHandleState {
    fn default() -> Self {
        Self::new()
    }
}
