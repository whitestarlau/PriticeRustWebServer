use std::f32::consts::E;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Html,
    Json,
};


use tracing::{info, instrument};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{models::{user::{RegUser, RegUserResp}, error::internal_error, state::AppState}, utils::{validate_payload, jwt}, db_access::db::add_new_user_from_db};

#[instrument]
pub async fn health_handler() -> Html<&'static str> {
    println!("some one call health check api.");
    Html("<h1>Certify server health ok.</h1>")
}

#[instrument]
pub async fn sign_up(
    Json(user): Json<RegUser>,
    State(state): State<AppState>,
) -> Result<axum::Json<RegUserResp>, (StatusCode, String)> {
    validate_payload(&user).map_err(internal_error)?;
    let addResultId = add_new_user_from_db(&state.pool, user).await?;
    
    let token = jwt::sign(addResultId).map_err(internal_error)?;
    
    return Ok(());
}



#[instrument]
pub async fn sign_in(
    
) -> Html<&'static str> {
    println!("some one call health check api.");
    Html("<h1>Certify server health ok.</h1>")
}



#[instrument]
pub async fn verify(
    
) -> Html<&'static str> {
    println!("some one call health check api.");
    Html("<h1>Certify server health ok.</h1>")
}



pub fn map_ok_result<T>(r: T) -> axum::Json<T> {
    axum::Json(r)
}

pub fn map_consult_error(err: reqwest::Error) -> (StatusCode, String) {
    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        "consul error.".to_string(),
    );
}
