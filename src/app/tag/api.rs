extern crate serde_json;

use super::model::Tag;
use super::response;
use crate::AppState;

use actix_web::{web, HttpResponse};

pub async fn index(state: web::Data<AppState>) -> Result<HttpResponse, HttpResponse> {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let list = web::block(move || Tag::list(&conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().json(e.to_string())
    })?;

    let res = response::TagsResponse::from(list);
    Ok(HttpResponse::Ok().json(res))
}