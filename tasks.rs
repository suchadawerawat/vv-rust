use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::{IntoResponse, Json as AxumJson},
    routing::{get, post, put, delete},
    Router,
};
use serde_json::json;
use sqlx::{Pool, Sqlite};
use crate::{
    models::{Task, CreateTask, UpdateTask},
    api::auth::get_user_id,
};

// Handler to create a new task.
async fn create_task(
    State(pool): State<Pool<Sqlite>>,
    user_id: Result<i64, crate::api::auth::AuthError>,
    Json(payload): Json<CreateTask>,
) -> Result<impl IntoResponse, crate::api::auth::AuthError> {
    let user_id = user_id?; // Ensure the user is authenticated

    let result = sqlx::query!(
        "INSERT INTO tasks (user_id, title, description) VALUES (?, ?, ?)",
        user_id,
        payload.title,
        payload.description
    )
    .execute(&pool)
    .await?;

    let task_id = result.last_insert_rowid();

    let task = sqlx::query_as!(
        Task,
        "SELECT id, user_id, title, description, completed, created_at, updated_at FROM tasks WHERE id = ?",
        task_id
    )
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, AxumJson(task)))
}

// Handler to get all tasks for the authenticated user.
async fn list_tasks(
    State(pool): State<Pool<Sqlite>>,
    user_id: Result<i64, crate::api::auth::AuthError>,
) -> Result<impl IntoResponse, crate::api::auth::AuthError> {
    let user_id = user_id?; // Ensure the user is authenticated

    let tasks = sqlx::query_as!(
        Task,
        "SELECT id, user_id, title, description, completed, created_at, updated_at FROM tasks WHERE user_id = ?",
        user_id
    )
    .fetch_all(&pool)
    .await?;

    Ok(AxumJson(tasks))
}

// Handler to get a specific task by ID.
async fn get_task(
    State(pool): State<Pool<Sqlite>>,
    user_id: Result<i64, crate::api::auth::AuthError>,
    Path(task_id): Path<i64>,
) -> Result<impl IntoResponse, crate::api::auth::AuthError> {
    let user_id = user_id?; // Ensure the user is authenticated

    let task = sqlx::query_as!(
        Task,
        "SELECT id, user_id, title, description, completed, created_at, updated_at FROM tasks WHERE id = ? AND user_id = ?",
        task_id,
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(AxumJson(task))
}

// Handler to update a task.
async fn update_task(
    State(pool): State<Pool<Sqlite>>,
    user_id: Result<i64, crate::api::auth::AuthError>,
    Path(task_id): Path<i64>,
    Json(payload): Json<UpdateTask>,
) -> Result<impl IntoResponse, crate::api::auth::AuthError> {
    let user_id = user_id?; // Ensure the user is authenticated

    let mut tx = pool.begin().await?;

    let result = sqlx::query!(
        "UPDATE tasks SET title = COALESCE(?, title), description = COALESCE(?, description), completed = COALESCE(?, completed), updated_at = CURRENT_TIMESTAMP WHERE id = ? AND user_id = ?",
        payload.title,
        payload.description,
        payload.completed,
        task_id,
        user_id
    )
    .execute(&mut tx)
    .await?;

    if result.rows_affected() == 0 {
        return Err(crate::api::auth::AuthError::Unauthorized); // Task not found or not owned by user
    }

    tx.commit().await?;

    let updated_task = sqlx::query_as!(
        Task,
        "SELECT id, user_id, title, description, completed, created_at, updated_at FROM tasks WHERE id = ?",
        task_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(AxumJson(updated_task))
}

// Handler to delete a task.
async fn delete_task(
    State(pool): State<Pool<Sqlite>>,
    user_id: Result<i64, crate::api::auth::AuthError>,
    Path(task_id): Path<i64>,
) -> Result<impl IntoResponse, crate::api::auth::AuthError> {
    let user_id = user_id?; // Ensure the user is authenticated

    let result = sqlx::query!(
        "DELETE FROM tasks WHERE id = ? AND user_id = ?",
        task_id,
        user_id
    )
    .execute(&pool)
    .await?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(crate::api::auth::AuthError::Unauthorized) // Task not found or not owned by user
    }
}

pub fn tasks_router() -> Router {
    Router::new()
        .route("/", post(create_task).get(list_tasks))
        .route("/:task_id", get(get_task).put(update_task).delete(delete_task))
        .route_layer(axum::middleware::from_fn(crate::api::auth::auth_middleware)) // Apply auth middleware to all task routes
}
