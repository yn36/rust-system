use super::*;
use actix_web::{delete, post, HttpRequest};
use model::{org::*, request::*};

#[post("save")]
pub async fn save(
    query: web::Json<OrgModel>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    let user = jwt_verify_to_id!(req);
    let query = query.into_inner();
    if query.name.is_none() {
        return Resp::err(400, "缺少机构名称 name 字段").to_json_result();
    } else if query.org_code.is_none() {
        return Resp::err(400, "缺少机构编码 org_code 字段").to_json_result();
    }
    // 判断编码是否已存在
    let d: Document = doc! {"org_code":query.clone().org_code.unwrap()};
    match ctrl.org_service.find_one(d).await? {
        Some(_) => {
            return Err(BusinessError::InternalError {
                source: anyhow!("该机构编码已存在"),
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
    info!("query.parent_id = {:?}", query.parent_id);
    if query.parent_id.is_none() || query.parent_id.clone().unwrap().eq("-1") {
        d.insert("parent_id", bson::Bson::Null);
    } else {
        d.insert(
            "parent_id",
            ObjectId::with_string(&query.parent_id.unwrap().as_str()).unwrap(),
        );
    }

    let result = ctrl.org_service.save(d).await;
    match result {
        Ok(d) => Resp::ok(Some(d), "机构新增成功", None, None, Some(1)).to_json_result(),
        Err(_) => {
            return Err(BusinessError::InternalError {
                source: anyhow!("机构新增失败"),
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
    let result = ctrl.org_service.remove(query.ids).await;
    match result {
        Ok(count) => Resp::ok(
            Some(count),
            format!("机构删除成功{}条", count).as_str(),
            None,
            None,
            Some(count),
        )
        .to_json_result(),
        Err(e) => Resp::err(400, format!("机构删除失败 {}", e).as_str()).to_json_result(),
    }
}

#[post("update")]
pub async fn update(
    query: web::Json<OrgModel>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    let user = jwt_verify_to_id!(req);
    let mut query = query.into_inner();

    if query._id.is_none() {
        return Resp::err(400, "缺少 _id 字段").to_json_result();
    }

    if !query.org_code.is_none() {
        // 判断编码是否已存在
        let d: Document = doc! {"org_code":query.clone().org_code.unwrap()};
        match ctrl.org_service.find_one(d).await? {
            Some(value) => {
                let oid = value.get("_id").unwrap().as_str().unwrap();
                if oid != query._id.clone().unwrap() {
                    return Err(BusinessError::InternalError {
                        source: anyhow!("该机构编码已存在"),
                    });
                }
            }
            None => {}
        };
    }

    let mut filter = ser::to_document(&query).unwrap();
    filter.remove("parent_id").unwrap();
    filter.remove("create_by").unwrap();

    filter.insert(
        "update_by",
        ObjectId::with_string(&user.id.as_str()).unwrap(),
    );
    let result = match ctrl.org_service.update(filter).await {
        Ok(res) => res,
        Err(e) => return Err(BusinessError::InternalError { source: anyhow!(e) })?,
    };

    match result {
        Some(result) => {
            Resp::ok(Some(result), "机构修改成功", None, None, Some(1)).to_json_result()
        }
        None => Resp::err(400, "机构修改失败").to_json_result(),
    }
}

#[post("find")]
pub async fn find(
    query: web::Json<QueryBody<OrgModel>>,
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
        .org_service
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
            "机构查询成功",
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

// TODO 树形结构查询未完成
#[post("findTree")]
pub async fn find_tree(req: HttpRequest, ctrl: CTRL) -> SimpleResp {
    jwt_verify_to_id!(req);
    let mut pipeline = doc! {};
    pipeline.insert(
        "$lookup",
        doc! { "from":"org_list",  "localField": "_id", "foreignField": "parent_id","as":"children"},
    );
    // pipeline.insert(
    //     "$match",
    //     doc! { "children":doc! {"$ne":bson::Bson::Array(vec![])}},
    // );
    // info!("pipeline = {:?}", pipeline);
    // let pipeline = bson::Bson::Array(vec![
    //     doc! {"$lookup":doc! { "from":"org_list",  "localField": "_id", "foreignField": "parent_id","as":"children"}}.into(),
    //     doc! {"$match":doc! {"children":doc! {"$ne":bson::Bson::Array(vec![])}}}.into(),
    // ]);
    // info!("pipeline = {:?}", pipeline);
    // match pipeline.as_document() {
    //     Some(value) => {
    //         info!("value = {:?}", value);
    //     }
    //     None => {
    //         info!("pipeline = {:?}", pipeline);
    //     }
    // }
    let mut cursor = match ctrl
        .org_service
        .op
        .coll
        .aggregate(Some(pipeline), None)
        .await
    {
        Ok(value) => {
            info!("value = {:?}", value);
            value
        }
        Err(e) => {
            info!("e = {:?}", e);
            return Err(BusinessError::InternalError { source: anyhow!(e) })?;
        }
    };
    let result = cursor.as_vec(true).await;
    // info!("list = {:?}", result);
    match result {
        Ok(list) => {
            let mut datas = vec![];
            for mut item in list {
                item = document_handle_id(item, Some(vec!["parent_id"])).unwrap();
                datas.push(item)
            }
            Resp::ok(Some(datas), "机构查询成功", Some(1), Some(10), Some(1)).to_json_result()
        }
        Err(e) => {
            return Err(BusinessError::InternalError { source: anyhow!(e) })?;
        }
    }
}

#[post("findAll")]
pub async fn find_all(req: HttpRequest, ctrl: CTRL) -> SimpleResp {
    jwt_verify_to_id!(req);

    let result = ctrl
        .org_service
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
            let mut datas = vec![];
            for mut item in list {
                item = document_handle_id(item, Some(vec!["parent_id"])).unwrap();
                datas.push(item)
            }
            Resp::ok(Some(datas), "机构查询成功", Some(1), Some(10), Some(total)).to_json_result()
        }
        Err(e) => {
            return Err(BusinessError::InternalError { source: anyhow!(e) })?;
        }
    }
}
