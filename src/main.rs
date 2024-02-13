
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get,post},
    Json,Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;


#[tokio::main]
async fn main(){
    //loggingの初期化
    let log_level = env::var("RUST_LOG").unwrap_or("into".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
        
    let app = Router::new()
    .route("/",get(root))
    .route("/users",post(create_user));

    let addr =SocketAddr::from(([0 , 0 , 0 , 0 ],3000));
    tracing::debug!("liseting on {}",addr);

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();

}

async fn root() -> &'static str{
    "Hello world"
}

async fn create_user(
    //ここでdeserialize
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = User{
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser{
    username: String,
}
#[derive(Deserialize)]
struct User{
    id: u64,
    username: String,
}