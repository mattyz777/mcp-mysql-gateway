
use serde::Deserialize;
use std::collections::HashMap;

/// config.toml
#[derive(Debug, Deserialize)]
pub struct ConfigDto {
    pub mysql: MySqlConfig,
    pub users: HashMap<String, String>, // username -> token
}

/// expected type in program code
#[derive(Debug, Clone)]
pub struct Config {
    pub mysql: MySqlConfig,
    pub auth_users: HashMap<String, String>, // token -> username
}

#[derive(Clone, Debug, Deserialize)]
pub struct MySqlConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
}

impl MySqlConfig {
    pub fn database_url(&self) -> String {
        let password = urlencoding::encode(&self.password);
        format!("mysql://{}:{}@{}:{}/{}", self.user, password, self.host, self.port, self.database)
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let content = std::fs::read_to_string("config.toml")?;
        let config_dto:ConfigDto = toml::from_str(&content)?;

        let mut token_user_map = HashMap::new();

        for (username, token) in config_dto.users {
            token_user_map.insert(token, username);
        }

        Ok(Self {
            mysql: config_dto.mysql,
            auth_users: token_user_map,
        })
    }
}
