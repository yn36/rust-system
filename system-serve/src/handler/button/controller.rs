use super::*;
use actix_web::{delete, post, HttpRequest};
use model::{button::*, request::*};

#[post("save")]
pub async fn save(
    query: web::Json<ButtonModel>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    let user = jwt_verify_to_id!(req);
    let query = query.into_inner();
    if query.name.is_none() {
        return Resp::err(400, "缺少按钮名称 name 字段").to_json_result();
    } else if query.btn_code.is_none() {
        return Resp::err(400, "缺少按钮编码 btn_code 字段").to_json_result();
    } else if query.menu_id.is_none() {
        return Resp::err(400, "缺少菜单_id menu_id 字段").to_json_result();
    }

    // 菜单编码
    let menu_id = ObjectId::with_string(query.clone().menu_id.unwrap().as_str()).unwrap();
    // 判断编码是否已存在
    let d: Document = doc! {"btn_code":query.clone().btn_code.unwrap(),"menu_id":menu_id.clone()};
    match ctrl.button_service.find_one(d).await? {
        Some(_) => {
            // return Err(BusinessError::InternalError {
            //     source: anyhow!("该按钮编码已存在"),
            // })
            return Resp::err(400, "该按钮编码已存在").to_json_result();
        }
        None => {}
    };

    let mut d = ser::to_document(&query).unwrap();
    d.insert(
        "create_by",
        ObjectId::with_string(&user.id.as_str()).unwrap(),
    );
    d.insert(
        "update_by",
        ObjectId::with_string(&user.id.as_str()).unwrap(),
    );
    d.insert("menu_id", menu_id);

    let result = ctrl.button_service.save(d).await;
    match result {
        Ok(d) => Resp::ok(Some(d), "按钮新增成功", None, None, Some(1)).to_json_result(),
        Err(_) => {
            return Err(BusinessError::InternalError {
                source: anyhow!("按钮新增失败"),
            });
        }
    }
}

#[delete("delete")]
pub async fn delete(
    query: web::Json<RemoveList>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    jwt_verify_to_id!(req);
    let query = query.into_inner();
    let result = ctrl.button_service.remove(query.ids).await;
    match result {
        Ok(count) => Resp::ok(
            Some(count),
            format!("按钮删除成功{}条", count).as_str(),
            None,
            None,
            Some(count),
        )
        .to_json_result(),
        Err(e) => Resp::err(400, format!("按钮删除失败 {}", e).as_str()).to_json_result(),
    }
}

#[post("update")]
pub async fn update(
    query: web::Json<ButtonModel>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    let user = jwt_verify_to_id!(req);
    let mut query = query.into_inner();

    if query._id.is_none() {
        return Resp::err(400, "缺少 _id 字段").to_json_result();
    }

    if !query.btn_code.is_none() {
        // 判断编码是否已存在
        let d: Document = doc! {"btn_code":query.clone().btn_code.unwrap()};
        match ctrl.button_service.find_one(d).await? {
            Some(value) => {
                let oid = value.get("_id").unwrap().as_str().unwrap();
                if oid != query._id.clone().unwrap() {
                    return Err(BusinessError::InternalError {
                        source: anyhow!("该按钮编码已存在"),
                    });
                }
            }
            None => {}
        };
    }

    query.update_by = Some(user.id);

    let mut filter = ser::to_document(&query).unwrap();
    filter.remove("menu_id");

    let result = match ctrl.button_service.update(filter).await {
        Ok(res) => res,
        Err(e) => return Err(BusinessError::InternalError { source: anyhow!(e) })?,
    };

    match result {
        Some(result) => {
            Resp::ok(Some(result), "按钮修改成功", None, None, Some(1)).to_json_result()
        }
        None => Resp::err(400, "按钮修改失败").to_json_result(),
    }
}

#[post("find")]
pub async fn find(
    query: web::Json<QueryBody<ButtonModel>>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    jwt_verify_to_id!(req);
    let query = query.into_inner();
    // let filter = ser::to_document(&query.body).unwrap();
    let mut filter = doc! {};
    if !query.body.is_none() {
        filter = struct_to_document(&query.body).unwrap();
    }
    let result = ctrl
        .button_service
        .find(
            filter,
            query.page,
            query.limit,
            query.sort_name,
            query.sort_order,
            false,
            None,
        )
        .await;
    match result {
        Ok((list, total)) => Resp::ok(
            Some(list),
            "按钮查询成功",
            Some(query.page.unwrap_or(1)),
            Some(query.limit.unwrap_or(10)),
            Some(total),
        )
        .to_json_result(),
        Err(e) => {
            return Err(BusinessError::InternalError { source: anyhow!(e) })?;
        }
    }
}
