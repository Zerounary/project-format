#[derive(Debug, Default)]
pub struct InsertBatchResult {
    pub rows_affected: u64,
    pub insert_ids: Vec<i64>,
}

macro_rules! impl_repo_update {
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql_where:expr}) => {
        impl Repository {
            pub async fn $fn_name(
                &self,
                rb: &mut dyn rbatis::executor::Executor,
                table: &$table,
                $($param_key:$param_type,)*
            ) -> Result<(), rbatis::rbdc::Error> {
                if $sql_where.is_empty(){
                    return Err(rbatis::rbdc::Error::from("sql_where can't be empty!"));
                }
                #[rbatis::py_sql("`update ${table_name} set  `
                                trim ',':
                                  for k,v in table:
                                    if k == 'id' || v== null:
                                        continue:
                                    `${k}=#{v},`
                                ` `",$sql_where)]
                  async fn $fn_name(
                      rb: &mut dyn rbatis::executor::Executor,
                      table_name: String,
                      table: &rbs::Value,
                      $($param_key:$param_type,)*
                  ) -> Result<rbatis::rbdc::db::ExecResult, rbatis::rbdc::Error> {
                      impled!()
                  }
                  let table_name = crate::macros::repository::to_sql_table_name(stringify!($table));
                  let table = rbs::to_value!(table);
                  let result = $fn_name(rb, table_name, &table, $($param_key,)*).await;
                match result {
                    Ok(_result) => {
                        if _result.rows_affected > 0 {
                            Ok(())
                        } else {
                            Err(rbatis::Error::E("更新失败，没有对应条件的数据".to_string()))
                        }
                    },
                    Err(e) => {
                        log::error!("{:}", e);
                        Err(rbatis::Error::E(e.to_string()))
                    } 
                }
            }
        }
    };
}



macro_rules! impl_repo_insert {
    ($table:ty, $insert_fn:ident, $insert_batch_fn:ident) => {
        impl_repo_insert!(
            $table,
            $insert_fn,
            $insert_batch_fn,
            crate::macros::repository::to_sql_table_name(stringify!($table))
        );
    };
    ($table:ty, $insert_fn:ident, $insert_batch_fn:ident, $table_name:expr) => {
        impl Repository {
            pub async fn $insert_batch_fn(
                &self,
                mut rb: &mut dyn rbatis::executor::Executor,
                tables: &mut [$table],
                batch_size: u64,
            ) -> Result<crate::macros::repository::InsertBatchResult, rbatis::rbdc::Error> {
                #[rbatis::py_sql(
                    "`insert into ${table_name} (`
            trim ',':
              for k,v in tables[0]:
                if k == 'id':
                  continue:
                ${k},
            `) VALUES `
            trim ',':
              for _,table in tables:
                (
                  trim ',':
                    for k,v in table:
                      if k == 'id':
                        continue:
                      #{v},
                ),
            "
                )]
                async fn insert_batch(
                    rb: &mut dyn rbatis::executor::Executor,
                    tables: &[$table],
                    table_name: &str,
                ) -> Result<rbatis::rbdc::db::ExecResult, rbatis::rbdc::Error> {
                    impled!()
                }
                if tables.is_empty() {
                    return Err(rbatis::rbdc::Error::from(
                        "insert can not insert empty array tables!",
                    ));
                }
                let mut insert_ids = Vec::new();
                let table_name = $table_name.to_string();
                let mut result = rbatis::rbdc::db::ExecResult {
                    rows_affected: 0,
                    last_insert_id: rbs::Value::Null,
                };
                let ranges = rbatis::sql::Page::<()>::into_ranges(tables.len() as u64, batch_size);
                for (offset, limit) in ranges {
                    let exec_result = insert_batch(
                        rb,
                        &tables[offset as usize..limit as usize],
                        table_name.as_str(),
                    )
                    .await?;
                    for i in 0..exec_result.rows_affected {
                        if let rbs::Value::U64(id) = exec_result.last_insert_id {
                            insert_ids.push((id + i) as i64);
                        }
                    }
                    result.rows_affected += exec_result.rows_affected;
                    result.last_insert_id = exec_result.last_insert_id;
                }

                Ok(crate::macros::repository::InsertBatchResult {
                    rows_affected: result.rows_affected,
                    insert_ids,
                })
            }
        }

        impl Repository {
            pub async fn $insert_fn(&self, rb: &mut dyn rbatis::executor::Executor, table: $table) -> Result<i64, rbatis::Error> {
                let result = self.$insert_batch_fn(rb, &mut [table.clone()], 1).await;
                match result {
                    Ok(insert_result) => {
                        let id = insert_result.insert_ids[0];
                        Ok(id)
                    }
                    Err(e) => Err(rbatis::Error::E(e.to_string())),
                }
            }
        }
    };
}

// TODO 控制列表查询的最大数量
// TODO 不使用 * 作为查询字段
// TODO 添加翻页
// TODO 使用 ident 拼接自动批量生成 基础方法

macro_rules! impl_repo_select {
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $container:tt => $sql:expr}) => {
        impl Repository{
            pub async fn $fn_name(&self, rb: &mut dyn rbatis::executor::Executor, $($param_key:$param_type,)*)->Result<$container<$table>, rbatis::rbdc::Error>{
                #[rbatis::py_sql("`select ${table_column} from ${table_name} t `",$sql)]
                async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> Result<$container<$table>,rbatis::rbdc::Error> {impled!()}
                let mut table_column = "*".to_string();
                let mut table_name = crate::macros::repository::to_sql_table_name(stringify!($table));
                $fn_name(&mut rb,&table_column,&table_name,$($param_key ,)*).await
            }
        }
    };
}

macro_rules! impl_repo_select_one {
    ($table:ty{$fn_name:ident}) => {
        impl Repository{
            pub async fn $fn_name(&self, rb: &mut dyn rbatis::executor::Executor, id: i64)->Result<$table, rbatis::Error>{
                #[rbatis::py_sql("`select ${table_column} from ${table_name} t `", "`where id = #{id}`")]
                async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str, id: i64) -> Result<Option<$table>,rbatis::rbdc::Error> {impled!()}
                let table_column = "*".to_string();
                let table_name = crate::macros::repository::to_sql_table_name(stringify!($table));
                let result = $fn_name(rb,&table_column,&table_name, id).await;
                match result {
                    Ok(bo_option) => {
                        match bo_option {
                            Some(bo) => Ok(bo.to_owned()),
                            None => Err(rbatis::Error::E("Not Found!".to_string())),
                        }
                    }
                    Err(e) => {
                        log::error!("{:}", e);
                        Err(rbatis::Error::E(e.to_string()))
                    } 
                }
            }
        }
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql:expr}) => {
        impl Repository{
            pub async fn $fn_name(&self, rb: &mut dyn rbatis::executor::Executor, $($param_key:$param_type,)*)->Result<$table, rbatis::Error>{
                #[rbatis::py_sql("`select ${table_column} from ${table_name} t `", $sql)]
                async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> Result<Option<$table>,rbatis::rbdc::Error> {impled!()}
                let table_column = "*".to_string();
                let table_name = crate::macros::repository::to_sql_table_name(stringify!($table));
                let result = $fn_name(&mut rb,&table_column,&table_name,$($param_key ,)*).await;
                match result {
                    Ok(bo_option) => {
                        match bo_option {
                            Some(bo) => Ok(bo.to_owned()),
                            None => Err(rbatis::Error::E("Not Found!".to_string())),
                        }
                    }
                    Err(e) => {
                        log::error!("{:}", e);
                        Err(rbatis::Error::E(e.to_string()))
                    } 
                }
            }
        }
    };
    ($table:ty => $table_column:expr => {$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql:expr}) => {
        impl Repository{
            pub async fn $fn_name(&self, rb: &mut dyn rbatis::executor::Executor, $($param_key:$param_type,)*)->Result<$table, rbatis::Error>{
                #[rbatis::py_sql("`select ${table_column} from ${table_name} t `", $sql)]
                async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> Result<Option<$table>,rbatis::rbdc::Error> {impled!()}
                let table_column = $table_column.to_string();
                let table_name = crate::macros::repository::to_sql_table_name(stringify!($table));
                let result = $fn_name(rb,&table_column,&table_name,$($param_key ,)*).await;
                match result {
                    Ok(bo_option) => {
                        match bo_option {
                            Some(bo) => Ok(bo.to_owned()),
                            None => Err(rbatis::Error::E("Not Found!".to_string())),
                        }
                    }
                    Err(e) => {
                        log::error!("{:}", e);
                        Err(rbatis::Error::E(e.to_string()))
                    } 
                }
            }
        }
    };
    ($table:ty => $table_column:expr => $result:ty {$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql:expr}) => {
        impl Repository{
            pub async fn $fn_name(&self, rb: &mut dyn rbatis::executor::Executor, $($param_key:$param_type,)*)->Result<$result, rbatis::Error>{
                #[rbatis::py_sql("`select ${table_column} from ${table_name} t `", $sql)]
                async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> Result<Option<$result>,rbatis::rbdc::Error> {impled!()}
                let table_column = $table_column.to_string();
                let table_name = crate::macros::repository::to_sql_table_name(stringify!($table));
                let result = $fn_name(rb,&table_column,&table_name,$($param_key ,)*).await;
                match result {
                    Ok(bo_option) => {
                        match bo_option {
                            Some(bo) => Ok(bo.to_owned()),
                            None => Err(rbatis::Error::E("Not Found!".to_string())),
                        }
                    }
                    Err(e) => {
                        log::error!("{:}", e);
                        Err(rbatis::Error::E(e.to_string()))
                    } 
                }
            }
        }
    };
}

macro_rules! impl_repo_select_list {
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql:expr}) => {
        impl Repository{
            pub async fn $fn_name(&self, rb: &mut dyn rbatis::executor::Executor, $($param_key:$param_type,)*)->Result<Vec<$table>, rbatis::Error>{
                #[rbatis::py_sql("`select ${table_column} from ${table_name} t `", $sql)]
                async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> Result<Vec<$table>,rbatis::rbdc::Error> {impled!()}
                let table_column = "t.*".to_string();
                let table_name = crate::macros::repository::to_sql_table_name(stringify!($table));
                let result = $fn_name(rb,&table_column,&table_name,$($param_key ,)*).await;
                match result {
                    Ok(bo_vec) => Ok(bo_vec),
                    Err(e) => {
                        log::error!("{:}", e);
                        Err(rbatis::Error::E(e.to_string()))
                    } 
                }
            }
        }
    };
    ($table:ty => $table_column:expr => {$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql:expr}) => {
        impl Repository{
            pub async fn $fn_name(&self, rb: &mut dyn rbatis::executor::Executor, $($param_key:$param_type,)*)->Result<Vec<$table>, rbatis::Error>{
                #[rbatis::py_sql("`select ${table_column} from ${table_name} t `", $sql)]
                async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> Result<Vec<$table>,rbatis::rbdc::Error> {impled!()}
                let table_column = $table_column.to_string();
                let table_name = crate::macros::repository::to_sql_table_name(stringify!($table));
                let result = $fn_name(rb,&table_column,&table_name,$($param_key ,)*).await;
                match result {
                    Ok(bo_vec) => Ok(bo_vec),
                    Err(e) => {
                        log::error!("{:}", e);
                        Err(rbatis::Error::E(e.to_string()))
                    } 
                }
            }
        }
    };
}

macro_rules! impl_repo_select_page {
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql:expr}) => {
        impl Repository{
            pub async fn $fn_name(&self, rb: &mut dyn rbatis::executor::Executor, $($param_key:$param_type,)* page_num: i64, page_size: i64)->Result<Vec<$table>, rbatis::Error>{
                #[rbatis::py_sql("`select ${table_column} from ${table_name} t `", $sql)]
                async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)* page_start: i64, page_size: i64) -> Result<Vec<$table>,rbatis::rbdc::Error> {impled!()}
                let table_column = "*".to_string();
                let table_name = crate::macros::repository::to_sql_table_name(stringify!($table));
                let start = (page_num - 1) * page_size;
                let result = $fn_name(rb,&table_column,&table_name,$($param_key ,)* start, page_size).await;
                match result {
                    Ok(bo_vec) => Ok(bo_vec),
                    Err(e) => {
                        log::error!("{:}", e);
                        Err(rbatis::Error::E(e.to_string()))
                    } 
                }
            }
        }
    };
    ($table:ty => $table_column:expr => {$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql:expr}) => {
        impl Repository{
            pub async fn $fn_name(&self, rb: &mut dyn rbatis::executor::Executor, $($param_key:$param_type,)* page_num: i64, page_size: i64)->Result<Vec<$table>, rbatis::Error>{
                #[rbatis::py_sql("`select ${table_column} from ${table_name} t `", $sql)]
                async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)* page_start: i64, page_size: i64) -> Result<Vec<$table>,rbatis::rbdc::Error> {impled!()}
                let table_column = $table_column.to_string();
                let table_name = crate::macros::repository::to_sql_table_name(stringify!($table));
                let start = (page_num - 1) * page_size;
                let result = $fn_name(&mut rb,&table_column,&table_name,$($param_key ,)* start, page_size).await;
                match result {
                    Ok(bo_vec) => Ok(bo_vec),
                    Err(e) => {
                        log::error!("{:}", e);
                        Err(rbatis::Error::E(e.to_string()))
                    } 
                }
            }
        }
    };
}

macro_rules! impl_repo_delete {
    ($table:ty{$fn_name:ident}) => {
        impl Repository {
            pub async fn $fn_name(&self, pool: &mut dyn rbatis::executor::Executor, ids: Vec<i64>) -> Result<(), rbatis::Error> {
                if ids.is_empty() {
                    return Ok(());
                }
                let table_name = crate::macros::repository::to_sql_table_name(stringify!($table));
                let sql = format!(
                    "DELETE FROM {} where id in ({})",
                    table_name,
                    ids.iter().join(",")
                );
                pool.fetch(sql.as_str(), vec![]).await.unwrap();
                Ok(())
            }
        }
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql:expr}) => {
        impl Repository{
            pub async fn $fn_name(&self, rb: &mut dyn rbatis::executor::Executor, $($param_key:$param_type,)*)->Result<Vec<$table>, rbatis::Error>{
                #[rbatis::py_sql("`delete from ${table_name} `", $sql)]
                async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_name:&str,$($param_key:$param_type,)*) -> Result<Vec<$table>,rbatis::rbdc::Error> {impled!()}
                let table_name = crate::macros::repository::to_sql_table_name(stringify!($table));
                let result = $fn_name(rb,&table_name,$($param_key ,)*).await;
                match result {
                    Ok(bo_vec) => Ok(bo_vec),
                    Err(e) => {
                        log::error!("{:}", e);
                        Err(rbatis::Error::E(e.to_string()))
                    } 
                }
            }
        }
    };
}

pub(crate) use impl_repo_select;
pub(crate) use impl_repo_select_one;
pub(crate) use impl_repo_select_list;
pub(crate) use impl_repo_select_page;
pub(crate) use impl_repo_update;
pub(crate) use impl_repo_insert;
pub(crate) use impl_repo_delete;


use crate::drivers::db::get_db_type;

pub fn pure_name(bo_name: &str) -> String {
    let name = rbatis::utils::string_util::to_snake_name(&bo_name).trim_end_matches("_option_bo").trim_end_matches("_bo").to_string();
    name
}


pub fn to_sql_table_name(table_name: &str) -> String {
    let name = pure_name(table_name);
    let name = match name.as_str() {
{{#each tables }}
{{#if prefix}}
        "{{name}}" => String::from("{{prefix}}{{name}}"),
{{/if}}
{{/each}}
{{#if auth}}
        "session_user" => String::from("sys_user"),
{{/if}}
        _ => name,
    };
    match get_db_type() {
        crate::drivers::db::DB_TYPE::Mysql => format!(r#"`{}`"#, name),
        crate::drivers::db::DB_TYPE::Pg => format!(r#""{}""#, name),
        crate::drivers::db::DB_TYPE::Sqlite => format!(r#""{}""#, name),
    }
}