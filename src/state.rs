use sqlx::MySqlPool;
use std::sync::Arc;
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub pool: MySqlPool,
}

impl AppState {
    pub fn new(config: Config, pool: MySqlPool) -> Self {
        Self { pool, config: Arc::new(config) }
    }
}