// --- Prompt Directory API ---
use crate::db::models::{PromptDirectory, PromptComponent};
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DirectoryQuery {
    pub parent_id: Option<i64>,
}

pub async fn create_directory(
    State(state): State<AppState>,
    Json(payload): Json<PromptDirectory>,
) -> Result<Json<i64>, AppError> {
    let id = state.db.prompt.create_directory(&payload.name, payload.parent_id).await?;
    Ok(Json(id))
}

pub async fn get_directory(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<Option<PromptDirectory>>, AppError> {
    let dir = state.db.prompt.get_directory(id).await?;
    Ok(Json(dir))
}

pub async fn list_directories(
    State(state): State<AppState>,
    Query(query): Query<DirectoryQuery>,
) -> Result<Json<Vec<PromptDirectory>>, AppError> {
    let dirs = state.db.prompt.list_directories(query.parent_id).await?;
    Ok(Json(dirs))
}

pub async fn update_directory(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<PromptDirectory>,
) -> Result<Json<bool>, AppError> {
    let updated = state.db.prompt.update_directory(id, &payload.name, payload.parent_id).await?;
    Ok(Json(updated))
}

pub async fn delete_directory(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<bool>, AppError> {
    let deleted = state.db.prompt.delete_directory(id).await?;
    Ok(Json(deleted))
}