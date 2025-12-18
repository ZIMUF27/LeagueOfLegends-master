use std::sync::Arc;

use axum::Json;
use axum::{routing::get, Router};

use crate::infrastructure::database::
{postgres_connection::PgpoolSquad, repositories::BrawlerPostgres};

use crate::application::use_cases::brawlers::BrawlerUseCase;

pub fn routes(db_pool: Arc<PgpoolSquad>) -> Router {
    let repository: Arc<dyn BrawlerPostgres> = Arc::new(db_pool);
    let use_case = BrawlerUseCase::new(repository);

    Router::new()
    .route("/register",post(register))
    .with_state(Arc::new(use_case))
}

pub async fn register<T>(
    State(user_case):State<Arc<BrawlerUseCase<T>>>,
    Json(model): Json<RegisterBrawlerModel>,
    
)-> impl IntoResponse 
where T: BrawlerRepository + Send + Sync {
    match user_case.register(model).await {
        Ok(user_id) => (StatusCode::CREATED,user_id.to_string()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR,e.to_string()).into_response(),
    }
}


