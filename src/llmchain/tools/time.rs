use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeTool;

impl TimeTool {
    pub fn new() -> Self {
        TimeTool
    }

    /// 获取当前本地时间
    pub fn get_local_time(&self, format: &str) -> String {
        let now = Local::now();
        now.format(format).to_string()
    }

    /// 获取当前UTC时间
    pub fn get_utc_time(&self, format: &str) -> String {
        let now = Utc::now();
        now.format(format).to_string()
    }

    /// 获取Unix时间戳
    pub fn get_unix_timestamp(&self) -> i64 {
        Utc::now().timestamp()
    }

    /// 获取RFC3339格式时间
    pub fn get_rfc3339(&self) -> String {
        let now: DateTime<Utc> = Utc::now();
        now.to_rfc3339()
    }

    /// 获取指定时区时间
    pub fn get_time_in_timezone(&self, timezone: &str, format: &str) -> Result<String, String> {
        match chrono_tz::Tz::from_str(timezone) {
            Ok(tz) => {
                let now = Utc::now().with_timezone(&tz);
                Ok(now.format(format).to_string())
            }
            Err(_) => Err(format!("Invalid timezone: {}", timezone)),
        }
    }
}

impl fmt::Display for TimeTool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TimeTool")
    }
}
