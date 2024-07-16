#![allow(unused)]

pub use self::error::{Error,Result};

use axum::Router;
use axum::routing::get;
use axum::response::Html;
use std::net::SocketAddr;
use axum::response::IntoResponse;
use serde::Deserialize;
use axum::extract::Query;
use axum::extract::Path;
use tower_http::services::ServeDir;
use axum::routing::get_service;
use serde_json::json;

mod error;
mod web;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}


#[tokio::main]
async fn main (){
    let route_all_merged = Router::new()
    .merge(routes_hello_all())
    .merge(web::routes_login::routes())
    .fallback_service(routes_static())
    ;
    

    let addr = SocketAddr::from(([127,0,0,1],8080));
    println!("-->> server is listening on {addr} \n");
    axum::Server::bind(&addr)
        .serve(route_all_merged.into_make_service())
        .await
        .unwrap()
}

fn routes_static() -> Router {
    Router::new()
        .nest_service("/", get_service(ServeDir::new("./")))
}


fn routes_hello_all()-> Router {
    Router::new()
    .route("/hello", get(handler_hello))
    .route("/hello/:name",get(handler_hello_params_in_route))
    
}

// /hello?name=xxx
async fn handler_hello(Query(params):Query<HelloParams>) ->impl IntoResponse{
    println!(":->> {:<12} - handler_hello - for {params:?} ","HANDLER");
    
    let name = params.name.as_deref().unwrap_or("World");

    Html(format!("Hello <strong> {name}!!!</strong><br/> sent by ilman"))
}


// /hello?name=xxx
async fn handler_hello_params_in_route(Path(name):Path<String>)-> impl IntoResponse {
    println!(":->> {:<12} - handler_hello - for {name:?} ","HANDLER");
    
    // let name = name.as_deref().unwrap_or("World");

    Html(format!("Hello <strong> {name}!!!</strong><br/> sent by ilman"))



}
 