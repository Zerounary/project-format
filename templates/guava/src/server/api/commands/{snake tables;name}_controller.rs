
use crate::{
    server::auth::GuavaSession,
    server::api::model::{{snake this.name}}_vo::{{upperCamel this.name}}VO,
    server::api::model::{{snake this.name}}_opt_vo::{{upperCamel this.name}}OptionVO,
    server::api::model::{{snake this.name}}_create_vo::Create{{upperCamel this.name}}VO,
    server::api::model::{{snake this.name}}_update_vo::{Update{{upperCamel this.name}}VO, Update{{upperCamel this.name}}OptionVO},
    service::{{snake this.name}}_service::{Create{{upperCamel this.name}}Input, Update{{upperCamel this.name}}Input, Update{{upperCamel this.name}}OptionInput}
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};
use itertools::Itertools;

use super::{AppResult, Resp, State, PageParams, AppListResult};

pub async fn find_{{snake this.name}}_by_id(
    Path(id): Path<i64>,
{{#if auth}}
    GuavaSession::FoundUser((_user, service)): GuavaSession,
{{else}}
    Extension(service): State,
{{/if}}
) -> AppResult<{{upperCamel this.name}}VO> {
    let res = service.find_{{snake this.name}}_by_id(id).await?;
    let mut vo: {{upperCamel this.name}}VO = res.into();
    Resp::ok(vo)
}
// read!(find_{{snake this.name}}_by_id > {{upperCamel this.name}}VO);

pub async fn find_{{snake this.name}}_by_id_no_cache(
    Path(id): Path<i64>,
{{#if auth}}
    GuavaSession::FoundUser((_user, service)): GuavaSession,
{{else}}
    Extension(service): State,
{{/if}}
) -> AppResult<{{upperCamel this.name}}VO> {
    let res = service.find_{{snake this.name}}_by_id_no_cache(id).await?;
    let mut vo: {{upperCamel this.name}}VO = res.into();
    Resp::ok(vo)
}
// read!(find_{{snake this.name}}_by_id_no_cache > {{upperCamel this.name}}VO);

pub async fn find_{{snake this.name}}_list(
    Json(params): Json<{{upperCamel this.name}}OptionVO>,
{{#if auth}}
    GuavaSession::FoundUser((_user, service)): GuavaSession,
{{else}}
    Extension(service): State,
{{/if}}
) -> AppListResult<{{upperCamel this.name}}VO> {
    let result = service.find_{{snake this.name}}_list(params.into()).await?;
    let vos = result.into_iter().map(|x| x.into()).collect_vec();
    Resp::page(None, vos)
}
// read!({{upperCamel this.name}}OptionVO > find_{{snake this.name}}_list > Vec<{{upperCamel this.name}}VO>);

pub async fn find_{{snake this.name}}_page(
    Query(page_params): Query<PageParams>,
    Json(params): Json<{{upperCamel this.name}}OptionVO>,
{{#if auth}}
    GuavaSession::FoundUser((_user, service)): GuavaSession,
{{else}}
    Extension(service): State,
{{/if}}
) -> AppListResult<{{upperCamel this.name}}VO> {
    let page_num = page_params.page.unwrap_or(1);
    let page_size = page_params.perPage.unwrap_or(10);
    let total = service.count_{{snake this.name}}().await.ok();
    let result = service.find_{{snake this.name}}_page(params.into(), page_num, page_size).await?;
    let vos = result.into_iter().map(|x| x.into()).collect_vec();
    Resp::page(total, vos)
}

pub async fn create_{{snake this.name}}(
    Json(params): Json<Create{{upperCamel this.name}}VO>,
{{#if auth}}
    GuavaSession::FoundUser((_user, service)): GuavaSession,
{{else}}
    Extension(service): State,
{{/if}}
) -> AppResult<i64> {
    let service_input: Create{{upperCamel this.name}}Input = params.into();
    let id = service.create_{{snake this.name}}(service_input).await?;
    Resp::ok(id)
}
// create!(Create{{upperCamel this.name}}VO > create_{{snake this.name}}(Create{{upperCamel this.name}}Input)  > {{upperCamel this.name}}VO);

pub async fn create_{{snake this.name}}_batch(
    Json(params): Json<Vec<Create{{upperCamel this.name}}VO>>,
{{#if auth}}
    GuavaSession::FoundUser((_user, service)): GuavaSession,
{{else}}
    Extension(service): State,
{{/if}}
) -> AppResult<Vec<String>> {
    let service_input: Vec<Create{{upperCamel this.name}}Input> = params.into_iter().map(|x| x.into()).collect();
    let service_result = service.create_{{snake this.name}}_batch(service_input).await?;
    let result = service_result.into_iter().map(|x| x.to_string()).collect_vec();
    Resp::ok(result)
}
// create!(Vec<Create{{upperCamel this.name}}VO> > create_{{snake this.name}}_batch(Vec<Create{{upperCamel this.name}}Input>) > Vec<String>);

pub async fn update_{{snake this.name}}(
    Path(id): Path<i64>,
{{#if auth}}
    GuavaSession::FoundUser((_user, service)): GuavaSession,
{{else}}
    Extension(service): State,
{{/if}}
    Json(mut params): Json<Update{{upperCamel this.name}}VO>,
) -> AppResult<bool> {
    params.id = Some(id);
    let service_input: Update{{upperCamel this.name}}Input = params.into();
    service.update_{{snake this.name}}(service_input).await?;
    Resp::ok(true)
}
// update!(Update{{upperCamel this.name}}VO -> update_{{snake this.name}}(Update{{upperCamel this.name}}Input) -> {{upperCamel this.name}}VO);

pub async fn update_{{snake this.name}}_opt(
    Path(id): Path<i64>,
{{#if auth}}
    GuavaSession::FoundUser((_user, service)): GuavaSession,
{{else}}
    Extension(service): State,
{{/if}}
    Json(mut params): Json<Update{{upperCamel this.name}}OptionVO>,
) -> AppResult<bool> {
    params.id = Some(id);
    let service_input: Update{{upperCamel this.name}}OptionInput = params.into();
    service.update_{{snake this.name}}_opt(service_input).await?;
    Resp::ok(true)
}

pub async fn delete_{{snake this.name}}_ids(
    Path(ids): Path<String>,
{{#if auth}}
    GuavaSession::FoundUser((_user, service)): GuavaSession,
{{else}}
    Extension(service): State,
{{/if}}
) -> impl IntoResponse {
    let ids: Vec<i64> = ids.split(",").into_iter().map(|x| x.trim().parse().unwrap_or(-1)).collect();
    match service.delete_{{snake this.name}}_ids(ids).await {
        Ok(_) => StatusCode::OK,
        Err(_e) => StatusCode::NOT_FOUND,
    }
}
// delete!(delete_{{snake this.name}}_ids);


