use std::{sync::{Arc}, collections::HashMap, path::Path};
use async_lock::RwLock;

use async_session::{ serde_json, Result, Session, SessionStore, async_trait, };
use easy_file::{read_file, create_write_file};
use itertools::Itertools;
use redis::{AsyncCommands};
use sled::Db;

use super::redis::{init_redis, Redis};

{{#if redis_session}}
pub type GuavaSessionStore = RedisSessionStore;
{{else}}
pub type GuavaSessionStore = LocalSessionStore;
{{/if}}

#[derive(Debug, Clone)]
pub struct LocalSessionStore {
    inner: Arc<RwLock<HashMap<String, Session>>>,
    session_file_name: String,
    tree: Db
}

impl LocalSessionStore {
    pub fn new() -> Self {
        // 加载本地数据到 storage中
        #[cfg(not(unix))]
        let session_file_name = String::from("./guava-session");
        #[cfg(unix)]
        let session_file_name = String::from("/dev/shm/guava-session");

        let tree = sled::open(session_file_name.clone()).expect("open");

        let mut sessions: HashMap<String, Option<Session>> = tree.iter().map(|pair| {
            match pair  {
                Ok((key, val)) => {
                    let key = key.to_vec();
                    let data = val.to_vec();
                    let session: Session = serde_json::from_slice(&data).unwrap();
                    if session.is_expired() {
                        (String::from_utf8(key).unwrap(), None)
                    }else {
                        (String::from_utf8(key).unwrap(), Some(session))
                    }
                },
                _ => ("".to_string(), None)
            }
        }).collect();

        sessions.drain_filter(|_, v| v.is_none());

        let mut map: HashMap<String, Session> = HashMap::new();

        for (k, v) in sessions {
            if let Some(s) = v {
                map.insert(k, s);
            }
        }

        let inner: Arc<RwLock<HashMap<String, Session>>> = Arc::new(RwLock::new(map));
        Self {
            inner,
            session_file_name,
            tree
        }
    }

    pub async fn save_to_local(&self, session_id: String, session: Session) {
        match self.tree.insert(session_id.as_bytes(), serde_json::to_vec(&session).unwrap()) {
            Ok(_r) => {log::info!("Session保存成功")},
            Err(e) => {
                log::error!("Session保存失败, {}", e.to_string());
            },
        }
    }
    pub async fn remove_from_local(&self, session_id: String) {
        match self.tree.remove(session_id.as_bytes()) {
            Ok(_r) => {log::info!("Session保存成功")},
            Err(e) => {
                log::error!("Session保存失败, {}", e.to_string());
            },
        }
    }
}

#[async_trait]
impl SessionStore for LocalSessionStore {
    async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
        let id = Session::id_from_cookie_value(&cookie_value)?;
        log::trace!("loading session by id `{}`", id);
        Ok(self
            .inner
            .read()
            .await
            .get(&id)
            .cloned()
            .and_then(Session::validate))
    }

    async fn store_session(&self, session: Session) -> Result<Option<String>> {
        log::trace!("storing session by id `{}`", session.id());
        self.inner
            .write()
            .await
            .insert(session.id().to_string(), session.clone());
        session.reset_data_changed();
        self.save_to_local(session.id().to_string(), session.clone()).await;
        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: Session) -> Result {
        log::trace!("destroying session by id `{}`", session.id());
        self.inner.write().await.remove(session.id());
        self.remove_from_local(session.id().to_string()).await;
        Ok(())
    }

    async fn clear_store(&self) -> Result {
        log::trace!("clearing memory store");
        self.inner.write().await.clear();
        Ok(())
    }
}

#[derive(Clone)]
pub struct RedisSessionStore {
    client: Redis,
    prefix: Option<String>,
}

impl RedisSessionStore {
    pub fn new() -> Self {
        Self {
            client: init_redis(),
            prefix: Some("session_".to_string()),
        }
    }

    fn prefix_key(&self, key: impl AsRef<str>) -> String {
        if let Some(ref prefix) = self.prefix {
            format!("{}{}", prefix, key.as_ref())
        } else {
            key.as_ref().into()
        }
    }

    pub async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
        let id = Session::id_from_cookie_value(&cookie_value)?;
        let mut connection = self.client.get().await?;
        let record: Option<String> = connection.get(self.prefix_key(id)).await?;
        match record {
            Some(value) => Ok(serde_json::from_str(&value)?),
            None => Ok(None),
        }
    }

    pub async fn store_session(&self, session: Session) -> Result<Option<String>> {
        let id = self.prefix_key(session.id());
        let string = serde_json::to_string(&session)?;

        let mut connection = self.client.get().await?;

        match session.expires_in() {
            None => connection.set(id, string).await?,

            Some(expiry) => {
                connection
                    .set_ex(id, string, expiry.as_secs() as usize)
                    .await?
            }
        };

        Ok(session.into_cookie_value())
    }

    pub async fn destroy_session(&self, session: Session) -> Result {
        let mut connection = self.client.get().await?;
        let key = self.prefix_key(session.id().to_string());
        connection.del(key).await?;
        Ok(())
    }

    async fn ids(&self) -> Result<Vec<String>> {
        Ok(self.client.get().await?.keys(self.prefix_key("*")).await?)
    }

    pub async fn clear_store(&self) -> Result {
        let mut connection = self.client.get().await?;

        if self.prefix.is_none() {
            let _: () = redis::cmd("FLUSHDB").query_async(&mut connection).await?;
        } else {
            let ids = self.ids().await?;
            if !ids.is_empty() {
                connection.del(ids).await?;
            }
        }
        Ok(())
    }
}