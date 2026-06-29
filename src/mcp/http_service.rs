use std::sync::Arc;

use rmcp::transport::StreamableHttpService;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::tower::StreamableHttpServerConfig;

use crate::state::AppState;
use super::mysql_mcp::MysqlMcp;

pub fn build_mcp_http_service(
    state: &AppState,
) -> StreamableHttpService<MysqlMcp, LocalSessionManager> {
    let pool = state.pool.clone();
    let config = Arc::clone(&state.config);

    let factory = move || Ok(MysqlMcp::new(pool.clone(), Arc::clone(&config)));

    StreamableHttpService::new(
        factory,
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default().disable_allowed_hosts(),
    )
}