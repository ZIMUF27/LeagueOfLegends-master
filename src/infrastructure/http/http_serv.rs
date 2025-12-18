use crate ::{
    config::config_model::DotEnvyConfig,
    infrastructure::{database::postgresql_connection::PgPoolSquad, http::routers::default_router},
};

use anyhow:: Result;
use std::{default,   sync :: Arc};

pub async fn start(config: Arc<DotEnvyConfig>, pool: Arc<PgPoolSquad>) -> Result<()> {
    let app = Router::new().fallback(default_router::not_found)
    .route("/health-check", get(default_router::health_check))
    .layer(TimeoutLayer::new().allow_method([
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::PATCH,
        Method::OPTIONS,
    ]))
    .allow_origin(Any)
    .allow_headers([
        AUTHORIZATION,
        CONTENT_TYPE,
    ])
    .layer(TraceLayer::new_for_http());
pub async fn start(co)
}