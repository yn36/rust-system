use super::*;
use actix_web::{delete, post, HttpRequest};
use model::{menu::*, request::*, role::*, theme::ThemeModel, user::*};

/// 判断是否认证
#[post("isAuthed")]
pub async fn is_authed(req: HttpRequest, ctrl: CTRL) -> SimpleResp {
    let user_token = jwt_verify_to_id!(req);

    // 查询语句
    // db.user_lists.aggregate([
    //    {$lookup:{
    //        from:"user_roles",
    //        localField:"_id",
    //        foreignField:"user_id",
    //        as:"roles"
    //    }},
    //    {$unwind:"$roles"},
    //    {$project:{
    //        roles:{
    //            "_id":{"$toString":"$roles._id"}}}}
    //    ])
    let oid = ObjectId::with_string(user_token.id.as_str()).unwrap();

    let query_match = doc! {
        "$match":doc! {
            "_id":oid
        }
    };

    let query_project = doc! {
        "$project":doc! {
            "_id":doc!{
                "$toString":"$_id"
            },
            "birth_day":1,
            "birth_month":1,
            "birth_year":1,
            "desc":1,
            "email":1,
            "enabled":1,
            "last_login_ip":1,
            "last_login_time":1,
            "org_id":doc!{
                "$toString":"$org_id"
            },
            "org_name":1,
            "portrait":doc!{
                "$toString":"$portrait"
            },
            "qq":1,
            "sex":1,
            "total_login_count":1,
            "types":1,
            "username":1,
            "roles":1
            // "roles":doc!{
            //     "_id":doc!{
            //         "$toString":"$roles._id"
            //     },
            //     "name":1
            // }
        }
    };

    let mut cursor = ctrl
        .user_service
        .get_op()
        .coll
        .aggregate(
            vec![
                query_match,
                doc! {
                    "$lookup":doc! {
                        "from":"user_roles",
                        "localField":"_id",
                        "foreignField":"user_id",
                        "as":"roles"
                    }
                },
                // doc! {
                //     "$unwind":"$roles"
                // },
                query_project,
            ],
            None,
        )
        .await?;

    match cursor.as_vec(false).await {
        Ok(list) => {
            if list.len() > 0 {
                let mut data = list[0].clone();
                let mut roles = vec![];
                for role in data.get_array("roles").unwrap() {
                    let role = role.as_document().unwrap().clone();
                    roles.push(
                        document_handle_id(role, Some(vec!["role_id", "user_id", "create_by"]))
                            .unwrap()
                            .into(),
                    )
                }
                data.insert("roles", bson::Bson::Array(roles));
                Resp::ok(Some(data), "用户已认证", None, None, Some(1)).to_json_result()
            } else {
                return Resp::err(400, "用户不存在").to_json_result();
            }
        }
        Err(_) => {
            return Err(BusinessError::Unauthorized);
        }
    }
}

/// 用户注册
#[post("reg")]
pub async fn reg_user(query: web::Json<UserModel>, _req: HttpRequest, ctrl: CTRL) -> SimpleResp {
    let query = query.into_inner();
    if query.username.is_none() {
        return Resp::err(400, "缺少用户名 username").to_json_result();
    } else if query.password.is_none() {
        return Resp::err(400, "缺少密码 password").to_json_result();
    }
    // 判断是否已注册
    let d: Document = doc! {"username": query.clone().username.unwrap()};
    match ctrl.user_service.find_one(d).await? {
        Some(_) => {
            return Err(BusinessError::InternalError {
                source: anyhow!("该用户已注册"),
            })
        }
        None => {}
    };

    let mut d = ser::to_document(&query).unwrap();
    d.insert(
        "password",
        get_password_default(
            d.get("password").unwrap().as_str().unwrap(),
            &*toml_conf::SETTING.app.secret_key,
        ),
    );
    d.insert("types", 2);
    d.insert("enabled", 1);
    d.insert("total_login_failure", 0);
    d.insert("total_login_count", 0);
    d.insert(
        "org_id",
        ObjectId::with_string(query.org_id.unwrap().as_str()).unwrap(),
    );
    let result = ctrl.user_service.save(d).await.unwrap();
    let uid = result.get("_id").unwrap().as_str().clone().unwrap();
    let uid = ObjectId::with_string(uid).unwrap();
    // 新用户创建时自动新建主题
    let theme = ThemeModel {
        _id: None,
        appearance: Some("auto".to_string()),
        theme_color: Some("#13c2c2".to_string()),
        navigate: Some("head".to_string()),
        fixed_header: Some(true),
        week_mode: Some(false),
        multi_pages: Some(true),
        user_id: Some(uid.clone()),
        user_name: Some(result.get("username").unwrap().clone().to_string()),
        create_by: Some(uid.clone()),
        create_time: None,
        update_by: Some(uid.clone()),
        update_time: None,
    };
    let theme = ser::to_document(&theme).unwrap();
    ctrl.theme_service.save(theme).await.unwrap();

    // 注册/新增用户时 默认添加 普通用户角色 role_id:601d697a00513d230026adb4 name:普通用户
    ctrl.user_role_service
        .save_role(vec![doc! {
            "role_id":ObjectId::with_string("601d697a00513d230026adb4").unwrap(),
            "user_id": uid.clone(),
            "create_by":uid
        }])
        .await?;

    Resp::ok(Some(result), "注册成功", Some(1), Some(1), Some(1)).to_json_result()
}

/// 用户查询
#[post("find")]
pub async fn get_user_list(
    query: web::Json<QueryBody<UserModel>>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    jwt_verify_to_id!(req);
    let query = query.into_inner();
    let mut filter: Document = doc! {};
    if !query.body.is_none() {
        filter = struct_to_document(&query.body).unwrap();
    }
    // 模糊查询
    let mut query_match = doc! {};
    let keys = filter.keys();
    let mut d = doc! {};
    let mut list = vec![];
    for k in keys.into_iter() {
        if !k.eq("_id") {
            match filter.get(k).unwrap().as_str() {
                Some(_) => {
                    let doc = doc! { k: bson::Regex {pattern:filter.get(k).unwrap().as_str().unwrap().to_string(),options:"i".to_string()}}.into();
                    list.push(doc);
                }
                None => {
                    let doc = doc! { k:bson::Bson::Int32(filter.get(k).unwrap().as_i32().unwrap())}
                        .into();
                    list.push(doc);
                }
            }
        } else {
            let oid = filter.get("_id").unwrap().as_str().unwrap();
            let oid = ObjectId::with_string(oid).unwrap();
            d.insert("_id", oid);
        }
    }
    if list.len() > 0 {
        d.insert("$and", bson::Bson::Array(list));
    }
    query_match.insert("$match", d);

    // 排序
    let mut query_sort = doc! {};
    // 设置查询排序  默认创建时间的倒序
    let mut sort = doc! {};
    if !query.sort_name.clone().unwrap_or("".to_string()).is_empty()
        && !query
            .sort_order
            .clone()
            .unwrap_or("".to_string())
            .is_empty()
    {
        if query.sort_order.unwrap().eq("desc") {
            sort.insert(query.sort_name.unwrap(), -1);
        } else {
            // sort.insert("province_code", 1);
            sort.insert(query.sort_name.unwrap(), 1);
        }
    } else {
        sort.insert("create_time", -1);
    }

    query_sort.insert("$sort", sort);
    info!("query_sort = {:?}", query_sort);
    // 聚合查询角色
    // db.user_lists.aggregate([{$lookup:{from:"user_roles",localField:"_id",foreignField:"user_id",as:"roles"}},{$match:{$and:[{"username":{$regex:"u"}}]}}])
    // let pipeline = vec![
    //     doc! {"$lookup":doc! {"from":"user_roles","localField":"_id","foreignField":"user_id","as":"roles"}}.into(),
    //     doc! {"$project":doc! {"roles":doc! {"_id":"0"}}}.into(),
    // ];
    // 设置一页多少条
    let page = Some(query.page.unwrap_or(1));
    let limit = Some(query.limit.unwrap_or(10));
    let skip = limit.unwrap_or(10) * (page.unwrap_or(1) - 1);
    // 指定输出文档里的字段.
    let query_project = doc! {
        "$project":doc! {
            "username":1,
            "realname":1,
            "birth_year":1,
            "birth_month":1,
            "birth_day":1,
            "sex":1,
            "org_id":1,
            "org_name":1,
            "types":1,
            "desc":1,
            "qq":1,
            "phone":1,
            "email":1,
            "enabled":1,
            "create_time":1,
            "update_time":1,
            // "roles":doc! {"_id":doc! {"$toString":"$_id"},"name":1}
            "roles":doc! {"name":1,"_id":1}
        }
    };
    let mut cursor = ctrl
        .user_service
        .get_op()
        .coll
        .aggregate(vec![
            doc! {"$lookup":doc! {"from":"user_roles","localField":"_id","foreignField":"user_id","as":"roles"}}.into(),
            query_match,
            query_sort,
            doc! {"$skip":skip},
            doc! {"$limit":limit.unwrap_or(10)},
            query_project,
            // doc! {"$project":doc! {"roles":doc! {"_id":0}}}.into()
        ], None)
        .await?;
    let result = cursor.as_vec(true).await;
    let total = ctrl.user_service.get_op().count(filter).await?;
    match result {
        Ok(list) => {
            let mut datas = vec![];
            for item in list {
                datas.push(document_handle_id(item, Some(vec!["org_id"])).unwrap());
            }
            Resp::ok(
                Some(datas),
                "用户查询成功",
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

/// 用户修改
/// 用户修改接口 不需提供_id 默认更改当前登陆人信息
#[post("update")]
pub async fn update_user(query: web::Json<UserModel>, req: HttpRequest, ctrl: CTRL) -> SimpleResp {
    let user = jwt_verify_to_id!(req);
    let query = query.into_inner();

    let mut filter = ser::to_document(&query).unwrap();
    filter.remove("username").unwrap();
    if query.org_id.is_some() {
        filter.insert(
            "org_id",
            ObjectId::with_string(query.org_id.unwrap().as_str()).unwrap(),
        );
    }

    filter.insert(
        "update_by",
        ObjectId::with_string(user.id.as_str()).unwrap(),
    );

    info!("用户修改 {:?}", filter);
    let result = match ctrl.user_service.update(filter).await {
        Ok(res) => res,
        Err(_) => {
            return Err(BusinessError::InternalError {
                source: anyhow!("修改失败"),
            })?
        }
    };

    match result {
        Some(mut result) => {
            result.remove("password");
            let result = document_handle_id(result, Some(vec!["portrait"])).unwrap();
            Resp::ok(Some(result), "用户修改成功", Some(1), Some(1), Some(1)).to_json_result()
        }
        None => Resp::err(400, "修改失败").to_json_result(),
    }
}

/// 用户删除
#[delete("delete")]
pub async fn delete_user(query: web::Json<RemoveList>, req: HttpRequest, ctrl: CTRL) -> SimpleResp {
    jwt_verify_to_id!(req);
    let query = query.into_inner();
    let result = ctrl.user_service.remove(query.ids.clone()).await;
    match result {
        Ok(count) => {
            let arr: Vec<&str> = query.ids.rsplit(",").collect();
            let mut remids: Vec<ObjectId> = Vec::new();
            for _id in arr {
                remids.push(ObjectId::with_string(_id).unwrap());
            }
            let mut doc: Document = doc! {};
            doc.insert("$in", remids);
            let d = doc! {"user_id":doc};
            ctrl.theme_service
                .get_op()
                .coll
                .delete_many(d.clone(), None)
                .await?;
            ctrl.user_role_service
                .get_op()
                .coll
                .delete_many(d.clone(), None)
                .await?;
            ctrl.auth_menu_service
                .get_op()
                .coll
                .delete_many(d, None)
                .await?;

            Resp::ok(
                Some(count),
                format!("删除成功{}条", count).as_str(),
                Some(1),
                Some(1),
                Some(count),
            )
            .to_json_result()
        }
        Err(e) => Resp::err(400, format!("{}", e).as_str()).to_json_result(),
    }
}

/// 用户登录
#[post("login")]
pub async fn login(query: web::Json<Login>, req: HttpRequest, ctrl: CTRL) -> SimpleResp {
    let query = query.into_inner();
    info!("用户登录 {:?}", query);
    let d: Document = doc! {"username": query.username};
    let result = ctrl.user_service.find_one(d).await?;
    let result = match result {
        Some(d) => d,
        None => {
            return Err(BusinessError::InternalError {
                source: anyhow!("该用户未注册"),
            });
        }
    };

    // 累计登录错误次数，超过最高次数，将限制登录
    let total_login_failure = result.get("total_login_failure").unwrap().as_i32().unwrap();
    // 用户状态 1.正常，2.禁用，3.注销
    let enabled = result.get("enabled").unwrap().as_i32().unwrap();
    if total_login_failure.eq(&5) || enabled.eq(&2) {
        return Err(BusinessError::InternalError {
            source: anyhow!("该账号已被锁定"),
        });
    }
    let password = get_password_default(&query.password, &*toml_conf::SETTING.app.secret_key);

    let mut user = UserModel::default();

    let oid = result.get("_id").unwrap().as_str().unwrap();
    user._id = Some(oid.to_string());

    if password.eq(result.get("password").unwrap().as_str().unwrap()) {
        // 登陆次数
        let total_login_count = result.get("total_login_count").unwrap().as_i32().unwrap();

        user.total_login_count = Some(total_login_count + 1); // 修改登陆次数
        user.total_login_failure = Some(0); // 重置登陆失败次数
        let ip = req.head().peer_addr.unwrap().ip(); // 获取ip地址
        user.last_login_ip = Some(ip.to_string()); // 更新ip地址
        user.last_login_time = Some(yn_util::date_time::to_string()); // 更新登陆时间
        let username = result.get("username").unwrap().as_str();

        let mut filter = ser::to_document(&user).unwrap();
        filter.remove("username").unwrap();
        let mut result = match ctrl.user_service.update(filter).await {
            Ok(value) => value.unwrap(),
            Err(e) => {
                return Err(BusinessError::InternalError { source: anyhow!(e) });
            }
        };

        // 处理头像返回的id
        let portrait = match result.get_object_id("portrait") {
            Ok(id) => id.to_hex(),
            Err(_) => String::new(),
        };
        result.insert("portrait", portrait);
        // 过滤不必返回的字段
        result.remove("password").unwrap();
        let res = ResUserInfo {
            user_info: Some(result),
            token: Some(yn_util::jwt::encode(oid, username.unwrap())),
        };
        Resp::ok(Some(res), "登陆成功", None, None, None).to_json_result()
    } else {
        let total_login_failure = total_login_failure + 1;
        user.total_login_failure = Some(total_login_failure);
        if total_login_failure >= 5 {
            user.enabled = Some(2);
        }

        let mut filter = ser::to_document(&user).unwrap();
        filter.remove("username").unwrap();
        ctrl.user_service.update(filter).await?;
        Resp::err(404, "密码错误").to_json_result()
    }
}

/// 用户登出
#[post("logout")]
pub async fn logout(req: HttpRequest) -> SimpleResp {
    let user = jwt_verify_to_id!(req);
    info!("用户{:?}登出", user.name);
    Resp::ok(Some("登出成功"), "登出成功", None, None, None).to_json_result()
}

/// 用户添加角色
#[post("/save/roles")]
pub async fn save_roles(
    query: web::Json<AuthQuery<RoleModel>>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    let token = jwt_verify_to_id!(req);
    let query = query.into_inner();
    // info!("用户角色权限修改 query = {:#?}", query);
    if query.user_id.clone().unwrap().len() > 0 {
        for user_id in query.user_id.clone().unwrap() {
            // 添加角色
            if query.adds.clone().is_some() && query.adds.clone().unwrap().len() > 0 {
                let mut adds_list = vec![];
                let adds = query.adds.clone().unwrap().into_iter();
                for r in adds.into_iter() {
                    let user_role = UserRoleModel::default();
                    let mut doc = bson::to_bson(&user_role)
                        .unwrap()
                        .as_document()
                        .unwrap()
                        .to_owned();
                    doc.insert(
                        "role_id",
                        ObjectId::with_string(r._id.unwrap().as_str()).unwrap(),
                    );
                    doc.insert("name", r.name.unwrap());
                    doc.insert("user_id", ObjectId::with_string(&user_id).unwrap());
                    doc.insert(
                        "create_by",
                        ObjectId::with_string(&token.id.as_str()).unwrap(),
                    );
                    adds_list.push(doc);
                }
                let result = ctrl.user_role_service.save_role(adds_list).await;
                match result {
                    Ok(value) => {
                        info!("insert = {:?}", value)
                    }
                    Err(_) => {}
                }
            }

            // 删除角色
            if query.deletes.clone().is_some() && query.deletes.clone().unwrap().len() > 0 {
                let mut remvec = vec![];
                let user_id = ObjectId::with_string(&user_id).unwrap();
                for item in query.deletes.clone().unwrap() {
                    let role_id = ObjectId::with_string(item._id.unwrap().as_str()).unwrap();
                    let mut doc = doc! {};
                    doc.insert(
                        "$and",
                        bson::Bson::Array(vec![
                            doc! {"role_id":role_id}.into(),
                            doc! {"user_id":user_id.clone()}.into(),
                        ]),
                    );
                    remvec.push(doc.into())
                }
                let remvec = bson::Bson::Array(remvec);
                let docs = doc! {"$or":remvec};
                let ret = ctrl
                    .user_role_service
                    .get_op()
                    .coll
                    .delete_many(docs, None)
                    .await;
                match ret {
                    Ok(value) => {
                        info!("value = {:?}", value);
                    }
                    Err(_) => {}
                }
            }
        }
    }
    Resp::ok(Some("操作成功"), "message", None, None, Some(1)).to_json_result()
}

/// 用户权限查询 (角色)
#[post("/find/roles")]
pub async fn get_user_roles_list(
    query: web::Json<QueryBody<UserRoleModel>>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    jwt_verify_to_id!(req);
    let query = query.into_inner();
    let mut filter: Document = doc! {};
    if query.body.is_none() {
        return Err(BusinessError::InternalError {
            source: anyhow!("缺少参数 user_id"),
        })?;
    }

    let oid = ObjectId::with_string(query.body.unwrap().user_id.unwrap().as_str()).unwrap();
    filter.insert("user_id", oid);

    let result = ctrl
        .user_role_service
        .get_op()
        .coll
        .find(filter, None)
        .await;
    let total = 1;
    match result {
        Ok(mut cursor) => {
            let list = cursor.as_vec(true).await?;
            let mut datas = vec![];
            for item in list.into_iter() {
                datas.push(document_handle_id(item, Some(vec!["user_id", "role_id"])).unwrap());
            }
            Resp::ok(Some(datas), "角色查询成功", None, None, Some(total)).to_json_result()
        }
        Err(e) => {
            return Err(BusinessError::InternalError { source: anyhow!(e) })?;
        }
    }
}

/// 菜单权限查询 (菜单) 查询
#[post("/prods/menu/find")]
pub async fn get_prods_menu_list(
    query: web::Json<AuthQuery<AuthMenuModel>>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    jwt_verify_to_id!(req);
    let query = query.into_inner();
    if query.prod_id.is_none() {
        return Err(BusinessError::InternalError {
            source: anyhow!("缺少参数 prod_id 产品编码"),
        })?;
    }
    if query.user_id.is_none() && query.role_id.is_none() {
        return Err(BusinessError::InternalError {
            source: anyhow!("参数 user_id,role_id 必须包含一个"),
        })?;
    }

    // 聚合查询
    // db.menu_list.aggregate([{
    //     $lookup:{
    //        from:"auth_menu_list",
    //        let:{menu_id:"$_id"},
    //        pipeline:[{
    //             $match:{
    //                   $and:[
    //                     {$expr:{ $eq: [ "$menu_id",  "$$menu_id" ] }}
    //                   ],
    //                $or:[
    //                  {$expr:{ $eq: [ "$name",  "工作台" ] }},
    //                  {$expr:{ $eq: [ "$name",  "首页" ] }}
    //                 ]
    //               }
    //             }
    //         ],
    //        as:"menu_list"}}])

    // 产品编码
    let prod_id = ObjectId::with_string(query.prod_id.unwrap().as_str()).unwrap();
    let query_match = doc! {
        "$match":doc! {"prod_id":prod_id},
    };

    let and_list: Vec<bson::Bson> = vec![
        doc! {"$expr": doc! {"$eq":bson::Bson::Array(vec![bson::Bson::String("$menu_id".to_string()),  bson::Bson::String("$$menu_id".to_string())])}}.into(),
    ];
    let mut or_list: Vec<bson::Bson> = vec![];
    let mut role_id_list = vec![];
    if query.user_id.is_some() {
        let user_id = query.user_id.unwrap()[0].clone();
        or_list.push(
            doc! {"$expr":doc! {"$eq":bson::Bson::Array(vec![
                bson::Bson::String("$user_id".to_string()),
                bson::Bson::ObjectId(ObjectId::with_string(user_id.as_str()).unwrap())
            ])}}
            .into(),
        );
        let filter = doc! {
            "user_id":user_id
        };
        match ctrl
            .user_role_service
            .find(filter, None, None, None, None, true, Some(vec!["user_id"]))
            .await
        {
            Ok((list, _)) => {
                role_id_list = list
                    .into_iter()
                    .map(|mut doc| doc.remove("role_id").unwrap())
                    .collect();
            }
            Err(e) => {
                return Err(BusinessError::InternalError { source: anyhow!(e) })?;
            }
        }
    } else {
        for role_id in query.role_id.unwrap() {
            let role_id = ObjectId::with_string(role_id.as_str()).unwrap();
            role_id_list.push(bson::Bson::ObjectId(role_id));
        }
    }
    if role_id_list.len() > 0 {
        for role_id in role_id_list {
            or_list.push(
                doc! {"$expr": doc! {"$eq":bson::Bson::Array(vec![
                    bson::Bson::String("$role_id".to_string()),
                    role_id.clone()
                ])}}
                .into(),
            );
        }
    }
    let and = bson::Bson::Array(and_list);
    let or = bson::Bson::Array(or_list);
    let pipeline = bson::Bson::Array(vec![doc! {"$match":doc!{"$and":and,"$or":or}}.into()]);

    let mut cursor = ctrl
        .menu_service
        .get_op()
        .coll
        .aggregate(
            vec![
                query_match,
                doc! {"$lookup":doc! {
                    "from":"auth_menu_list",
                    "let":doc!{"menu_id":"$_id"},
                    "pipeline":pipeline,
                    "as":"menu_list"
                }},
            ],
            None,
        )
        .await?;

    let result = cursor.as_vec(true).await;

    let total = 1;
    match result {
        Ok(list) => {
            let mut datas = vec![];
            for mut item in list.into_iter() {
                if item.get("menu_list").unwrap().as_array().unwrap().len() > 0 {
                    item.insert("checked", bson::Bson::Boolean(true));
                } else {
                    item.insert("checked", bson::Bson::Boolean(false));
                }
                item.remove("menu_list");
                datas.push(document_handle_id(item, Some(vec!["prod_id"])).unwrap());
            }
            Resp::ok(Some(datas), "菜单查询成功", None, None, Some(total)).to_json_result()
        }
        Err(e) => {
            return Err(BusinessError::InternalError { source: anyhow!(e) })?;
        }
    }
}

/// 用户/角色添加菜单 (菜单) 添加
#[post("/prods/menu/save")]
pub async fn save_auth_menu(
    query: web::Json<AuthQuery<AuthMenuModel>>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    let token = jwt_verify_to_id!(req);
    let query = query.into_inner();
    if query.prod_id.is_none() {
        return Err(BusinessError::InternalError {
            source: anyhow!("缺少参数 prod_id 产品编码"),
        })?;
    }
    let mut relation_list = vec![];
    let mut is_user = true;
    if query.user_id.is_some() {
        relation_list = query.user_id.unwrap();
    } else if query.role_id.is_some() {
        is_user = false;
        relation_list = query.role_id.unwrap();
    } else {
        return Err(BusinessError::InternalError {
            source: anyhow!("参数 user_id,role_id 必须包含一个"),
        })?;
    }
    if relation_list.len() <= 0 {
        return Err(BusinessError::InternalError {
            source: anyhow!("参数 user_id,role_id 必须包含一个"),
        })?;
    }
    for relation_id in relation_list {
        // 添加菜单
        if query.adds.clone().is_some() && query.adds.clone().unwrap().len() > 0 {
            let mut adds_list = vec![];
            let adds = query.adds.clone().unwrap().into_iter();
            for r in adds.into_iter() {
                let user_role = AuthMenuModel::default();
                let mut doc = bson::to_bson(&user_role)
                    .unwrap()
                    .as_document()
                    .unwrap()
                    .to_owned();
                doc.insert(
                    "menu_id",
                    ObjectId::with_string(r._id.unwrap().as_str()).unwrap(),
                );
                doc.insert("name", r.name.unwrap());
                if is_user {
                    doc.insert(
                        "user_id",
                        ObjectId::with_string(relation_id.clone().as_str()).unwrap(),
                    );
                } else {
                    doc.insert(
                        "role_id",
                        ObjectId::with_string(relation_id.clone().as_str()).unwrap(),
                    );
                }
                doc.insert(
                    "relation_id",
                    ObjectId::with_string(relation_id.clone().as_str()).unwrap(),
                );
                doc.insert(
                    "create_by",
                    ObjectId::with_string(&token.id.as_str()).unwrap(),
                );
                doc.insert(
                    "update_by",
                    ObjectId::with_string(&token.id.as_str()).unwrap(),
                );
                adds_list.push(doc);
            }
            ctrl.auth_menu_service.save_role(adds_list).await?;
        }

        // 删除菜单
        if query.deletes.clone().is_some() && query.deletes.clone().unwrap().len() > 0 {
            let mut remvec = vec![];
            let user_id = ObjectId::with_string(&relation_id).unwrap();
            for item in query.deletes.clone().unwrap() {
                let role_id = ObjectId::with_string(item._id.unwrap().as_str()).unwrap();
                let mut doc = doc! {};
                doc.insert(
                    "$and",
                    bson::Bson::Array(vec![
                        doc! {"menu_id":role_id}.into(),
                        doc! {"relation_id":user_id.clone()}.into(),
                    ]),
                );
                remvec.push(doc.into())
            }
            let remvec = bson::Bson::Array(remvec);
            let docs = doc! {"$or":remvec};
            ctrl.auth_menu_service
                .get_op()
                .coll
                .delete_many(docs, None)
                .await?;
        }
    }
    Resp::ok(Some("操作成功"), "message", None, None, Some(1)).to_json_result()
}

/// 菜单权限查询 (菜单) 菜单路由
#[post("/prods/menus")]
pub async fn get_prods_menu(
    query: web::Json<MenuModel>,
    req: HttpRequest,
    ctrl: CTRL,
) -> SimpleResp {
    let user = jwt_verify_to_id!(req);
    let query = query.into_inner();
    if query.prod_id.is_none() {
        return Err(BusinessError::InternalError {
            source: anyhow!("缺少参数 prod_id 产品编码"),
        })?;
    }

    // 当前登录 用户id
    let user_id = user.id.as_str();

    // 聚合查询
    // db.menu_list.aggregate([{
    //     $lookup:{
    //        from:"auth_menu_list",
    //        let:{menu_id:"$_id"},
    //        pipeline:[{
    //             $match:{
    //                   $and:[
    //                     {$expr:{ $eq: [ "$menu_id",  "$$menu_id" ] }}
    //                   ],
    //                $or:[
    //                  {$expr:{ $eq: [ "$name",  "工作台" ] }},
    //                  {$expr:{ $eq: [ "$name",  "首页" ] }}
    //                 ]
    //               }
    //             }
    //         ],
    //        as:"menu_list"}}])

    // 产品编码
    let prod_id = ObjectId::with_string(query.prod_id.unwrap().as_str()).unwrap();
    let query_match = doc! {
        "$match":doc! {"prod_id":prod_id},
    };

    let and_list: Vec<bson::Bson> = vec![
        doc! {"$expr": doc! {"$eq":bson::Bson::Array(vec![bson::Bson::String("$menu_id".to_string()),  bson::Bson::String("$$menu_id".to_string())])}}.into(),
    ];
    let mut or_list: Vec<bson::Bson> = vec![];
    let mut role_id_list = vec![];
    or_list.push(
        doc! {"$expr":doc! {"$eq":bson::Bson::Array(vec![
            bson::Bson::String("$user_id".to_string()),
            bson::Bson::ObjectId(ObjectId::with_string(user_id.clone()).unwrap())
        ])}}
        .into(),
    );
    let filter = doc! {
        "user_id":user_id
    };
    match ctrl
        .user_role_service
        .find(filter, None, None, None, None, true, Some(vec!["user_id"]))
        .await
    {
        Ok((list, _)) => {
            role_id_list = list
                .into_iter()
                .map(|mut doc| doc.remove("role_id").unwrap())
                .collect();
        }
        Err(e) => {
            return Err(BusinessError::InternalError { source: anyhow!(e) })?;
        }
    }
    if role_id_list.len() > 0 {
        for role_id in role_id_list {
            or_list.push(
                doc! {"$expr": doc! {"$eq":bson::Bson::Array(vec![
                    bson::Bson::String("$role_id".to_string()),
                    role_id.clone()
                ])}}
                .into(),
            );
        }
    }
    let and = bson::Bson::Array(and_list);
    let or = bson::Bson::Array(or_list);
    let pipeline = bson::Bson::Array(vec![doc! {"$match":doc!{"$and":and,"$or":or}}.into()]);

    let mut cursor = ctrl
        .menu_service
        .get_op()
        .coll
        .aggregate(
            vec![
                query_match,
                doc! {"$lookup":doc! {
                    "from":"auth_menu_list",
                    "let":doc!{"menu_id":"$_id"},
                    "pipeline":pipeline,
                    "as":"menu_list"
                }},
                doc! {
                    "$match":doc! {
                        "menu_list":doc! {"$ne":bson::Bson::Array(Vec::new())}
                    }
                },
            ],
            None,
        )
        .await?;

    let result = cursor.as_vec(true).await;

    let total = 1;
    match result {
        Ok(list) => {
            let mut datas = vec![];
            for mut item in list.into_iter() {
                // if item.get("menu_list").unwrap().as_array().unwrap().len() > 0 {
                //     item.insert("checked", bson::Bson::Boolean(true));
                // } else {
                //     item.insert("checked", bson::Bson::Boolean(false));
                // }
                item.remove("menu_list");
                datas.push(document_handle_id(item, Some(vec!["prod_id"])).unwrap());
            }
            Resp::ok(Some(datas), "菜单查询成功", None, None, Some(total)).to_json_result()
        }
        Err(e) => {
            return Err(BusinessError::InternalError { source: anyhow!(e) })?;
        }
    }
}
