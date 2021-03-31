use super::*;
use actix_web::{post, HttpRequest};
use model::theme::*;

#[post("save")]
pub async fn save(query: web::Json<ThemeModel>, req: HttpRequest, ctrl: CTRL) -> SimpleResp {
    let user = jwt_verify_to_id!(req);
    let query = query.into_inner();
    info!("query = {:?}", query);
    let mut d = ser::to_document(&query).unwrap();
    d.insert("user_id", ObjectId::with_string(&user.id.as_str()).unwrap());
    d.insert("user_name", user.name);
    d.insert(
        "create_by",
        ObjectId::with_string(&user.id.as_str()).unwrap(),
    );
    d.insert(
        "update_by",
        ObjectId::with_string(&user.id.as_str()).unwrap(),
    );
    let result = ctrl.theme_service.save(d).await;
    match result {
        Ok(d) => {
            let ids = vec!["user_id"];
            let data = document_handle_id(d, Some(ids));
            Resp::ok(Some(data), "主题新增成功", None, None, Some(1)).to_json_result()
        }
        Err(_) => {
            return Err(BusinessError::InternalError {
                source: anyhow!("主题新增失败"),
            });
        }
    }
}

#[post("find")]
pub async fn find_theme(req: HttpRequest, ctrl: CTRL) -> SimpleResp {
    let user = jwt_verify_to_id!(req);
    let mut d = doc! {};
    d.insert("user_id", ObjectId::with_string(user.id.as_str()).unwrap());
    info!("d = {:?}", d);
    let theme = ctrl.theme_service.find_one(d).await?;

    match theme {
        Some(d) => {
            let d = document_handle_id(d, Some(vec!["user_id"]));
            Resp::ok(Some(d), "用户主题查找成功", None, None, Some(1)).to_json_result()
        }
        None => Resp::err(404, "该用户暂无自定义主题").to_json_result(),
    }
}

#[post("update")]
pub async fn update(query: web::Json<ThemeModel>, req: HttpRequest, ctrl: CTRL) -> SimpleResp {
    jwt_verify_to_id!(req);
    let query = query.into_inner();
    let filter = ser::to_document(&query).unwrap();
    let result = ctrl.theme_service.update(filter).await;
    let result = match result {
        Ok(var) => var,
        Err(_) => {
            return Err(BusinessError::InternalError {
                source: anyhow!("修改失败"),
            })?
        }
    };
    match result {
        Some(result) => Resp::ok(Some(result), "修改成功", None, None, Some(1)).to_json_result(),
        None => Resp::err(400, "修改失败").to_json_result(),
    }
}
