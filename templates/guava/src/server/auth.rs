use async_session::{async_trait, MemoryStore, Session, SessionStore};
use axum::{
    extract::{FromRequest, Query, RequestParts},
    headers::Cookie,
    http::{self, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Extension, TypedHeader,
};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use super::api::commands::State;

pub enum GuavaSession {
    FoundUser(SessionUser),
}

const AXUM_SESSION_COOKIE_NAME: &str = "axum_session";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct LoginParams {
    username: String,
    password: String,
}


#[derive(Debug, SmartDefault, Clone, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: i64,
    pub name: String,
    pub password: String,
    pub store_id: i64,
    pub dept_id: i64,
    pub tenant_id: i64,
}


pub async fn login(
    Query(params): Query<LoginParams>,
    Extension(state): State,
    Extension(store): Extension<MemoryStore>,
) -> impl IntoResponse {
    let result = state.service.repo.select_session_user_by_name(&state.service.db, &params.username).await;
    if let Ok(user) = result  {
        if params.password.eq(&user.password) {
            let mut session = Session::new();
            session.insert("user_bo", user).unwrap();
            let cookie = store.store_session(session).await.unwrap().unwrap();
            let mut headers = HeaderMap::new();
            let cookie =
                HeaderValue::from_str(format!("{}={}", AXUM_SESSION_COOKIE_NAME, cookie).as_str())
                    .unwrap();
            headers.insert(http::header::SET_COOKIE, cookie);
            return (headers, StatusCode::OK)
        }

    }
    (HeaderMap::new(), StatusCode::UNAUTHORIZED)

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

pub async fn check_auth(GuavaSession::FoundUser(user): GuavaSession) -> impl IntoResponse {
    println!("{:?}", format!("{:?}", user));
    "ok"
}

#[async_trait]
impl<B> FromRequest<B> for GuavaSession
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
        let user_bo = if let Some(session) = store
            .load_session(session_cookie.unwrap().to_owned())
            .await
            .unwrap()
        {
            if let Some(user_bo) = session.get::<SessionUser>("user_bo") {
                log::debug!(
                    "UserIdFromSession: session decoded success, user_bo={:?}",
                    user_bo
                );
                user_bo
            } else {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "No `user_bo` found in session",
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

        Ok(Self::FoundUser(user_bo))
    }
}
