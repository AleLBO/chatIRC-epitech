use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use crate::errors::AppError;
use crate::utils::verify_token;

/// Extractor pour l'utilisateur authentifié
pub struct AuthUser {
    pub user_id: i32,
    pub username: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extraire le header Authorization
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized)?;

        // Vérifier le token
        let claims = verify_token(bearer.token())?;

        Ok(AuthUser {
            user_id: claims.sub,
            username: claims.username,
        })
    }
}
