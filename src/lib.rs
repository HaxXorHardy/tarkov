use crate::auth::LoginError;
use actix_web::client::Client;
use actix_web::http::StatusCode;
use err_derive::Error;
use crate::profile::{ProfileError, SelectError};
use serde::Deserialize;

const GAME_VERSION: &str = "0.12.2.5485";
const LAUNCHER_VERSION: &str = "0.9.1.935";
const UNITY_VERSION: &str = "2018.4.13f1";

const LAUNCHER_ENDPOINT: &str = "https://launcher.escapefromtarkov.com";
const PROD_ENDPOINT: &str = "https://prod.escapefromtarkov.com";
const TRADING_ENDPOINT: &str = "https://trading.escapefromtarkov.com";

mod auth;
mod profile;
mod friend;
pub mod hwid;

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "io error: {}", _0)]
    Io(#[error(source)] std::io::Error),
    #[error(display = "send request error: {}", _0)]
    SendRequestError(#[error(from)] actix_web::client::SendRequestError),
    #[error(display = "payload error: {}", _0)]
    PayloadError(#[error(from)] actix_web::client::PayloadError),
    #[error(display = "json error: {}", _0)]
    Json(#[error(source)] serde_json::error::Error),
    #[error(display = "non-success response from api: {}", _0)]
    Status(StatusCode),

    #[error(display = "unidentified login error with error code: {}", _0)]
    UnknownAPIError(u8),
    #[error(display = "login api error: {}", _0)]
    LoginError(#[error(source)] LoginError),
    #[error(display = "profile api error: {}", _0)]
    ProfileError(#[error(source)] ProfileError),
    #[error(display = "profile select api error: {}", _0)]
    SelectError(#[error(source)] SelectError),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    #[serde(rename = "err")]
    error_code: u8,
    #[serde(rename = "errmsg")]
    error_message: Option<String>,
}

pub struct Tarkov {
    client: Client,
    hwid: String,
    pub session: String,
}

impl Tarkov {
    pub async fn new(email: &str, password: &str, hwid: &str) -> Result<Self> {
        let client = Client::new();

        let user = auth::login(&client, email, password, &hwid).await?;
        let session = auth::exchange_access_token(&client, &user.access_token, &hwid).await?;

        Ok(Tarkov {
            client,
            hwid: hwid.to_string(),
            session: session.session,
        })
    }

    pub async fn from_access_token(access_token: &str, hwid: &str) -> Result<Self> {
        let client = Client::new();
        let session = auth::exchange_access_token(&client, &access_token, &hwid).await?;

        Ok(Tarkov {
            client,
            hwid: hwid.to_string(),
            session: session.session,
        })
    }

    pub async fn from_session(session: &str, hwid: &str) -> Result<Self> {
        let client = Client::new();

        Ok(Tarkov {
            client,
            hwid: hwid.to_string(),
            session: session.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
