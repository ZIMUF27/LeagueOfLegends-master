use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, patch, post},
};

use crate::{
    application::use_cases::{
        crew_operation::CrewOperationUseCase, mission_management::MissionManagementUseCase,
    },
    domain::{
        repositories::{
            crew_operation::CrewOperationRepository,
            mission_management::MissionManagementRepository,
            mission_viewing::MissionViewingRepository,
        },
        value_objects::mission_model::{AddMissionModel, EditMissionModel},
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{
                mission_management::MissionManagementPostgres,
                mission_viewing::MissionViewingPostgres,
            },
        },
        http::middlewares::auth::auth,
    },
};

pub async fn join<T1, T2>(
    State(user_case): State<Arc<CrewOperationUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    match user_case.join(mission_id, user_id).await {
        Ok(_) => (
            StatusCode::OK,
            format!("Join Mission_id:{} completed", mission_id),
        )
            .into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn leave<T1, T2>(
    State(user_case): State<Arc<CrewOperationUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    match user_case.leave(mission_id, user_id).await {
        Ok(_) => (
            StatusCode::OK,
            format!("Leave Mission_id:{} completed", mission_id),
        )
            .into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let crew_operation = CrewOperationPostgres::new(Arc::clone(&db_pool));
    let viewing_repositiory = CrewOperationPostgres::new(Arc::clone(&db_pool));
    let user_case =
        CrewOperationUseCase::new(Arc::new(crew_operation), Arc::new(viewing_repositiory));

    Router::new()
        .route("/join/{mission}", post(add))
        .route("/{mission_id}", patch(edit))
        .route("/{mission_id}", delete(remove))
        .route_layer(middleware::from_fn(auth))
        .with_state(Arc::new(user_case))
}
