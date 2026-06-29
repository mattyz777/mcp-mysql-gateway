use std::sync::Arc;
use sqlx::{AssertSqlSafe, Column, MySqlPool, Row};
use tracing::info;

use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_handler, tool_router, ServerHandler};
use rmcp::model::{CallToolResult, Content, ServerCapabilities, ServerInfo};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::Config;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DatabaseParams {
    #[schemars(description = "database name")]
    database: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableParams {
    #[schemars(description = "database name")]
    database: String,
    #[schemars(description = "table name")]
    table: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct QueryParams {
    #[schemars(description = "SQL SELECT query")]
    sql: String,
}

#[derive(Clone)]
pub struct MysqlMcp {
    pub pool: MySqlPool,
    pub config: Arc<Config>,
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

impl MysqlMcp {
    pub fn new(pool: MySqlPool, config: Arc<Config>) -> Self {
        Self {
            pool,
            config,
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl MysqlMcp {
    #[tool(description = "list all databases")]
    pub async fn list_databases(&self) -> CallToolResult {
        info!(tool = "list_databases", "tool called");
        let rows = sqlx::query("SHOW DATABASES").fetch_all(&self.pool).await;
        
        match rows {
            Ok(rows) => {
                let databases: Vec<String> = rows
                    .iter()
                    .filter_map(|r| r.try_get(0).ok())
                    .collect();
                CallToolResult::success(vec![Content::text(
                    serde_json::to_string(&databases).unwrap_or_default()
                )])
            }
            Err(e) => CallToolResult::success(vec![Content::text(
                serde_json::json!({"error": e.to_string()}).to_string()
            )])
        }
    }

    #[tool(description = "list all tables by database")]
    pub async fn list_tables(&self, Parameters(params): Parameters<DatabaseParams>) -> CallToolResult {
        info!(tool = "list_tables", database = %params.database, "tool called");
        let rows = sqlx::query("SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = ?")
            .bind(&params.database)
            .fetch_all(&self.pool)
            .await;
        
        match rows {
            Ok(rows) => {
                let tables: Vec<String> = rows
                    .iter()
                    .filter_map(|r| r.try_get(0).ok())
                    .collect();
                CallToolResult::success(vec![Content::text(
                    serde_json::to_string(&tables).unwrap_or_default()
                )])
            }
            Err(e) => CallToolResult::success(vec![Content::text(
                serde_json::json!({"error": e.to_string()}).to_string()
            )])
        }
    }

    #[tool(description = "describe table schema")]
    pub async fn describe_table(&self, Parameters(params): Parameters<TableParams>) -> CallToolResult {
        info!(tool = "describe_table", database = %params.database, table = %params.table, "tool called");
        let rows = sqlx::query("SELECT COLUMN_NAME, COLUMN_TYPE, IS_NULLABLE, COLUMN_KEY, COLUMN_DEFAULT, EXTRA FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ? ORDER BY ORDINAL_POSITION")
            .bind(&params.database)
            .bind(&params.table)
            .fetch_all(&self.pool)
            .await;
        
        match rows {
            Ok(rows) => {
                let columns: Vec<serde_json::Value> = rows
                    .iter()
                    .filter_map(|r| {
                        Some(serde_json::json!({
                            "field": r.try_get::<String, _>(0).ok()?,
                            "type": r.try_get::<String, _>(1).ok()?,
                            "null": r.try_get::<String, _>(2).ok()?,
                            "key": r.try_get::<String, _>(3).ok()?,
                            "default": r.try_get::<Option<String>, _>(4).ok()?.unwrap_or_default(),
                            "extra": r.try_get::<String, _>(5).ok()?,
                        }))
                    })
                    .collect();
                CallToolResult::success(vec![Content::text(
                    serde_json::to_string(&columns).unwrap_or_default()
                )])
            }
            Err(e) => CallToolResult::success(vec![Content::text(
                serde_json::json!({"error": e.to_string()}).to_string()
            )])
        }
    }

    #[tool(description = "execute SQL SELECT query")]
    pub async fn query(&self, Parameters(params): Parameters<QueryParams>) -> CallToolResult {
        info!(tool = "query", sql = %params.sql, "tool called");
        let sql: String = params.sql.trim().to_string();
        
        if !sql.to_uppercase().starts_with("SELECT") {
            return CallToolResult::success(vec![Content::text(
                serde_json::json!({"error": "Only SELECT queries are allowed"}).to_string()
            )]);
        }

        let rows = sqlx::query(AssertSqlSafe(sql)).fetch_all(&self.pool).await;
        
        match rows {
            Ok(rows) => {
                let results: Vec<serde_json::Value> = rows
                    .iter()
                    .map(|r| {
                        let mut obj = serde_json::Map::new();
                        for i in 0..r.len() {
                            if let Some(col) = r.columns().get(i) {
                                let name = col.name();
                                let value: Option<String> = r.try_get(i).ok();
                                obj.insert(name.to_string(), serde_json::Value::String(value.unwrap_or_default()));
                            }
                        }
                        serde_json::Value::Object(obj)
                    })
                    .collect();
                CallToolResult::success(vec![Content::text(
                    serde_json::to_string(&results).unwrap_or_default()
                )])
            }
            Err(e) => CallToolResult::success(vec![Content::text(
                serde_json::json!({"error": e.to_string()}).to_string()
            )])
        }
    }
}

#[tool_handler]
impl ServerHandler for MysqlMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .build(),
        )
        .with_instructions("MySQL MCP server".to_string())
    }
}