use super::*;
use bson::Document;
#[derive(Serialize, Deserialize, Debug)]
pub struct QueryBody<T> {
    // 请求条件
    #[serde(default)]
    pub body: Option<T>,

    // 第几页
    #[serde(default)]
    pub page: Option<i64>,

    // 一页限制条数
    #[serde(default)]
    pub limit: Option<i64>,

    // 根据字段排序
    #[serde(default)]
    pub sort_name: Option<String>,

    // 排序规律
    #[serde(default)]
    pub sort_order: Option<String>,
}

impl Default for QueryBody<Login> {
    fn default() -> Self {
        Self {
            body: None,
            page: Some(1),
            limit: Some(10),
            sort_name: None,
            sort_order: None,
        }
    }
}

/// 用户登陆响应信息
#[derive(Deserialize, Serialize, Debug)]
pub struct ResUserInfo {
    pub user_info: Option<Document>,
    pub token: Option<String>,
}

/// 登陆结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

/// 删除参数结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveList {
    pub ids: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthQuery<T> {
    /// 添加数据
    pub adds: Option<Vec<T>>,

    /// 删除数据
    pub deletes: Option<Vec<T>>,

    /// 产品id
    pub prod_id: Option<String>,

    /// 用户id
    pub user_id: Option<Vec<String>>,

    /// 角色id
    pub role_id: Option<Vec<String>>,
}
