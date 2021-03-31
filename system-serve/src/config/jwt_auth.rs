use super::*;
use toml_conf::SETTING;
use yn_util::date_time;
use yn_util::jwt;

/// token 验证 返回用户信息
/// 接收参数类型  HttpRequest
/// # Example
/// ```rust
/// #[post("find")]
/// pub async fn get_user_list(
///    query: web::Json<T>,
///    req: HttpRequest,
///    ctrl: web::Data<sync::Arc<T>>,
/// ){
///   let user = jwt_verify!(req,ctrl);
/// }
/// ```
#[macro_export]
macro_rules! jwt_verify {
    ($req:expr,$ctrl:ident) => {{
        let token = match $req.headers().get("cookie") {
            Some(h) => match h.to_str() {
                Ok(t) => match t.find("token") {
                    Some(start) => &t[(start + 6)..],
                    None => {
                        return Err(BusinessError::Unauthorized);
                    }
                },
                Err(_) => {
                    return Err(BusinessError::Unauthorized);
                }
            },
            None => {
                return Err(BusinessError::Unauthorized);
            }
        };
        let auth = yn_util::jwt::decode(token);
        let claims = match auth {
            Ok(user) => {
                let s = $ctrl
                    .user_service
                    .find_by_id(bson::oid::ObjectId::with_string(user.claims.id.as_str()).unwrap())
                    .await?;
                // s.unwrap()
                match s {
                    Some(v) => v,
                    None => {
                        return Err(BusinessError::Unauthorized);
                    }
                }
            }
            Err(_) => {
                return Err(BusinessError::Unauthorized);
            }
        };
        claims
    }};
}

/// token 验证 返回用户id
/// 接收参数类型  HttpRequest
/// # Example
/// ```rust
/// let req:HttpRequest = HttpRequest;
/// let token_info = jwt_verify_to_id!(req);
/// ```
#[macro_export]
macro_rules! jwt_verify_to_id {
    ($req:expr) => {{
        let token = match $req.headers().get("cookie") {
            Some(h) => match h.to_str() {
                Ok(t) => match t.find("token") {
                    Some(start) => &t[(start + 6)..],
                    None => {
                        return Err(BusinessError::Unauthorized);
                    }
                },
                Err(_) => {
                    return Err(BusinessError::Unauthorized);
                }
            },
            None => {
                return Err(BusinessError::Unauthorized);
            }
        };
        let auth = yn_util::jwt::decode(token);
        let claims = match auth {
            Ok(user) => user.claims,
            Err(_) => {
                return Err(BusinessError::Unauthorized);
            }
        };
        claims
    }};
}

/// 设置token时长
/// 默认 86400S
const DAY_ONE: u64 = 86400;

/// 设置token
#[inline]
#[allow(dead_code)]
pub fn set_token(id: String, name: String) -> String {
    let now = date_time::timestamp();
    let user = jwt::UserToken {
        exp: now + DAY_ONE,
        id,
        name,
    };
    jwt::encode_by(&user, &*SETTING.app.secret_key.as_str().as_bytes())
}

/// 校验token
#[inline]
#[allow(dead_code)]
pub fn verify_token(token: &str) -> yn_util::jwt::UserToken {
    let user =
        jwt::decode_by::<jwt::UserToken>(token, &*SETTING.app.secret_key.as_str().as_bytes());
    match user {
        Ok(v) => v.claims,
        Err(e) => {
            panic!("token 验证失败 {}", e);
        }
    }
}
