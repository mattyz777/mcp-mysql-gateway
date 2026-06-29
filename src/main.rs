use app::config::Config;
use app::logging;
use app::router::build_router;
use app::state::AppState;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use tracing::info;


#[tokio::main]
async fn main() -> anyhow::Result<()>{
    logging::init();
    
    let config = Config::load()?;
    let pool = init_mysql_pool(config.clone()).await;
    let state = AppState::new(config, pool);
    
    let app = build_router(state);

    let addr: &str = "0.0.0.0:5111";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("MySQL MCP gateway listening on http://{addr}/mcp");

    axum::serve(listener, app.into_make_service_with_connect_info::<std::net::SocketAddr>()).await?;

    Ok(())
}


async fn init_mysql_pool(config: Config) -> MySqlPool {
    let database_url = config.mysql.database_url();
    match MySqlPoolOptions::new()
        .max_connections(config.mysql.max_connections)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("connected to MySQL");
            pool
        }
        Err(e) => {
            eprintln!("Failed to connect to MySQL: {}", e);
            std::process::exit(1);
        }
    }
}