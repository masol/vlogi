use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// 定义配置操作的枚举类型，方便未来扩展
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum ConfigAction {
    Focus { target: u32 },
    // 未来可以添加更多操作类型
    // Reload { module: String },
    // Update { key: String, value: String },
}

impl ConfigAction {
    pub fn focus(pid: u32) -> Self {
        ConfigAction::Focus { target: pid }
    }
}

/// 配置消息包装器 - 包含时间戳的完整消息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConfigMessage {
    /// 创建时间戳（Unix 时间戳，单位：秒）
    pub ctime: u64,
    /// 配置操作内容
    #[serde(flatten)]
    pub action: ConfigAction,
}

impl ConfigMessage {
    /// 创建新的配置消息，自动设置当前时间戳
    pub fn new(action: ConfigAction) -> Self {
        let ctime = Self::current_timestamp();
        Self { ctime, action }
    }

    /// 从已有的 action 和时间戳创建消息（用于测试或特殊场景）
    #[cfg(test)]
    pub fn with_timestamp(action: ConfigAction, ctime: u64) -> Self {
        Self { ctime, action }
    }

    /// 获取当前 Unix 时间戳（秒）
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// 检查消息是否在有效期内
    ///
    /// # Arguments
    /// * `max_age_secs` - 最大有效期（秒）
    ///
    /// # Returns
    /// * `true` - 消息仍然有效
    /// * `false` - 消息已过期
    pub fn is_valid(&self, max_age_secs: u64) -> bool {
        self.age_secs() <= max_age_secs
    }

    /// 获取消息年龄（秒）
    pub fn age_secs(&self) -> u64 {
        let now = Self::current_timestamp();
        now.saturating_sub(self.ctime)
    }

    /// 从 JSON 字符串反序列化
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// 序列化为 JSON 字符串
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// 序列化为格式化的 JSON 字符串（带缩进）
    // pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
    //     serde_json::to_string_pretty(self)
    // }

    /// 从文件读取配置消息
    ///
    /// # Arguments
    /// * `path` - 配置文件路径
    ///
    /// # Returns
    /// * `Ok(ConfigMessage)` - 成功读取并解析
    /// * `Err(io::Error)` - 文件读取或解析错误
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_json(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// 写入配置消息到文件
    ///
    /// # Arguments
    /// * `path` - 配置文件路径
    ///
    /// # Returns
    /// * `Ok(())` - 成功写入
    /// * `Err(io::Error)` - 文件写入或序列化错误
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let json = self
            .to_json()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }

    /// 读取文件并验证消息有效性
    ///
    /// # Arguments
    /// * `path` - 配置文件路径
    /// * `max_age_secs` - 消息最大有效期（秒），None 表示不检查有效期
    ///
    /// # Returns
    /// * `Ok(Some(ConfigMessage))` - 成功读取有效消息
    /// * `Ok(None)` - 消息已过期
    /// * `Err(_)` - 文件读取或解析错误
    pub fn read_and_validate<P: AsRef<Path>>(
        path: P,
        max_age_secs: Option<u64>,
    ) -> io::Result<Option<Self>> {
        let message = Self::read_from_file(path)?;

        if let Some(max_age) = max_age_secs {
            if !message.is_valid(max_age) {
                tracing::warn!(
                    "配置消息已过期: 年龄 {} 秒 > 最大 {} 秒",
                    message.age_secs(),
                    max_age
                );
                return Ok(None);
            }
        }

        Ok(Some(message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_message_creation() {
        let action = ConfigAction::focus(1234);
        let message = ConfigMessage::new(action.clone());

        assert_eq!(message.action, action);
        assert!(message.ctime > 0);
    }

    #[test]
    fn test_message_validation() {
        let action = ConfigAction::focus(1234);
        let message = ConfigMessage::new(action);

        // 新消息应该有效
        assert!(message.is_valid(60));

        // 测试过期消息
        let old_message =
            ConfigMessage::with_timestamp(ConfigAction::focus(5678), message.ctime - 100);
        assert!(!old_message.is_valid(60));
    }

    #[test]
    fn test_json_serialization() {
        let action = ConfigAction::focus(1234);
        let message = ConfigMessage::new(action);

        let json = message.to_json().unwrap();
        let decoded = ConfigMessage::from_json(&json).unwrap();

        assert_eq!(message, decoded);
    }

    #[test]
    fn test_age_calculation() {
        let action = ConfigAction::focus(1234);
        let message = ConfigMessage::new(action);

        thread::sleep(Duration::from_secs(1));

        let age = message.age_secs();
        assert!(age >= 1 && age <= 2);
    }
}
