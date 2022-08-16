use crate::{
    api::{Api, EmailOrPhone},
    error::Error,
    session::Session,
    user::{User, UserAttributes, UserUpdate},
};

pub struct Client {
    pub api: Api,
    current_user: Option<User>,
    current_session: Option<Session>,
}

#[allow(unused)]
impl Client {
    /// Creates a GoTrue Client.
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::Client;
    ///
    /// let client = Client::new("http://your.gotrue.endpoint".to_string());
    /// ```
    pub fn new(url: String) -> Client {
        let api = Api::new(url);

        Client {
            api,
            current_user: None,
            current_session: None,
        }
    }

    /// Signs up a new user.
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client, EmailOrPhone};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Client::new("http://your.gotrue.endpoint".into());
    ///     let email = "some_email".to_string();
    ///     let password = "some_password".to_string();
    ///     let res = client
    ///         .sign_up(EmailOrPhone::Email(email), &password)
    ///         .await?;
    ///     Ok(())
    /// }
    pub async fn sign_up(
        &mut self,
        email_or_phone: EmailOrPhone,
        password: &String,
    ) -> Result<return_data::SignUp, Error> {
        self.remove_session();

        let response = self.api.sign_up(email_or_phone, &password).await;

        match response {
            Ok(result) => {
                let (session, user) = (
                    serde_json::from_value::<Session>(result.clone()).ok(),
                    serde_json::from_value::<User>(result).ok(),
                );

                self.current_session = session.clone();

                Ok(return_data::SignUp { session, user })
            }

            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    return Err(Error::AlreadySignedUp);
                }

                return Err(Error::InternalError);
            }
        }
    }

    /// Signs in a user.
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client, EmailOrPhone};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///     let email = "some_email".to_string();
    ///     let password = "some_password".to_string();
    ///     let res = client
    ///         .sign_in(EmailOrPhone::Email(email), &password)
    ///         .await?;
    ///     Ok(())
    /// }
    pub async fn sign_in(
        &mut self,
        email_or_phone: EmailOrPhone,
        password: &String,
    ) -> Result<return_data::SignIn, Error> {
        self.remove_session();

        let result = self.api.sign_in(email_or_phone, &password).await;

        match result {
            Ok(result) => {
                let (session, user) = (
                    serde_json::from_value::<Session>(result.clone()).ok(),
                    serde_json::from_value::<User>(result).ok(),
                );

                self.current_session = session.clone();

                Ok(return_data::SignIn {
                    session,
                    user,
                    ..Default::default()
                })
            }
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    return Err(Error::WrongCredentials);
                }
                return Err(Error::InternalError);
            }
        }
    }

    /// Sends an OTP
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client, EmailOrPhone};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///     let email = "some_email".to_string();
    ///
    ///     let res = client
    ///         .send_otp(EmailOrPhone::Email(email), None)
    ///         .await?;
    ///     Ok(())
    /// }
    pub async fn send_otp(
        &self,
        email_or_phone: EmailOrPhone,
        should_create_user: Option<bool>,
    ) -> Result<bool, Error> {
        let result = self.api.send_otp(email_or_phone, should_create_user).await;

        match result {
            Ok(_) => return Ok(true),
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "422" {
                    return Err(Error::UserNotFound);
                }
                return Err(Error::InternalError);
            }
        }
    }

    pub async fn verify_otp<T: serde::Serialize>(&mut self, params: T) -> Result<bool, Error> {
        self.current_session = None;
        let result = self.api.verify_otp(params).await;

        match result {
            Ok(_) => return Ok(true),
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    return Err(Error::WrongToken);
                }
                return Err(Error::InternalError);
            }
        }
    }

    /// Sign out the current user
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///
    ///     // Sign in first
    ///
    ///     let res = client.sign_out().await?;
    ///     Ok(())
    /// }
    pub async fn sign_out(&self) -> Result<bool, Error> {
        let result = match &self.current_session {
            Some(session) => self.api.sign_out(&session.access_token).await,
            None => return Err(Error::NotAuthenticated),
        };

        match result {
            Ok(_) => return Ok(true),
            Err(_) => return Err(Error::InternalError),
        }
    }

    /// Reset a user's password for an email address
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///     let email = "some_email".to_string()
    ///
    ///     let res = client.reset_password_for_email(&email).await?;
    ///     Ok(())
    /// }
    pub async fn reset_password_for_email(&self, email: &str) -> Result<bool, Error> {
        let result = self.api.reset_password_for_email(&email).await;

        match result {
            Ok(_) => return Ok(true),
            Err(_) => return Err(Error::UserNotFound),
        }
    }

    pub async fn update_user(&self, user: UserAttributes) -> Result<UserUpdate, Error> {
        let session = match &self.current_session {
            Some(s) => s,
            None => return Err(Error::NotAuthenticated),
        };

        let result = self.api.update_user(user, &session.access_token).await;

        match result {
            Ok(user) => return Ok(user),
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    return Err(Error::UserNotFound);
                }
                return Err(Error::InternalError);
            }
        }
    }

    /// Refreshes the current session
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///
    ///     // sign in first
    ///
    ///     client.refresh_session().await?:
    ///     Ok(())
    /// }
    pub async fn refresh_session(&mut self) -> Result<Session, Error> {
        if self.current_session.is_none() {
            return Err(Error::NotAuthenticated);
        }

        let result = match self.current_session {
            Some(Session {
                refresh_token: Some(ref refresh_token),
                ..
            }) => self.api.refresh_access_token(refresh_token).await,

            _ => return Err(Error::MissingRefreshToken),
        };

        let session = match result {
            Ok(session) => session,

            Err(_) => return Err(Error::InternalError),
        };

        self.current_session = Some(session.clone());

        return Ok(session);
    }

    /// Sets a session by refresh token
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///     let token = "refresh_token".to_string();
    ///
    ///     let session = client.set_session(token).await?:
    ///     Ok(())
    /// }
    pub async fn set_session(&mut self, refresh_token: &str) -> Result<Session, Error> {
        if refresh_token.len() < 1 {
            return Err(Error::NotAuthenticated);
        }

        let result = self.api.refresh_access_token(refresh_token).await;

        let session = match result {
            Ok(session) => session,
            Err(_) => return Err(Error::InternalError),
        };

        self.current_session = Some(session.clone());

        return Ok(session);
    }

    fn remove_session(&mut self) {
        self.current_session = None;
    }

    fn user(&self) -> &Option<User> {
        &self.current_user
    }

    fn session(&self) -> &Option<Session> {
        &self.current_session
    }
}

mod return_data {

    use crate::session::Session;
    use crate::user::User;

    #[derive(Debug, Default)]
    pub struct SignUp {
        pub user: Option<User>,
        pub session: Option<Session>,
    }

    #[derive(Debug, Default)]
    pub struct SignIn {
        pub user: Option<User>,
        pub session: Option<Session>,
        pub url: Option<String>,
        pub provider: Option<String>,
    }
}
