use async_session::{async_trait, MemoryStore, Session, SessionStore};
use axum::{
    extract::{FromRequest, Query, RequestParts},
    headers::Cookie,
    http::{self, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Extension, TypedHeader,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum UserSession {
    FoundUser(UserId),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct UserId(Uuid);

impl UserId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

const AXUM_SESSION_COOKIE_NAME: &str = "axum_session";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct LoginParams {
    username: String,
    password: String,
}

pub async fn login(
    Query(params): Query<LoginParams>,
    Extension(store): Extension<MemoryStore>,
) -> impl IntoResponse {
    if params.username.eq("123") && params.password.eq("123") {
        let user_id = UserId::new();
        let mut session = Session::new();
        session.insert("user_id", user_id).unwrap();
        let cookie = store.store_session(session).await.unwrap().unwrap();
        let mut headers = HeaderMap::new();
        let cookie =
            HeaderValue::from_str(format!("{}={}", AXUM_SESSION_COOKIE_NAME, cookie).as_str())
                .unwrap();
        headers.insert(http::header::SET_COOKIE, cookie);
        (headers, StatusCode::OK)
    } else {
        (HeaderMap::new(), StatusCode::UNAUTHORIZED)
    }
}

pub async fn logout(
    Extension(store): Extension<MemoryStore>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> StatusCode {
    let session_cookie = cookie.get(AXUM_SESSION_COOKIE_NAME);
    if session_cookie.is_none() {
        return StatusCode::OK;
    }
    if let Some(session) = store
        .load_session(session_cookie.unwrap().to_owned())
        .await
        .unwrap()
    {
        store.destroy_session(session).await;
    }
    StatusCode::OK
}

pub async fn check_auth(UserSession::FoundUser(user): UserSession) -> impl IntoResponse {
    println!("{:?}", format!("{:?}", user));
    "ok"
}

#[async_trait]
impl<B> FromRequest<B> for UserSession
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(store) = Extension::<MemoryStore>::from_request(req)
            .await
            .expect("`MemoryStore` extension missing");

        let cookie = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .unwrap();

        let session_cookie = cookie
            .as_ref()
            .and_then(|cookie| cookie.get(AXUM_SESSION_COOKIE_NAME));

        // return the new created session cookie for client
        if session_cookie.is_none() {
            return Err((StatusCode::UNAUTHORIZED, "用户未登录"));
        }

        log::debug!(
            "UserIdFromSession: got session cookie from user agent, {}={}",
            AXUM_SESSION_COOKIE_NAME,
            session_cookie.unwrap()
        );
        // continue to decode the session cookie
        let user_id = if let Some(session) = store
            .load_session(session_cookie.unwrap().to_owned())
            .await
            .unwrap()
        {
            if let Some(user_id) = session.get::<UserId>("user_id") {
                log::debug!(
                    "UserIdFromSession: session decoded success, user_id={:?}",
                    user_id
                );
                user_id
            } else {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "No `user_id` found in session",
                ));
            }
        } else {
            log::debug!(
                "UserIdFromSession: err session not exists in store, {}={}",
                AXUM_SESSION_COOKIE_NAME,
                session_cookie.unwrap()
            );
            return Err((StatusCode::UNAUTHORIZED, "No session found for cookie"));
        };

        Ok(Self::FoundUser(user_id))
    }
}
