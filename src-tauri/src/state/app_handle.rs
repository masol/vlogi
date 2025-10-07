// src/state/app_handle.rs

use std::future::Future;
// use std::pin::Pin;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;

// 定义一个类型别名，简化签名
// type BoxFuture<R> = Pin<Box<dyn Future<Output = Result<R, String>> + Send>>;

#[derive(Clone, Debug)]
pub struct AppHandleState {
    inner: Arc<Mutex<Option<AppHandle>>>,
}

#[allow(dead_code)]
impl AppHandleState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::const_new(None)),
        }
    }

    // 修改：返回 Result
    pub async fn init(&self, handle: AppHandle) -> Result<(), String> {
        let mut guard = self.inner.lock().await;
        if guard.is_some() {
            return Err("AppHandle 已初始化".to_string());
        }
        *guard = Some(handle);
        Ok(())
    }

    /// 便捷方法：通过全局单例访问 AppHandle
    /// 支持同步和异步闭包（f 返回 impl Future<Output = Result<R, String>>）
    ///
    /// # 示例（异步）：
    /// ```rust
    /// GlobalState::with_app_handle(|handle| async move {
    ///     handle.emit_all("event", "hello").map_err(|e| e.to_string())?;
    ///     Ok::<_, String>("emitted")
    /// }).await;
    /// ```
    ///
    /// # 示例（同步）：
    /// ```rust
    /// GlobalState::with_app_handle(|handle| async move {
    ///     std::future::ready({
    ///         handle.emit_all("sync", "ok").map_err(|e| e.to_string())?;
    ///         Ok::<_, String>("done")
    ///     }).await
    /// }).await;
    /// ```
    pub async fn with_handle<R, F, Fut>(&self, f: F) -> Result<R, String>
    where
        R: Send + 'static,
        F: FnOnce(AppHandle) -> Fut,
        Fut: Future<Output = Result<R, String>> + Send,
    {
        // 1. 持锁获取 AppHandle
        let handle = {
            let guard = self.inner.lock().await;
            guard.clone()
        }; // 🔓 锁已释放

        // 2. 检查是否初始化
        let handle = match handle {
            Some(h) => h,
            None => return Err("AppHandle not initialized".to_string()),
        };

        // 3. 调用闭包并 await 结果
        f(handle).await
    }

    /// 同步检查是否已初始化
    /// 适用于快速判断（如日志、调试）
    pub fn is_initialized_sync(&self) -> bool {
        // 由于是 `Arc<Mutex<>>`，我们只能尝试非阻塞获取
        // 注意：可能因锁争用返回 false，但实际已初始化
        // 所以仅用于“尽力而为”的检查
        match self.inner.try_lock() {
            Ok(guard) => guard.is_some(),
            Err(_) => true, // 锁被占用，激进返回 true.
        }
    }

    pub async fn is_initialized(&self) -> bool {
        let guard = self.inner.lock().await;
        guard.is_some()
    }
}
