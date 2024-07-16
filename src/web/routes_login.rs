use crate::{Error, Result};
use axum::routing::post;
use serde::Deserialize;
use axum::{Json,Router};
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    // todo!()
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "taqi" || payload.pwd != "boneka" {
        return Err(Error::LoginFail);
    }

    let body = Json(json!({
       "result":{
           "success":true
       }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
