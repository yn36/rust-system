use super::*;
use actix_web::{delete, post, HttpRequest};
use model::{menu::*, request::*};

#[post("save")]
pub async fn save(query: web::Json<MenuModel>, req: HttpRequest, ctrl: CTRL) -> SimpleResp {
    let user = jwt_verify_to_id!(req);
    let query = query.into_inner();
    if query.prod_id.is_none() {
        return Resp::err(400, "缺少产品编码 prod_id 字段").to_json_result();
    }
    let mut d = ser::to_document(&query).unwrap();
    info!("query = {:?}", query);
    d.insert(
        "create_by",
        ObjectId::with_string(&user.id.as_str()).unwrap(),
    );
    d.insert(
        "update_by",
        ObjectId::with_string(&user.id.as_str()).unwrap(),
    );
    let prod_id = ObjectId::with_string(&query.prod_id.unwrap().as_str()).unwrap();
    d.insert("prod_id", prod_id);
    
    let result = ctrl.menu_service.save(d).await;
    match result {
        Ok(d) => Resp::ok(Some(d), "菜单新增成功", None, None, Some(1)).to_json_result(),
        Err(_) => {
            return Err(BusinessError::InternalError {
                source: anyhow!("菜单新增失败"),
            });
        }
    }
}

#[delete("delete")]
pub async fn delete(query: web::Json<RemoveList>, req: HttpRequest, ctrl: CTRL) -> SimpleResp {
    jwt_verify_to_id!(req);
    let query = query.into_inner();
    let result = ctrl.menu_service.remove(query.ids).await;
    match result {
        Ok(count) => Resp::ok(
            Some(count),
            format!("菜单删除成功{}条", count).as_str(),
            None,
            None,
            Some(count),
        )
        .to_json_result(),
        Err(e) => Resp::err(400, format!("菜单删除失败 {}", e).as_str()).to_json_result(),
    }
}

#[post("update")]
pub async fn update(query: web::Json<MenuModel>, req: HttpRequest, ctrl: CTRL) -> SimpleResp {
    let user = jwt_verify_to_id!(req);
    let query = query.into_inner();

    if query._id.is_none() {
        return Resp::err(400, "缺少 _id 字段").to_json_result();
    }
    let mut filter = ser::to_document(&query).unwrap();
    filter.insert("update_by", ObjectId::with_string(&user.id).unwrap());
    filter.remove("parent_id");
    filter.remove("prod_id");
    filter.remove("create_by");
    filter.remove("create_time");

    let result = match ctrl.menu_service.update(filter).await {
        Ok(res) => res,
        Err(e) => return Err(BusinessError::InternalError { source: anyhow!(e) })?,
    };

    match result {
        Some(result) => {
            Resp::ok(Some(result), "菜单修改成功", None, None, Some(1)).to_json_result()
        }
        None => Resp::err(400, "菜单修改失败").to_json_result(),
    }
}

#[post("find")]
pub async fn find(
    query: web::Json<QueryBody<MenuModel>>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    jwt_verify_to_id!(req);
    let query = query.into_inner();
    let mut filter = doc! {};
    if !query.body.is_none() {
        filter = struct_to_document(&query.body).unwrap();
    }
    info!("菜单查询 filter = {:?}", filter);
    let result = ctrl
        .menu_service
        .find(
            filter,
            query.page,
            query.limit,
            query.sort_name,
            query.sort_order,
            false,
            Some(vec!["prod_id"]),
        )
        .await;
    match result {
        Ok((list, total)) => {
            let mut datas = vec![];
            for mut item in list {
                item = document_handle_id(
                    item,
                    Some(vec!["prod_id", "pic_min", "pic_mid", "pic_max"]),
                )
                .unwrap();
                datas.push(item)
            }
            Resp::ok(
                Some(datas),
                "菜单查询成功",
                Some(query.page.unwrap_or(1)),
                Some(query.limit.unwrap_or(10)),
                Some(total),
            )
            .to_json_result()
        }
        Err(e) => {
            return Err(BusinessError::InternalError { source: anyhow!(e) })?;
        }
    }
}

#[post("findAll")]
pub async fn find_all(
    query: web::Json<QueryBody<MenuModel>>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    jwt_verify_to_id!(req);
    let query = query.into_inner();
    let mut filter = doc! {};
    if !query.body.is_none() {
        filter = struct_to_document(&query.body).unwrap();
    }
    let result = ctrl
        .menu_service
        .find(
            filter,
            None,
            None,
            Some("sort".to_string()),
            Some("asc".to_string()),
            true,
            Some(vec!["prod_id"]),
        )
        .await;
    match result {
        Ok((list, total)) => {
            let mut datas = vec![];
            for mut item in list {
                item = document_handle_id(item, Some(vec!["parent_id", "prod_id"])).unwrap();
                datas.push(item)
            }
            Resp::ok(Some(datas), "菜单查询成功", Some(1), Some(10), Some(total)).to_json_result()
        }
        Err(e) => {
            return Err(BusinessError::InternalError { source: anyhow!(e) })?;
        }
    }
}
