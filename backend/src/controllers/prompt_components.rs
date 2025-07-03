use axum::{
    extract::{Path, State},
    Json,
};
use crate::{AppState, AppError};
use crate::db::models::PromptComponent;

pub async fn create_component(
    State(state): State<AppState>,
    Json(payload): Json<PromptComponent>,
) -> Result<Json<i64>, AppError> {
    let id = state.db.prompt_component.create_component(
        &payload.name,
        &payload.content,
        payload.description.as_deref(),
    ).await?;
    Ok(Json(id))
}

pub async fn get_component(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<PromptComponent>, AppError> {
    let component = state.db.prompt_component.get_component(id).await?;
    Ok(Json(component))
}

pub async fn list_components(
    State(state): State<AppState>,
) -> Result<Json<Vec<PromptComponent>>, AppError> {
    let components = state.db.prompt_component.list_components().await?;
    Ok(Json(components))
}

pub async fn update_component(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<PromptComponent>,
) -> Result<Json<PromptComponent>, AppError> {
    let updated = state.db.prompt_component.update_component(
        id,
        &payload.name,
        &payload.content,
        payload.description.as_deref(),
    ).await?;
    if !updated {
        return Err(AppError::NotFound("Component not found".into()));
    }
    let component = state.db.prompt_component.get_component(id).await?;
    Ok(Json(component))
}

pub async fn delete_component(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<(), AppError> {
    let deleted = state.db.prompt_component.delete_component(id).await?;
    if !deleted {
        return Err(AppError::NotFound("Component not found".into()));
    }
    Ok(())
}