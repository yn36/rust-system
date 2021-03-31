use super::*;
use actix_web::{post, HttpRequest};
use model::{city::CityModel, provinces::*, request::*};

#[post("findProvinces")]
pub async fn find_provinces(
    query: web::Json<QueryBody<ProvinceModel>>,
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
        .provinces_service
        .find(
            filter,
            query.page,
            query.limit,
            Some("sort".to_string()),
            Some("asc".to_string()),
            false,
            None,
        )
        .await;
    match result {
        Ok((list, total)) => Resp::ok(
            Some(list),
            "省份查询成功",
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

#[post("findCity")]
pub async fn find_city(
    query: web::Json<QueryBody<CityModel>>,
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
        .city_service
        .find(
            filter,
            query.page,
            query.limit,
            Some("sort".to_string()),
            Some("asc".to_string()),
            false,
            None,
        )
        .await;
    match result {
        Ok((list, total)) => Resp::ok(
            Some(list),
            "城市查询成功",
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
