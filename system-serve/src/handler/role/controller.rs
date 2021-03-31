use super::*;
use actix_web::{delete, post, HttpRequest};
use model::{request::*, role::*};

#[post("save")]
pub async fn save(
    query: web::Json<RoleModel>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    let user = jwt_verify_to_id!(req);
    let query = query.into_inner();
    if query.name.is_none() {
        return Resp::err(400, "缺少角色名称 name 字段").to_json_result();
    } else if query.role_code.is_none() {
        return Resp::err(400, "缺少角色编码 role_code 字段").to_json_result();
    }
    // 判断编码是否已存在
    let d: Document = doc! {"role_code":query.clone().role_code.unwrap()};
    match ctrl.role_service.find_one(d).await? {
        Some(_) => {
            return Err(BusinessError::InternalError {
                source: anyhow!("该角色编码已存在"),
            })
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
    let result = ctrl.role_service.save(d).await;
    match result {
        Ok(d) => Resp::ok(Some(d), "角色新增成功", None, None, Some(1)).to_json_result(),
        Err(_) => {
            return Err(BusinessError::InternalError {
                source: anyhow!("角色新增失败"),
            });
        }
    }
}

#[delete("delete")]
pub async fn delete_role(
    query: web::Json<RemoveList>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    jwt_verify_to_id!(req);
    let query = query.into_inner();
    let result = ctrl.role_service.remove(query.ids).await;
    match result {
        Ok(count) => Resp::ok(
            Some(count),
            format!("角色删除成功{}条", count).as_str(),
            None,
            None,
            Some(count),
        )
        .to_json_result(),
        Err(e) => Resp::err(400, format!("角色删除失败 {}", e).as_str()).to_json_result(),
    }
}

#[post("update")]
pub async fn update_role(
    query: web::Json<RoleModel>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    let user = jwt_verify_to_id!(req);
    let mut query = query.into_inner();

    if query._id.is_none() {
        return Resp::err(400, "缺少 _id 字段").to_json_result();
    }

    let oid = query._id.clone().unwrap();

    if !query.role_code.is_none() {
        // 判断编码是否已存在
        let d: Document = doc! {"role_code":query.clone().role_code.unwrap()};
        match ctrl.role_service.find_one(d).await? {
            Some(d) => {
                let id = d.get("_id").unwrap().as_str().unwrap().to_string();
                if !oid.eq(&id) {
                    return Err(BusinessError::InternalError {
                        source: anyhow!("该角色编码已存在"),
                    });
                }
            }
            None => {}
        };
    }

    query.update_by = Some(user.id);

    let filter = ser::to_document(&query).unwrap();

    let result = match ctrl.role_service.update(filter).await {
        Ok(res) => res,
        Err(e) => return Err(BusinessError::InternalError { source: anyhow!(e) })?,
    };

    match result {
        Some(result) => {
            Resp::ok(Some(result), "角色修改成功", None, None, Some(1)).to_json_result()
        }
        None => Resp::err(400, "角色修改失败").to_json_result(),
    }
}

#[post("find")]
pub async fn find_roles(
    query: web::Json<QueryBody<RoleModel>>,
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
        .role_service
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
            "角色查询成功",
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

#[post("findAll")]
pub async fn find_all(req: HttpRequest, ctrl: CTRL) -> SimpleResp {
    jwt_verify_to_id!(req);

    let result = ctrl
        .role_service
        .find(
            doc! {},
            None,
            None,
            Some("sort".to_string()),
            Some("asc".to_string()),
            true,
            None,
        )
        .await;
    // info!("list = {:?}", result);
    match result {
        Ok((list, total)) => {
            Resp::ok(Some(list), "角色查询成功", Some(1), Some(10), Some(total)).to_json_result()
        }
        Err(e) => {
            return Err(BusinessError::InternalError { source: anyhow!(e) })?;
        }
    }
}
