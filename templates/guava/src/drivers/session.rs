use async_session::{ serde_json, Result, Session, MemoryStore };
use redis::{AsyncCommands};

use super::redis::{init_redis, Redis};

{{#if redis_session}}
pub type GuavaSessionStore = RedisSessionStore;
{{else}}
pub type GuavaSessionStore = MemoryStore;
{{/if}}

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