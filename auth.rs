use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::post,
    Router, ServerError,
    extract::{State, FromRequestParts, Request, Body},
    middleware::{self, Next},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header, EncodingKey, Validation, DecodingKey, TokenData};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, Row};
use crate::models::{User, CreateUser};
use crate::utils::hash_password;
use std::sync::Arc;
use async_trait::async_trait;

// Custom error type for authentication
#[derive(Debug, Serialize)]
pub enum AuthError {
    WrongCredentials,
    Unauthorized,
    DatabaseError,
    TokenExpired,
    TokenInvalid,
    MissingAuthHeader,
    InternalServerError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AuthError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired"),
            AuthError::TokenInvalid => (StatusCode::UNAUTHORIZED, "Token invalid"),
            AuthError::MissingAuthHeader => (StatusCode::BAD_REQUEST, "Missing authorization header"),
            AuthError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };
        Json(serde_json::json!({
            "error": message,
        })).into_response()
    }
}

impl From<sqlx::Error> for AuthError {
    fn from(_: sqlx::Error) -> Self {
        AuthError::DatabaseError
    }
}

// Representation of the claims within the JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64, // User ID
    pub exp: i64, // Expiration timestamp (Unix)
}

// Utility function to generate a JWT.
fn generate_jwt(user_id: i64, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now() + Duration::hours(24); // Token expires in 24 hours
    let claims = Claims {
        sub: user_id,
        exp: expiration.timestamp(),
    };
    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    encode(&header, &claims, &encoding_key)
}

// Handler for user registration.
async fn register(
    State(pool): State<Pool<Sqlite>>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, AuthError> {
    let hashed_password = hash_password(&payload.password)
        .map_err(|_| AuthError::InternalServerError)?; // Use our utility function

    let mut tx = pool.begin().await?;
    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES (?, ?)",
        payload.username,
        hashed_password
    )
    .execute(&mut tx)
    .await?;

    let user_id = result.last_insert_rowid();

    tx.commit().await?;

    let user = sqlx::query_as!(
        User,
        "SELECT id, username, password_hash, created_at, updated_at FROM users WHERE id = ?",
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(json!({
        "user": user,
        "message": "User registered successfully",
    })))
}

// Handler for user login.
async fn login(
    State(pool): State<Pool<Sqlite>>,
    Json(payload): Json<CreateUser>, // Reuse CreateUser for simplicity, but you might want a LoginUser
    State(jwt_secret): State<Arc<String>>,
) -> Result<impl IntoResponse, AuthError> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, password_hash, created_at, updated_at FROM users WHERE username = ?",
        payload.username
    )
    .fetch_one(&pool)
    .await?;

    if bcrypt::verify(payload.password, &user.password_hash).unwrap_or(false) {
        let token = generate_jwt(user.id, &jwt_secret)
            .map_err(|_| AuthError::InternalServerError)?;
        Ok(Json(json!({
            "token": token,
            "user": user,
            "message": "Login successful",
        })))
    } else {
        Err(AuthError::WrongCredentials)
    }
}

// Middleware to extract and verify the JWT from the Authorization header.
pub async fn auth_middleware<B>(req: Request<B>, next: Next<B>, jwt_secret: Arc<String>) -> Result<Response, AuthError> {
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = auth_header
        .ok_or(AuthError::MissingAuthHeader)?
        .trim_start_matches("Bearer ")
        .to_string();


    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let validation = Validation::default();
    let token_data: TokenData<Claims> =
        jsonwebtoken::decode(&token, &decoding_key, &validation)
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::TokenInvalid,
        })?;

    // Store the user ID in a request extension so it can be accessed by later handlers.
    req.extensions_mut().insert(token_data.claims.sub);

    Ok(next.run(req).await)
}

// Extract user ID from request extensions
pub async fn get_user_id<B>(req: Request<B>) -> Result<i64, AuthError> {
    req.extensions()
        .get::<i64>()
        .copied() // Extract the i64 value
        .ok_or(AuthError::Unauthorized)
}


pub fn auth_router(jwt_secret: Arc<String>) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .layer(middleware::from_fn(move |req, next| auth_middleware(req, next, jwt_secret.clone()))) // Apply the middleware
}
