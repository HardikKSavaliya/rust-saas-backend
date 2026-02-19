use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::state::AppState;

use super::entity::{self, Entity as Users};

#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub name: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl From<entity::Model> for UserResponse {
    fn from(model: entity::Model) -> Self {
        Self {
            id: model.id,
            email: model.email,
            name: model.name,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

/// POST /api/users
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<impl IntoResponse> {
    // TODO: Hash password with bcrypt/argon2 before storing
    let user = entity::ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(payload.email),
        name: Set(payload.name),
        password_hash: Set(payload.password),
        is_active: Set(true),
        created_at: Set(chrono::Utc::now().fixed_offset()),
        updated_at: Set(chrono::Utc::now().fixed_offset()),
    };

    let user = user.insert(&state.db).await.map_err(AppError::from)?;
    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

/// GET /api/users
pub async fn list_users(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    let users = Users::find().all(&state.db).await.map_err(AppError::from)?;
    let responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
    Ok(Json(responses))
}

/// GET /api/users/:id
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let user = Users::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(AppError::from)?
        .ok_or(AppError::NotFound(format!(
            "User with id {} not found",
            id
        )))?;

    Ok(Json(UserResponse::from(user)))
}

/// PUT /api/users/:id
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> AppResult<impl IntoResponse> {
    let user = Users::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(AppError::from)?
        .ok_or(AppError::NotFound(format!(
            "User with id {} not found",
            id
        )))?;

    let mut user: entity::ActiveModel = user.into();

    if let Some(email) = payload.email {
        user.email = Set(email);
    }
    if let Some(name) = payload.name {
        user.name = Set(name);
    }
    if let Some(is_active) = payload.is_active {
        user.is_active = Set(is_active);
    }
    user.updated_at = Set(chrono::Utc::now().fixed_offset());

    let user = user.update(&state.db).await.map_err(AppError::from)?;
    Ok(Json(UserResponse::from(user)))
}

/// DELETE /api/users/:id
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let result = Users::delete_by_id(id)
        .exec(&state.db)
        .await
        .map_err(AppError::from)?;

    if result.rows_affected == 0 {
        return Err(AppError::NotFound(format!(
            "User with id {} not found",
            id
        )));
    }

    Ok(StatusCode::NO_CONTENT)
}
