use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UserIdentity {
    pub id: String,
    pub user_id: String,
    pub identity_data: Value,
    pub provider: String,
    pub created_at: String,
    pub last_sign_in_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct User {
    pub id: String,
    pub app_metadata: Value,
    pub user_metadata: Value,
    pub aud: String,
    pub confirmation_sent_at: Option<String>,
    pub recovery_sent_at: Option<String>,
    pub email_change_sent_at: Option<String>,
    pub new_email: Option<String>,
    pub invited_at: Option<String>,
    pub action_link: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: String,
    pub confirmed_at: Option<String>,
    pub email_confirmed_at: Option<String>,
    pub phone_confirmed_at: Option<String>,
    pub last_sign_in_at: Option<String>,
    pub role: Option<String>,
    pub updated_at: Option<String>,
    pub identities: Option<Vec<UserIdentity>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UserAttributes {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub email_change_token: Option<String>,
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserList {
    pub users: Vec<User>,
}

#[derive(Debug, Default, Deserialize)]
pub struct UserUpdate {
    pub id: String,
    pub email: String,
    pub new_email: String,
    pub email_change_sent_at: String,
    pub created_at: String,
    pub updated_at: String,
}
