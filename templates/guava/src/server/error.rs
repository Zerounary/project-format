use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use deadpool::managed::PoolError;
use rbatis::rbdc::Error;
use redis::RedisError;
use serde::{Serialize, Deserialize};

use super::api::commands::resp_err;

macro_rules! app_error_register {
    (
        $(
            $err:ident {
                $( $err_item:ident => $expr:expr ),+
                $(,)?
            }
        ),+
        $(,)?
    ) => {
        // 创建AppError枚举
        pub enum AppError {
            $(
                $err($err),
            )+
        }
        // 并自动实现业务错误到 AppError 的 from
        $(
            impl From<$err> for AppError {
                fn from(inner: $err) -> Self {
                    AppError::$err(inner)
                }
            }
        )+

        impl IntoResponse for AppError {
            fn into_response(self) -> Response {
                let (status, code, error_message) = match self {
                    // 匹配错误类型和对应的响应
                    $(
                        $(
                            AppError::$err($err::$err_item) => {
                                $expr
                            },
                        )+
                    )+
                    // RepoError(msg) => {
                    //     (StatusCode::OK, 1, msg)
                    // },
                    _ => {
                        (StatusCode::INTERNAL_SERVER_ERROR, 99999, "Unkown Server Error")
                    }
                };

                let body = resp_err(code, error_message.to_string());

                (status, body).into_response()
            }
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    RepoError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, error_message) = match self {
            AppError::RepoError(e) => (StatusCode::OK, 1, e),
            // 匹配错误类型和对应的响应
        };

        let body = resp_err(code, error_message.to_string());

        (status, body).into_response()
    }
}

impl From<Error> for AppError {
    fn from(e: Error) -> Self {
        match e {
            Error::E(e) => AppError::RepoError(e),
            Error::Io(e) => AppError::RepoError(e.to_string()),
        }
    }
}

impl From<PoolError<RedisError>> for AppError {
    fn from(e: PoolError<RedisError>) -> Self {
        AppError::RepoError(e.to_string())
    }
}

impl From<RedisError> for AppError {
    fn from(e: RedisError) -> Self {
        AppError::RepoError(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::RepoError(e.to_string())
    }
}