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

// 视频详情
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/video/detail").route(web::get().to(video_detail)));
}

// 入参
define_request_struct!(VideoDetail, {
    id: String,
});

impl VideoDetail {
    async fn requests(req: HttpRequest, query: Query<VideoDetail>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
        });
        create_request(
            "/api/cloudvideo/v1/video/detail",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}

cache_handler!(video_detail, VideoDetail);

// // 视频详情
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//   }
//   return request(
//     `/api/cloudvideo/v1/video/detail`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
