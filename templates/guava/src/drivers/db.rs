use std::env;

use lazy_static::lazy_static;
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use rbdc_pg::driver::PgDriver;
use rbdc_pg::options::PgConnectOptions;
use rbdc_oracle::driver::OracleDriver;
use rbdc_sqlite::driver::SqliteDriver;
use url::Url;
use handlebars::Handlebars;
use jsonpath_rust::JsonPathQuery;
use serde_json::Value;

// alias DB pool type
pub type DB = Rbatis;
pub type DBOptions = PgConnectOptions;

lazy_static! {
        // connect database
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("No DATABASE_URL provided");

    pub static ref MAX_CONNECTIONS: String =
        env::var("MAX_CONNECTIONS").unwrap_or(String::from("200"));

    pub static ref MIGRATE_PATH: String =
        env::var("MIGRATE_PATH").unwrap_or(String::from("./migrations"));
}

pub enum DB_TYPE {
    Mysql,
    Pg,
    Sqlite,
    Oracle,
}

pub fn get_db_type() -> DB_TYPE {
    let parsed_db_url = Url::parse(&DATABASE_URL).ok();
    match parsed_db_url {
        Some(url) => match url.scheme() {
            "postgres" => DB_TYPE::Pg,
            "mysql" => DB_TYPE::Mysql,
            "sqlite" => DB_TYPE::Sqlite,
            "oracle" => DB_TYPE::Oracle,
            _ => panic!("unsupport database"),
        },
        None => {
            panic!("Incorrect database url")
        }
    }
}

// 自动选择用数据库驱动
pub fn init_db() -> Rbatis {
    let db = Rbatis::new();
    let oracle_client_lib_dir = env::var("ORACLE_CLIENT_LIB_DIR").map(format_dir).unwrap();
    env::set_var("PATH", oracle_client_lib_dir.clone());
    match get_db_type() {
        DB_TYPE::Pg => db.init(PgDriver {}, DATABASE_URL.as_str()).unwrap(),
        DB_TYPE::Mysql => db.init(MysqlDriver {}, DATABASE_URL.as_str()).unwrap(),
        DB_TYPE::Sqlite => db.init(SqliteDriver {}, DATABASE_URL.as_str()).unwrap(),
        DB_TYPE::Oracle => db.init(OracleDriver {}, DATABASE_URL.as_str()).unwrap(),
    };


    db
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub async fn migrate() {
    match get_db_type() {
        DB_TYPE::Mysql => {
            let pool = mysql::Pool::new(DATABASE_URL.as_str()).expect("Incorrect database url");
            let mut conn = pool.get_conn().expect("can't connect to mysql.");
            match embedded::migrations::runner().run(&mut conn) {
                Ok(_) => {}
                Err(e) => {
                    panic!("\nDatabase migrate Error: \n{:?}", e.kind());
                }
            }
        }
        DB_TYPE::Pg => {
            println!("Running DB migrations...");
            let (mut client, con) =
                tokio_postgres::connect(DATABASE_URL.as_str(), tokio_postgres::NoTls).await.unwrap();

            tokio::spawn(async move {
                if let Err(e) = con.await {
                    eprintln!("connection error: {}", e);
                }
            });
            let migration_report = embedded::migrations::runner()
                .run_async(&mut client)
                .await.unwrap();

            for migration in migration_report.applied_migrations() {
                println!(
                    "Migration Applied -  Name: {}, Version: {}",
                    migration.name(),
                    migration.version()
                );
            }

            println!("DB migrations finished!");
        }
        DB_TYPE::Oracle => {
            
        }
        DB_TYPE::Sqlite => {
            todo!()
        }
    };
}

pub fn get_sql(sql_name: &str, params: &Value) -> (String, Vec<rbs::Value>) {
    use regex::Regex;
    let regex = Regex::new("@\\s?+[a-zA-Z0-9_\\.]+\\s?+@").unwrap();
    let sql_template = match sql_name {
       "test" => include_str!("../../sql/test.sql").to_string(),
       _  => "".to_string()
    };
    let h = Handlebars::new();
    let mut sql = h.render_template(&sql_template, params).expect(&format!("SQL模板 {} 异常", sql_name));
    let mut vals: Vec<rbs::Value> = vec![];
    if let Some(_captures) = regex.captures(&sql.clone()) {
        for (i, caps) in regex.captures_iter(&sql.clone()).enumerate() {
            let matched_str = caps[0].to_string();
            sql = sql.replace(&matched_str, "?");
            let path = matched_str.trim_matches('@').trim();
            let val = params
                .clone()
                .path(&format!("$.{}", path))
                .expect(&format!("sql 参数 @{}@ 不存在", path))
                .as_array()
                .unwrap()
                .first()
                .expect(&format!("sql 参数 @{}@ 不存在", path))
                .clone();
            vals.push(serde_json2rbs(val));
        }
    }

    (sql, vals)
}


pub fn serde_json2rbs(val: Value) -> rbs::Value {
    type rv = rbs::Value;
    match val {
        Value::Bool(v) => rv::Bool(v),
        Value::Number(v) => rv::I64(v.as_i64().unwrap()),
        Value::String(v) => rv::String(v),
        Value::Array(v) => {
            let mut vec: Vec<rv> = vec![];
            for x in v {
                vec.push(serde_json2rbs(x));
            }
            rv::Array(vec)
        },
        Value::Object(_) => rv::Null,
        Value::Null => rv::Null,
    }
}


// 支持相对路径
fn format_dir(dir: String) -> String {
    if dir.starts_with(".") {
        let cur_dir = env::current_dir().unwrap();
        let abs_dir = format!(
            "{}{}",
            cur_dir.to_str().unwrap(),
            dir.trim_start_matches(".")
        );
        abs_dir
    } else {
        dir
    }
}