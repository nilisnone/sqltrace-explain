use std::collections::HashMap;
use std::env;
use std::path::Path;

#[derive(serde::Deserialize, Clone, Debug)]
#[cfg(not(test))]
pub struct Settings {
    pub env: String,
    pub scan_dir: String,
    pub position_file: String,
    pub explain_interval: u32,
    pub database: HashMap<String, Database>,
}

#[derive(serde::Deserialize, Clone, Debug)]
#[cfg(not(test))]
pub struct Database {
    pub driver: String,
    pub dsn: String,
}

pub fn set_home_dir() {
    match env::var("SOI_HOME") {
        Ok(val) => {
            let home = Path::new(&val); // 判断home是否为合法存在的路径
            if !home.is_dir() {
                panic!("SOI_HOME: {val} 不是存在的目录，请重新配置");
            }

            match std::fs::read_dir(home) {
                Ok(_) => {
                    env::set_current_dir(&val).expect(format!("无法设置目录 {} 为工作目录", val.as_str()).as_str()); // 设置工作目录
                }
                Err(_) => panic!("SOI_HOME: {val} 不存在，或者没有权限，请检查")
            }
        }
        Err(_e) => {}
    }
}

pub fn load_config() -> Result<Settings, config::ConfigError> {
    set_home_dir();
    let base_path = env::current_dir().expect("无法确定当前的目录");
    let config_path = base_path.join("config");

    let env: Environment = env::var("SOI_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("环境解析错误");

    let config = config::Config::builder()
        .add_source(config::File::from(
            config_path.join("app.yaml")
        ))
        .add_source(config::File::from(
            config_path.join(format!("{}.yaml", env.as_str()))
        ))
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP_DATABASE__DRIVER=5001 would set `Settings.database.driver`
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;
    config.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} 是不支持的环境配置. 请设置 APP_ENV 为 `local` 或者 `production`.",
                other
            )),
        }
    }
}