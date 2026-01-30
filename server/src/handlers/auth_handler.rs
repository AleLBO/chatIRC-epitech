use axum::{extract::State, http::StatusCode, Json};
use crate::errors::AppResult;
use crate::handlers::middleware::AuthUser;
use crate::models::{AuthResponse, CreateUserDto, LoginDto, User};
use crate::state::AppState;

/// Handler pour l'inscription (POST /auth/signup)
pub async fn signup(
    State(app_state): State<AppState>,
    Json(dto): Json<CreateUserDto>,
) -> AppResult<(StatusCode, Json<AuthResponse>)> {
    let response = app_state.auth_service.register(dto).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

/// Handler pour la connexion (POST /auth/login)
pub async fn login(
    State(app_state): State<AppState>,
    Json(dto): Json<LoginDto>,
) -> AppResult<Json<AuthResponse>> {
    let response = app_state.auth_service.login(dto).await?;
    Ok(Json(response))
}

/// Handler pour récupérer l'utilisateur actuel (GET /auth/me)
pub async fn get_me(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<User>> {
    let user = app_state.auth_service.get_current_user(auth_user.user_id).await?;
    Ok(Json(user))
}
