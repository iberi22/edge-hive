use axum::Json;
use serde::Serialize;
use crate::middleware::auth::AuthenticatedUser;

#[derive(Serialize)]
pub struct UserProfile {
    pub email: String,
    pub name: Option<String>,
}

pub async fn get_profile(
    AuthenticatedUser(user): AuthenticatedUser,
) -> Json<UserProfile> {
    Json(UserProfile {
        email: user.email,
        name: user.name,
    })
}
