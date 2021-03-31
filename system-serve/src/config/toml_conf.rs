use super::*;
use mongodb::{Client, Collection};
use std::env;
use std::fs::File;
use std::io::prelude::*;
// use yn_util::db;

/// 获取toml相关配置
macro_rules! get_setting_from_toml {
    ($struct: ident) => {{
        let result = $struct::default();
        let current_dir = if let Ok(v) = env::current_dir() {
            v
        } else {
            return result;
        };
        let current_path = if let Some(v) = current_dir.to_str() {
            v
        } else {
            return result;
        };
        let toml_file = format!("{}/setting.toml", current_path);
        match File::open(&toml_file) {
            Ok(mut v) => {
                let mut content = String::new();
                if let Ok(_) = v.read_to_string(&mut content) {
                    if let Ok(t) = toml::from_str::<$struct>(&content) {
                        // info!("t = {:#?}", t);
                        t
                    } else {
                        result
                    }
                } else {
                    result
                }
            }
            Err(err) => {
                println!("读取文件失败: {}", err);
                result
            }
        }
    }};
}

#[derive(Deserialize, Default, Debug)]
pub struct App {
    /// 应用ip
    pub host: String,
    /// 应用端口
    pub port: usize,
    /// 线程
    pub workers: usize,
    /// token secret_key
    pub secret_key: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct DataBase {
    /// 数据库ip
    pub host: String,
    /// 数据库用户名
    pub user: String,
    /// 数据库密码
    pub password: String,
    /// 数据库端口
    pub port: usize,
}

/// 缓存 redis
#[derive(Deserialize, Debug, Default)]
pub struct CachesRedis {
    /// ip
    pub ip: Vec<String>,
}

/// 缓存数据库
#[derive(Deserialize, Debug, Default)]
pub struct Caches {
    /// redis
    pub redis: CachesRedis,
}

#[derive(Deserialize, Debug, Default)]
pub struct Setting {
    /// 应用基础配置
    pub app: App,
    /// 数据库基础配置
    pub database: DataBase,
    /// 缓存数据库
    pub caches: Caches,
}

lazy_static! {
    #[derive(Deserialize,Debug,Clone,Copy)]
    #[macro_export]
    pub(crate) static ref SETTING: Setting = get_setting_from_toml!(Setting);
}

/// 初始化缓存服务器 redis
#[allow(dead_code)]
pub fn init_caches() {
    // 初始化 redis 服务器
    let setting = &*SETTING;
    let ip = &setting.caches.redis.ip;
    yn_util::caches::init_connections(&ip[0]);
}

/// 获取redis连接
#[allow(dead_code)]
pub fn redis_client()  -> redis::Connection {
    yn_util::caches::get_conn()
}

/// 获取数据库连接地址
#[allow(dead_code)]
pub fn get_conn_string() -> String {
    let setting = &*SETTING;
    let db = &setting.database;
    format!(
        "mongodb://{}:{}@{}:{}",
        db.user, db.password, db.host, db.port
    )
}

#[allow(dead_code)]
pub async fn get_db_client() -> Client {
    Client::with_uri_str(get_conn_string().as_str())
        .await
        .unwrap()
}

#[allow(dead_code)]
pub async fn get_db_collection(name: &str) -> Collection {
    get_db_client()
        .await
        .database(user::UserModel::DATA_BASE_NAME)
        .collection(name)
}

/// 设置日志打印
pub fn init_logger() {
    use chrono::Local;

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
}
