use super::*;
use actix_web::{delete, post, HttpRequest};
use model::{portal_product::*, request::*};

#[post("save")]
pub async fn save(
    query: web::Json<PortalProductModel>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    let user = jwt_verify!(req, ctrl);
    let query = query.into_inner();
    if query.name.is_none() {
        return Resp::err(400, "缺少产品名称.name 字段").to_json_result();
    } else if query.codes.is_none() {
        return Resp::err(400, "缺少产品编码 codes 字段").to_json_result();
    }
    // 判断编码是否已存在
    let d: Document = doc! {"codes":query.clone().codes.unwrap()};
    match ctrl.job_service.find_one(d).await? {
        Some(_) => {
            return Err(BusinessError::InternalError {
                source: anyhow!("该产品编码已存在"),
            })
        }
        None => {}
    };
    let mut d = ser::to_document(&query).unwrap();
    d.insert(
        "org_id",
        ObjectId::with_string(user.get_str("org_id").unwrap()).unwrap(),
    );
    d.insert(
        "create_by",
        ObjectId::with_string(user.get_str("_id").unwrap()).unwrap(),
    );
    d.insert(
        "update_by",
        ObjectId::with_string(user.get_str("_id").unwrap()).unwrap(),
    );
    let result = ctrl.portal_product_service.save(d).await;
    match result {
        Ok(d) => Resp::ok(Some(d), "产品新增成功", None, None, Some(1)).to_json_result(),
        Err(_) => {
            return Err(BusinessError::InternalError {
                source: anyhow!("产品新增失败"),
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
    let result = ctrl.portal_product_service.remove(query.ids).await;
    match result {
        Ok(count) => Resp::ok(
            Some(count),
            format!("产品删除成功{}条", count).as_str(),
            None,
            None,
            Some(count),
        )
        .to_json_result(),
        Err(e) => Resp::err(400, format!("产品删除失败 {}", e).as_str()).to_json_result(),
    }
}

#[post("update")]
pub async fn update(
    query: web::Json<PortalProductModel>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    let user = jwt_verify_to_id!(req);
    let mut query = query.into_inner();

    if query._id.is_none() {
        return Resp::err(400, "缺少 _id 字段").to_json_result();
    }

    if !query.codes.is_none() {
        // 判断编码是否已存在
        let d: Document = doc! {"codes":query.clone().codes.unwrap()};
        match ctrl.job_service.find_one(d).await? {
            Some(value) => {
                let oid = value.get("_id").unwrap().as_str().unwrap();
                if oid != query._id.clone().unwrap() {
                    return Err(BusinessError::InternalError {
                        source: anyhow!("该产品编码已存在"),
                    });
                }
            }
            None => {}
        };
    }

    query.update_by = Some(user.id);

    let mut filter = ser::to_document(&query).unwrap();
    filter.remove("org_id");

    let result = match ctrl.portal_product_service.update(filter).await {
        Ok(res) => res,
        Err(e) => return Err(BusinessError::InternalError { source: anyhow!(e) })?,
    };

    match result {
        Some(result) => {
            Resp::ok(Some(result), "产品修改成功", None, None, Some(1)).to_json_result()
        }
        None => Resp::err(400, "产品修改失败").to_json_result(),
    }
}

#[post("find")]
pub async fn find(
    query: web::Json<QueryBody<PortalProductModel>>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    let user = jwt_verify!(req, ctrl);
    let query = query.into_inner();
    // let filter = ser::to_document(&query.body).unwrap();
    let mut filter = doc! {};
    if !query.body.is_none() {
        filter = struct_to_document(&query.body).unwrap();
    }

    let org_id = user.get_object_id("org_id").unwrap();
    let org_id = org_id.to_hex();
    filter.insert("org_id", org_id);

    let result = ctrl
        .portal_product_service
        .find(
            filter,
            query.page,
            query.limit,
            query.sort_name,
            query.sort_order,
            false,
            Some(vec!["org_id"]),
        )
        .await;
    match result {
        Ok((list, total)) => {
            let mut datas = vec![];
            for item in list {
                datas.push(document_handle_id(item, Some(vec!["org_id"])))
            }
            Resp::ok(
                Some(datas),
                "产品查询成功",
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
