use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use web::Query;

// // 动态
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/event").route(web::get().to(event)));
}

// 入参
define_request_struct!(Event, {
    pagesize: Option<i32>,
    lasttime: Option<i32>,
});

impl Event {
  async fn requests(req: HttpRequest, query: Query<Event>) -> Result<Response, Value> {
    let data = json!({
        "pagesize": query.pagesize.unwrap_or(20),
        "lasttime": query.lasttime.unwrap_or(-1),
    });
    create_request(
      "/api/v1/event/get",
      data,
      create_request_option(extract_headers!(req), &query.common, "weapi"),
    ).await
  }
}
cache_handler!(event, Event);



// // 动态
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     pagesize: query.pagesize || 20,
//     lasttime: query.lasttime || -1,
//   }
//   return request(`/api/v1/event/get`, data, createOption(query, 'weapi'))
// }