use futures_core::future::BoxFuture;
use rbdc::db::{ConnectOptions, Connection};
use rbdc::Error;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::connection::OracleConnection;

#[derive(Serialize, Deserialize, Debug)]
pub struct OracleConnectOptions {
    pub username: String,
    pub password: String,
    pub connect_string: String,
}

impl ConnectOptions for OracleConnectOptions {
    fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        Box::pin(async move {
            let v = OracleConnection::establish(self)
                .await
                .map_err(|e| Error::from(e.to_string()))?;
            Ok(Box::new(v) as Box<dyn Connection>)
        })
    }

    fn set_uri(&mut self, url: &str) -> Result<(), Error> {
        *self = OracleConnectOptions::from_str(url)?;
        Ok(())
    }

    fn uppercase_self(&self) -> &(dyn Any + Send + Sync) {
        self
    }
}

impl Default for OracleConnectOptions {
    fn default() -> Self {
        Self {
            username: "scott".to_owned(),
            password: "tiger".to_owned(),
            connect_string: "//localhost/XE".to_owned(),
        }
    }
}

impl OracleConnectOptions {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        let s = s.trim_start_matches("oracle://");
        let parts: Vec<&str> = s.split('@').collect();
        let auth_part = parts[0];
        let host_part = parts[1];

        let auth_parts: Vec<&str> = auth_part.split(':').collect();
        let username = auth_parts[0];
        let password = auth_parts[1];
        Ok(Self {
            username: username.to_string(),
            password: password.to_string(),
            connect_string: host_part.to_string(),
        })
    }
}