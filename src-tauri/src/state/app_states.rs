use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::Arc;

/// 应用状态结构体 - 内部数据
#[derive(Debug)]
struct AppStatesInner {
    initialized: bool,
    /// 配置文件监听路径（初始化后只读）
    config_watch_path: Option<PathBuf>,
    // 未来可扩充的字段示例：
    // is_loading: bool,
    // error_count: u32,
    // last_update: Option<std::time::SystemTime>,
}

impl Default for AppStatesInner {
    fn default() -> Self {
        Self {
            initialized: false,
            config_watch_path: None,
        }
    }
}

/// 应用状态包装器 - 提供线程安全访问
#[derive(Debug, Clone)]
pub struct AppStates {
    inner: Arc<RwLock<AppStatesInner>>,
}

impl AppStates {
    /// 创建新的应用状态实例
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(AppStatesInner::default())),
        }
    }

    /// 读操作 - 获取 initialized 状态
    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.inner.read().initialized
    }

    /// 写操作 - 设置 initialized 状态
    #[inline]
    pub fn set_initialized(&self, value: bool) {
        self.inner.write().initialized = value;
    }

    /// 设置配置文件监听路径（仅初始化时调用一次）
    ///
    /// # 注意
    /// - 此方法应该只在 `setup_config_watcher` 中调用一次
    /// - 后续访问通过 `get_config_watch_path()` 获取只读引用
    pub fn set_config_watch_path(&self, path: PathBuf) {
        let mut guard = self.inner.write();
        guard.config_watch_path = Some(path);
    }

    /// 获取配置文件监听路径（只读访问）
    ///
    /// # Returns
    /// - `Some(PathBuf)` - 路径的克隆副本
    /// - `None` - 尚未初始化
    pub fn get_config_watch_path(&self) -> Option<PathBuf> {
        self.inner.read().config_watch_path.clone()
    }

    /// Touch 配置文件以触发文件监听事件（跨平台安全）
    ///
    /// # 实现原理
    /// - 使用 `filetime` crate 修改文件的访问时间和修改时间为当前时间
    /// - 这种方式不改变文件内容，但能确保触发文件系统事件
    /// - 跨平台兼容（Windows/Linux/macOS）
    ///
    /// # Errors
    /// - 返回 `std::io::Error` 如果文件不存在或无权限访问
    pub fn touch_config_file(&self) -> std::io::Result<()> {
        let path = self
            .get_config_watch_path()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "配置文件路径尚未初始化",
                )
            })?;

        // 使用 filetime crate 更新文件时间戳（跨平台方案）
        use filetime::{set_file_mtime, FileTime};
        let now = FileTime::now();
        set_file_mtime(&path, now)?;

        tracing::debug!("已触发配置文件 touch: {:?}", path);
        Ok(())
    }

    /*
    /// 原子性地检查并设置 initialized 状态
    /// 返回操作前的旧值
    pub fn test_and_set_initialized(&self, value: bool) -> bool {
        let mut guard = self.inner.write();
        let old_value = guard.initialized;
        guard.initialized = value;
        old_value
    }

    /// 批量读取（避免多次加锁）
    /// 返回状态的不可变快照
    pub fn snapshot(&self) -> AppStatesSnapshot {
        let guard = self.inner.read();
        AppStatesSnapshot {
            initialized: guard.initialized,
            config_watch_path: guard.config_watch_path.clone(),
        }
    }

    /// 批量更新（使用闭包模式，避免死锁）
    ///
    /// # Examples
    ///
    /// ```
    /// app_states.update(|state| {
    ///     state.initialized = true;
    /// });
    /// ```
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut AppStatesInner),
    {
        let mut guard = self.inner.write();
        f(&mut guard);
    }

    /// 条件更新 - 仅当条件满足时才执行更新
    /// 返回是否执行了更新
    ///
    /// # Examples
    ///
    /// ```
    /// let updated = app_states.update_if(
    ///     |state| !state.initialized,
    ///     |state| state.initialized = true,
    /// );
    /// ```
    pub fn update_if<P, F>(&self, predicate: P, f: F) -> bool
    where
        P: FnOnce(&AppStatesInner) -> bool,
        F: FnOnce(&mut AppStatesInner),
    {
        let mut guard = self.inner.write();
        if predicate(&guard) {
            f(&mut guard);
            true
        } else {
            false
        }
    }

    /// 读取并转换数据（避免克隆整个状态）
    ///
    /// # Examples
    ///
    /// ```
    /// let status = app_states.read_with(|state| {
    ///     format!("Initialized: {}", state.initialized)
    /// });
    /// ```
    pub fn read_with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&AppStatesInner) -> R,
    {
        let guard = self.inner.read();
        f(&guard)
    }
     */
}

impl Default for AppStates {
    fn default() -> Self {
        Self::new()
    }
}

// /// 状态快照（用于批量读取）
// #[derive(Debug, Clone)]
// pub struct AppStatesSnapshot {
//     pub initialized: bool,
//     pub config_watch_path: Option<PathBuf>,
// }