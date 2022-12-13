use std::{sync::Arc, collections::HashMap};

use async_session::{async_trait, MemoryStore, Session, SessionStore};
use axum::{
    extract::{FromRequest, Query, RequestParts},
    headers::Cookie,
    http::{self, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Extension, TypedHeader,
};
use rbatis::{intercept::SqlIntercept, Rbatis};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use crate::{drivers::{db::DB, cache::ServiceCache}, server::api::commands::Resp, service::Service, AppState};

use super::api::commands::{State, AppResult};

pub enum GuavaSession {
    FoundUser((SessionUser, Service)),
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
    Extension(service): State,
    Extension(store): Extension<MemoryStore>,
) -> impl IntoResponse {
    let result = service.repo.select_session_user_by_name(&service.db, &params.username).await;
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

pub async fn check_auth(GuavaSession::FoundUser((user, service2)): GuavaSession, Extension(service): Extension<Arc<Service>>) -> AppResult<i64> {
    // println!("{:?}", format!("{:?}", user));
    let result = service.count_category().await?;
    Resp::ok(result)
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

        let Extension(db) = Extension::<DB>::from_request(req)
            .await
            .expect("`DB` extension missing");

        let Extension(cache) = Extension::<Arc<HashMap<String, ServiceCache>>>::from_request(req)
            .await
            .expect("`Arc<HashMap<String, ServiceCache>>` extension missing");

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

        let mut db = db.clone();
        db.set_sql_intercepts(vec![Box::new(UserIntercept{
            user: user_bo.clone()
        })]);
        Ok(Self::FoundUser((user_bo.clone(), Service::new_scope(db, user_bo.clone(), cache))))
    }
}


#[derive(Debug)]
pub struct UserIntercept{
    user: SessionUser,
}

impl SqlIntercept for UserIntercept{
    /// do intercept sql/args
    /// is_prepared_sql: if is run in prepared_sql=ture
    fn do_intercept(&self, rb: &Rbatis, sql: &mut String, args: &mut Vec<rbs::Value>, is_prepared_sql: bool) -> Result<(), rbatis::Error>{
        println!("sql=>{}",sql);
        println!("args=>{:?}",args);
        // args.push(to_value!(self.userId));
        sql.push_str(&format!(" and tenant_id = {} ", self.user.tenant_id));
        println!("args=>{:?}",args);
        println!("[LogicDeletePlugin] after=> {}", sql);
        Ok(())
    }
}
