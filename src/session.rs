use crate::user::User;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Session {
    pub provider_token: Option<String>,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<i32>,
    pub refresh_token: Option<String>,
    pub user: Option<User>,
}
