// use cached::proc_macro::cached;
use serde::Deserialize;
use smart_default::SmartDefault;
use rbatis::rbdc::{decimal::Decimal, date::Date};

use crate::{
    drivers::{cache::ServiceResult}, cache_value, cache, cache_invalidate,
    entities::{{snake this.name}}_bo::{{upperCamel this.name}}BO,
    entities::{{snake this.name}}_opt_bo::{{upperCamel this.name}}OptionBO,
    server::error::AppError
};

use super::Service;


#[derive(Debug, SmartDefault, Deserialize)]
#[allow(dead_code)]
pub struct Create{{upperCamel this.name}}Input {
    {{#each columns}}
    {{#unless (isId name) }}
    {{#if skip.[4]}}
    pub {{name}}: Option<{{type}}>,
    {{else}}
    {{#if default}}
    #[default(_code = "{{default}}")]
    {{/if}}
    pub {{name}}: {{type}},
    {{/if}}
    {{/unless}}
    {{/each}}
}

#[derive(Debug, SmartDefault, Deserialize)]
pub struct Update{{upperCamel this.name}}Input {
    {{#each columns}}
    {{#if default}}
    #[default(_code = "{{default}}")]
    {{/if}}
    pub {{name}}: {{type}},
    {{/each}}
}

#[derive(Debug, SmartDefault, Deserialize)]
pub struct Update{{upperCamel this.name}}OptionInput {
    {{#each columns}}
    {{#if (isId name) }}
    pub {{name}}: {{type}},
    {{/if}}
    {{#unless (isId name) }}
    pub {{name}}: Option<{{type}}>,
    {{/unless}}
    {{/each}}
}

impl Service {

    pub async fn find_{{snake this.name}}_list(&self, bo: {{upperCamel this.name}}OptionBO) -> Result<Vec<{{upperCamel this.name}}BO>, AppError> {
        let result = self.repo.select_{{snake this.name}}_list(&self.db, bo).await?;
        Ok(result)
    }

    pub async fn count_{{snake name}}(&self) -> Result<i64, AppError> {
        let count = self.repo.count_{{snake name}}(&self.db).await?;
        Ok(count)
    }

    pub async fn find_{{snake this.name}}_page(&self, bo: {{upperCamel this.name}}OptionBO, page_num: i64, page_size: i64) -> Result<Vec<{{upperCamel this.name}}BO>, AppError> {
        let result = self.repo.select_{{snake this.name}}_page(&self.db, bo, page_num, page_size).await?;
        Ok(result)
    }
    pub async fn find_{{snake this.name}}_by_id_no_cache(&self, id: i64) -> Result<{{upperCamel this.name}}BO, AppError> {
        let result = self.repo.select_{{snake this.name}}_by_id(&self.db, id).await?;
        Ok(result)
    }
    
    pub async fn find_{{snake this.name}}_by_id(&self, id: i64) -> Result<{{upperCamel this.name}}BO, AppError> {
        cache!{
            self(id) -> Result<{{upperCamel this.name}}BO, AppError> {
                let result = self.repo.select_{{snake this.name}}_by_id(&self.db, id).await?;
                Ok(result)
            }
        }
    }

    pub async fn create_{{snake this.name}}(&self, input: Create{{upperCamel this.name}}Input) -> Result<i64, AppError> {
        let bo = {{upperCamel this.name}}BO {
            {{#each columns}}
            {{#unless (isId name) }}
            {{#if skip.[4]}}
                {{#if default}}
                {{name}}: input.{{name}}.clone().unwrap_or({{default}}),
                {{else}}
                {{name}}: input.{{name}}.clone().unwrap_or_default(),
                {{/if}}
            {{else}}
            {{name}}: input.{{name}},
            {{/if}}
            {{/unless}}
            {{/each}}
            ..{{upperCamel this.name}}BO::default()
        };
        {{#if ac}}
        let mut conn = self.db.acquire_begin().await?;
        let mut conn = conn.defer_async(|mut tx| async move {
            if !tx.done {
                tx.rollback().await.unwrap();
            }
        });
        {{else}}
        let mut conn = self.db.acquire().await?;
        {{/if}}
        let id = self.repo.create_{{snake this.name}}(&mut conn, bo).await?;

        {{#if ac}}
        conn.commit().await?;
        {{/if}}
        Ok(id)
    }

    pub async fn create_{{snake this.name}}_batch(&self, mut input: Vec<Create{{upperCamel this.name}}Input>) -> Result<Vec<i64>, AppError> {
        let mut bos = input.iter_mut().map(|e| {
            {{upperCamel this.name}}BO {
            {{#each columns}}
            {{#unless (isId name) }}
            {{#if skip.[4]}}
                {{#if default}}
                {{name}}: e.{{name}}.clone().unwrap_or({{default}}),
                {{else}}
                {{name}}: e.{{name}}.clone().unwrap_or_default(),
                {{/if}}
            {{else}}
                {{name}}: e.{{name}}.clone(),
            {{/if}}
            {{/unless}}
            {{/each}}
                ..{{upperCamel this.name}}BO::default()
            }
        }).collect::<Vec<{{upperCamel this.name}}BO>>();
        {{#if ac}}
        let mut conn = self.db.acquire_begin().await?;
        let mut conn = conn.defer_async(|mut tx| async move {
            if !tx.done {
                tx.rollback().await.unwrap();
            }
        });
        {{else}}
        let mut conn = self.db.acquire().await?;
        {{/if}}
        let insert_result = self.repo.create_{{snake this.name}}_batch(&mut conn, &mut bos, 100).await?;

        {{#if ac}}
        conn.commit().await?;
        {{/if}}
        Ok(insert_result.insert_ids)
    }

    pub async fn update_{{snake this.name}}(&self, input: Update{{upperCamel this.name}}Input) -> Result<(), AppError> {
        let bo = {{upperCamel this.name}}BO {
            {{#each columns}}
            {{name}}: input.{{name}}.clone(),
            {{/each}}
            ..{{upperCamel this.name}}BO::default()
        };
        {{#if am}}
        let mut conn = self.db.acquire_begin().await?;
        let mut conn = conn.defer_async(|mut tx| async move {
            if !tx.done {
                tx.rollback().await.unwrap();
            }
        });
        {{else}}
        let mut conn = self.db.acquire().await?;
        {{/if}}
        let result = self.repo.update_{{snake this.name}}_by_id(&mut conn, &bo, bo.id).await;

        {{#if am}}
        conn.commit().await?;
        {{/if}}
        match result {
            Ok(_) => {
                cache_invalidate!(self(&input.id => {{upperCamel name}}));
                Ok(())
            },
            Err(e) => Err(e.into()),
        }
    }
    pub async fn update_{{snake this.name}}_opt(&self, input: Update{{upperCamel this.name}}OptionInput) -> Result<(), AppError> {
        let bo = {{upperCamel this.name}}OptionBO {
            {{#each columns}}
            {{#if (isId name) }}
            {{name}}: Some(input.{{name}}.clone()),
            {{/if}}
            {{#unless (isId name) }}
            {{name}}: input.{{name}}.clone(),
            {{/unless}}
            {{/each}}
            ..{{upperCamel this.name}}OptionBO::default()
        };
        {{#if am}}
        let mut conn = self.db.acquire_begin().await?;
        let mut conn = conn.defer_async(|mut tx| async move {
            if !tx.done {
                tx.rollback().await.unwrap();
            }
        });
        {{else}}
        let mut conn = self.db.acquire().await?;
        {{/if}}
        let result = self.repo.update_{{snake this.name}}_opt_by_id(&mut conn, &bo, input.id).await;

        {{#if am}}
        conn.commit().await?;
        {{/if}}

        match result {
            Ok(_) => {
                cache_invalidate!(self(&input.id => {{upperCamel name}}));
                Ok(())
            },
            Err(e) => Err(e.into()),
        }
    }
    pub async fn delete_{{snake this.name}}(&self, id: i64) -> Result<(), AppError> {
        {{#if bd}}
        let mut conn = self.db.acquire_begin().await?;
        let mut conn = conn.defer_async(|mut tx| async move {
            if !tx.done {
                tx.rollback().await.unwrap();
            }
        });
        {{else}}
        let mut conn = self.db.acquire().await?;
        {{/if}}

        let result = self.repo.delete_{{snake this.name}}(&mut conn, id).await;

        {{#if bd}}
        conn.commit().await?;
        {{/if}}
        match result {
            Ok(_) => {
                cache_invalidate!(self(&id => {{upperCamel name}}));
                Ok(())
            },
            Err(e) => Err(e.into()),
        }
    }
    pub async fn delete_{{snake this.name}}_ids(&self, ids: Vec<i64>) -> Result<(), AppError> {
        {{#if bd}}
        let mut conn = self.db.acquire_begin().await?;
        let mut conn = conn.defer_async(|mut tx| async move {
            if !tx.done {
                tx.rollback().await.unwrap();
            }
        });
        {{else}}
        let mut conn = self.db.acquire().await?;
        {{/if}}

        let result = self.repo.delete_{{snake this.name}}_ids(&mut conn, ids.clone()).await;

        {{#if bd}}
        conn.commit().await?;
        {{/if}}

        match result {
            Ok(_) => {
                for id in ids {
                    cache_invalidate!(self(&id => {{upperCamel name}}));
                }
                Ok(())
            },
            Err(e) => Err(e.into()),
        }
    }
}
