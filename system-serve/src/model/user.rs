//！ 用户模型

use super::*;

/// 用户查询传入的参数
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserModel {
    #[serde(default)]
    pub _id: Option<String>,

    /// 用户名
    #[serde(default)]
    pub username: Option<String>,

    /// 密码
    #[serde(default)]
    pub password: Option<String>,

    /// 真实姓名
    #[serde(default)]
    pub realname: Option<String>,

    /// 出生年
    #[serde(default)]
    pub birth_year: Option<i32>,

    /// 出生月
    #[serde(default)]
    pub birth_month: Option<i32>,

    /// 出生日
    #[serde(default)]
    pub birth_day: Option<i32>,

    /// 性别 0.女 1.男 3.保密
    #[serde(default)]
    pub sex: Option<i32>,

    /// 机构id
    #[serde(default)]
    pub org_id: Option<String>,

    /// 机构名称
    #[serde(default)]
    pub org_name: Option<String>,

    // /// 角色
    // #[serde(default)]
    // pub roles: Option<String>,
    /// 用户类型 1.管理员 2.普通用户 3.其他
    #[serde(default)]
    pub types: Option<i32>,

    /// 描述/备注
    #[serde(default)]
    pub desc: Option<String>,

    /// qq
    #[serde(default)]
    pub qq: Option<String>,

    /// 手机号码
    #[serde(default)]
    pub phone: Option<String>,

    /// 电子邮箱
    #[serde(default)]
    pub email: Option<String>,

    /// 用户状态 1.正常，2.禁用，3.注销
    #[serde(default)]
    pub enabled: Option<i32>,

    /// 头像id
    #[serde(default)]
    pub portrait: Option<ObjectId>,

    /// 积累登陆次数
    #[serde(default)]
    pub total_login_count: Option<i32>,

    /// 累计登录错误次数，超过最高次数，将限制登录
    #[serde(default)]
    pub total_login_failure: Option<i32>,

    /// 上次登陆ip
    #[serde(default)]
    pub last_login_ip: Option<String>,

    /// 上次登陆时间
    #[serde(default)]
    pub last_login_time: Option<String>,

    /// 创建时间
    #[serde(default)]
    pub create_time: Option<String>,

    /// 修改时间
    #[serde(default)]
    pub update_time: Option<String>,

    /// 修改人
    #[serde(default)]
    pub update_by: Option<String>,
}

impl Default for UserModel {
    fn default() -> Self {
        Self {
            _id: None,
            username: None,
            password: None,
            realname: None,
            birth_year: None,
            birth_month: None,
            birth_day: None,
            sex: None,
            org_id: None,
            org_name: None,
            // roles: None,
            types: None,
            desc: None,
            qq: None,
            phone: None,
            email: None,
            enabled: None,
            portrait: None,
            total_login_count: None,
            total_login_failure: None,
            last_login_ip: None,
            last_login_time: None,
            create_time: None,
            update_time: None,
            update_by: None,
        }
    }
}

impl UserModel {
    /// 数据库名字
    #[allow(dead_code)]
    pub const DATA_BASE_NAME: &'static str = "YNOS";

    /// 集合名字
    #[allow(dead_code)]
    pub const COLL_NAME: &'static str = "user_lists";
}
