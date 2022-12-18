use std::{collections::HashMap, sync::Arc};

use crate::{
    drivers::{
        cache::ServiceCache,
        db::{get_db_type, DB}, session::GuavaSessionStore,
    },
    server::api::commands::{Resp, resp_err},
    service::Service,
};
use async_session::{async_trait, Session, MemoryStore, SessionStore};
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

use super::api::commands::{AppResult, State, AppErrResult};

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
    pub tenant_id: i64,
}

pub async fn login(
    Query(params): Query<LoginParams>,
    Extension(service): State,
    Extension(store): Extension<GuavaSessionStore>,
) -> impl IntoResponse {
    let result = service
        .repo
        .select_session_user_by_name(&service.db, &params.username)
        .await;
    if let Ok(user) = result {
        if params.password.eq(&user.password) {
            let mut session = Session::new();
            session.insert("user_bo", user).unwrap();
            let cookie = store.store_session(session).await.unwrap().unwrap();
            let mut headers = HeaderMap::new();
            let cookie =
                HeaderValue::from_str(format!("{}={}", AXUM_SESSION_COOKIE_NAME, cookie).as_str())
                    .unwrap();
            headers.insert(http::header::SET_COOKIE, cookie);
            return (headers, Resp::ok(true));
        }
    }
    (HeaderMap::new(), Resp::ok(false))
}

pub async fn logout(
    Extension(store): Extension<GuavaSessionStore>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> AppResult<bool> {
    let session_cookie = cookie.get(AXUM_SESSION_COOKIE_NAME);
    if session_cookie.is_none() {
        return Resp::ok(false);
    }
    if let Some(session) = store
        .load_session(session_cookie.unwrap().to_owned())
        .await
        .unwrap()
    {
        store.destroy_session(session).await;
    }
    return Resp::ok(true);
}

pub async fn check_auth(
    GuavaSession::FoundUser((user, service2)): GuavaSession,
    Extension(service): Extension<Arc<Service>>,
) -> AppResult<bool> {
    // println!("{:?}", format!("{:?}", user));
    // let result = service.count_category().await?;
    Resp::ok(true)
}

#[async_trait]
impl<B> FromRequest<B> for GuavaSession
where
    B: Send,
{
    type Rejection = (StatusCode, AppErrResult);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(store) = Extension::<GuavaSessionStore>::from_request(req)
            .await
            .expect("`SessionStore` extension missing");

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
            return Err((StatusCode::OK, resp_err(401, "请先登录".to_string())));
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
                return Err((StatusCode::OK, resp_err(401, "No `user_bo` found in session".to_string())));
            }
        } else {
            log::debug!(
                "UserIdFromSession: err session not exists in store, {}={}",
                AXUM_SESSION_COOKIE_NAME,
                session_cookie.unwrap()
            );
            return Err((StatusCode::OK, resp_err(401, "No session found for cookie".to_string())));
        };

        let mut db = db.clone();
        db.set_sql_intercepts(vec![Box::new(UserIntercept {
            user: user_bo.clone(),
        })]);
        Ok(Self::FoundUser((
            user_bo.clone(),
            Service::new_scope(db, user_bo.clone(), cache),
        )))
    }
}

#[derive(Debug)]
pub struct UserIntercept {
    user: SessionUser,
}

use regex::Regex;

impl SqlIntercept for UserIntercept {
    /// do intercept sql/args
    /// is_prepared_sql: if is run in prepared_sql=ture
    fn do_intercept(
        &self,
        rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<rbs::Value>,
        is_prepared_sql: bool,
    ) -> Result<(), rbatis::Error> {
        let plugin_name = "[UserIntercept]: ";
        let upper_sql = sql.clone().to_uppercase().trim().to_string();
        let current_time = match get_db_type() {
            crate::drivers::db::DB_TYPE::Mysql => "now()",
            crate::drivers::db::DB_TYPE::Pg => "sysdate",
            crate::drivers::db::DB_TYPE::Sqlite => "datetime()",
        };
        if upper_sql.contains("SELECT") {
            let regex = match get_db_type() {
                crate::drivers::db::DB_TYPE::Mysql => Regex::new("`[a-zA-Z0-9_]+`").unwrap(),
                crate::drivers::db::DB_TYPE::Pg => Regex::new("\"[a-zA-Z0-9_]+\"").unwrap(),
                crate::drivers::db::DB_TYPE::Sqlite => Regex::new("\"[a-zA-Z0-9_]+\"").unwrap(),
            };
            if let Some(_captures) = regex.captures(&sql.clone()) {
                for (i, caps) in regex.captures_iter(&sql.clone()).enumerate() {
                    let table_name = caps[0].to_string();
                    let limit_table_name = format!(
                        "(select * from {} where tenant_id = {})",
                        table_name, self.user.tenant_id
                    );
                    *sql = sql.replace(&table_name, &limit_table_name);
                }
                Ok(())
            } else {
                log::error!("{}表名未使用符号包裹, sql => {} ", plugin_name, sql);
                Err(rbatis::Error::E(format!(
                    "{}表名未使用符号包裹",
                    plugin_name
                )))
            }
        } else if upper_sql.contains("DELETE FROM") {
            if upper_sql.contains("WHERE") {
                sql.push_str(&format!(" and tenant_id = {}", self.user.tenant_id));
                Ok(())
            } else {
                Err(rbatis::Error::E(format!(
                    "{}删除时不可没有条件",
                    plugin_name
                )))
            }
        } else if upper_sql.contains("UPDATE") {
            if upper_sql.contains("WHERE") {
                let set_end_pos = upper_sql.find("WHERE").unwrap_or(0);
                sql.insert_str(set_end_pos, &format!( ", updated={},updated_time={}", self.user.id, current_time));
                sql.push_str(&format!(" and tenant_id = {}", self.user.tenant_id));
                Ok(())
            } else {
                Err(rbatis::Error::E(format!(
                    "{}更新时不可没有条件",
                    plugin_name
                )))
            }
        } else if upper_sql.contains("INSERT INTO") {
            let regex = Regex::new(r"\(.+?\)").unwrap();
            if let Some(_captures) = regex.captures(&sql.clone()) {
                for (i, caps) in regex.captures_iter(&sql.clone()).enumerate() {
                    let matched_str = caps[0].to_string();
                    let matched_content = matched_str
                        .clone()
                        .trim_start_matches("(")
                        .trim_end_matches(")")
                        .to_string();
                    let result_str = if i == 0 {
                        format!("({},tenant_id,created,created_time,updated,updated_time)", matched_content)
                    } else {
                        format!("({},{},{},{},{},{})", matched_content, self.user.tenant_id, self.user.id, current_time, self.user.id, current_time)
                    };
                    *sql = sql.replace(&matched_str, &result_str);
                }
                Ok(())
            } else {
                log::error!("{}不能识别的新增语句, sql => {} ", plugin_name, sql);
                Err(rbatis::Error::E(format!(
                    "{}不能识别的新增语句",
                    plugin_name
                )))
            }
        } else {
            log::error!("{}未知SQL, sql => {} ", plugin_name, sql);
            Err(rbatis::Error::E(format!("{}未知SQL", plugin_name)))
        }
    }
}
