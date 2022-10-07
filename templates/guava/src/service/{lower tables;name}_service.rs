// use cached::proc_macro::cached;
use serde::Deserialize;

use crate::{
    drivers::{cache::ServiceResult}, cache_value, cache,
    entities::{{lower this.name}}_bo::{{upperCamel this.name}}BO,
    entities::{{lower this.name}}_opt_bo::{{upperCamel this.name}}OptionBO,
};

use super::Service;

// 业务错误
#[derive(Debug, Clone)]
pub enum {{upperCamel this.name}}RepoError {
    #[allow(dead_code)]
    NotFound,
}

#[derive(Debug, Default, Deserialize)]
#[allow(dead_code)]
pub struct Create{{upperCamel this.name}}Input {
    {{#each columns}}
    {{#unless (isId name) }}
    pub {{name}}: {{type}},
    {{/unless}}
    {{/each}}
}

#[derive(Debug, Default, Deserialize)]
pub struct Update{{upperCamel this.name}}Input {
    {{#each columns}}
    pub {{name}}: {{type}},
    {{/each}}
}

impl Service {

    pub async fn find_{{lower this.name}}_list(&self, bo: {{upperCamel this.name}}OptionBO) -> Result<Vec<{{upperCamel this.name}}BO>, {{upperCamel this.name}}RepoError> {
        let result = self.repo.select_{{lower this.name}}_list(&self.db, bo).await;
        match result {
            Ok(result_list) => Ok(result_list),
            Err(_e) => Err({{upperCamel this.name}}RepoError::NotFound)
        }
    }

    pub async fn find_{{lower this.name}}_by_id_no_cache(&self, id: i64) -> Result<{{upperCamel this.name}}BO, {{upperCamel this.name}}RepoError> {
        let result = self.repo.select_{{lower this.name}}_by_id(&self.db, id).await;
        match result {
            Ok(bo) => Ok(bo),
            Err(_e) => Err({{upperCamel this.name}}RepoError::NotFound),
        }
    }
    
    pub async fn find_{{lower this.name}}_by_id(&self, id: i64) -> Result<{{upperCamel this.name}}BO, {{upperCamel this.name}}RepoError> {
        cache!{
            self(id) -> Result<{{upperCamel this.name}}BO, {{upperCamel this.name}}RepoError> {
                let result = self.repo.select_{{lower this.name}}_by_id(&self.db, id).await;
                match result {
                    Ok(bo) => Ok(bo),
                    Err(_e) => Err({{upperCamel this.name}}RepoError::NotFound),
                }
            }
        }
    }

    pub async fn create_{{lower this.name}}(&self, input: Create{{upperCamel this.name}}Input) -> Result<{{upperCamel this.name}}BO, {{upperCamel this.name}}RepoError> {
        let bo = {{upperCamel this.name}}BO {
            {{#each columns}}
            {{#unless (isId name) }}
            {{name}}: input.{{name}},
            {{/unless}}
            {{/each}}
            ..{{upperCamel this.name}}BO::default()
        };
        let result = self.repo.create_{{lower this.name}}(&self.db, bo).await;

        match result {
            Ok(id) => self.find_{{lower this.name}}_by_id(id).await,
            Err(e) => {
                dbg!(e);
                Err({{upperCamel this.name}}RepoError::NotFound)
            }
        }
    }

    pub async fn create_{{lower this.name}}_batch(&self, mut input: Vec<Create{{upperCamel this.name}}Input>) -> Result<Vec<i64>, {{upperCamel this.name}}RepoError> {
        let mut bos = input.iter_mut().map(|e| {
            {{upperCamel this.name}}BO {
            {{#each columns}}
            {{#unless (isId name) }}
                {{name}}: e.{{name}}.clone(),
            {{/unless}}
            {{/each}}
                ..{{upperCamel this.name}}BO::default()
            }
        }).collect::<Vec<{{upperCamel this.name}}BO>>();
        let result = self.repo.create_{{lower this.name}}_batch(&self.db, &mut bos, 100).await;

        match result {
            Ok(insert_result) => Ok(insert_result.insert_ids),
            Err(e) => {
                dbg!(e);
                Err({{upperCamel this.name}}RepoError::NotFound)
            }
        }
    }

    pub async fn update_{{lower this.name}}(&self, input: Update{{upperCamel this.name}}Input) -> Result<{{upperCamel this.name}}BO, {{upperCamel this.name}}RepoError> {
        let bo = {{upperCamel this.name}}BO {
            {{#each columns}}
            {{name}}: input.{{name}}.clone(),
            {{/each}}
            ..{{upperCamel this.name}}BO::default()
        };
        let result = self.repo.update_{{lower this.name}}_by_id(&self.db, &bo, bo.id).await;

        match result {
            Ok(_) => {
                self.cache.invalidate(&input.id);
                self.find_{{lower this.name}}_by_id(input.id).await
            },
            Err(_e) => Err({{upperCamel this.name}}RepoError::NotFound),
        }
    }
    pub async fn delete_{{lower this.name}}(&self, id: i64) -> Result<(), {{upperCamel this.name}}RepoError> {
        let result = self.repo.delete_{{lower this.name}}(&self.db, id).await;

        match result {
            Ok(_) => {
                self.cache.invalidate(&id);
                Ok(())
            },
            Err(_e) => Err({{upperCamel this.name}}RepoError::NotFound),
        }
    }
    pub async fn delete_{{lower this.name}}_ids(&self, ids: Vec<i64>) -> Result<(), {{upperCamel this.name}}RepoError> {
        let result = self.repo.delete_{{lower this.name}}_ids(&self.db, ids.clone()).await;

        match result {
            Ok(_) => {
                for id in ids {
                    self.cache.invalidate(&id);
                }
                Ok(())
            },
            Err(_e) => Err({{upperCamel this.name}}RepoError::NotFound),
        }
    }
}
