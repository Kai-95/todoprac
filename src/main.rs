use anyhow::Context;
use axum::{
    extact::Extension, extract::Extension, http::StatusCode, response::IntoResponse, routing::{get,post}, Json, Router
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap, env, fs::OpenOptions, process::ExitCode, sync::{Arc,RwLock}
};
use std::net::SocketAddr;
use thiserror::Error;

type TodoDatas = HashMap<i32, Todo>;

pub struct TodoRepositoryForMemory{
    store: Arc<RwLock<TodoDatas>>,
}

fn create(&self, payload: CreateTodo)->{
    todo!();
}

fn find(&self, id:i32)-> Option<Todo>{
    todo!();
}
fn all(&self)-> Vec<Todo>{
    todo!();
}

fn update(&self, id:i32,payload: UpdateTodo)-> anyhow::Result<Todo>{
    todo!();
}

fn delete(&self ,id:i32)-> anyhow::Result<()>{
    todo!();

}






#[derive(Debug,Error)]
enum RepositoryError {
    #[error("not foun,id is{0}")]
    NotFound(i32),
    
}

pub trait TodoRepository : Clone + std::marker::Sent + std::marker::Sync + 'static{
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self,id:i32)-> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo)->
    anyhow::Result<Todo>;
    fn delete(&self, id:i32) -> anyhow::Result<()>;
}

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Eq)]
pub struct Todo{
    id:i32,
    text:String,
    completed: bool,
}

#[derive(Debug, Serialize,Deserialize,Clone,PartialEq,Eq)]
pub struct CreateTodo{
    text: String,
}
#[derive(Debug, Serialize,Deserialize,Clone,PartialEq,Eq)]
pub struct UpdateTodo{
    text: Option<String>,
    completed: Option<bool>,
}

impl Todo{
    pub fn new(id;i32, text:String)-> Self{
        Self{
            id,
            text,
            completed:false,
        }
    }
}


#[tokio::main]
async fn main(){
    //loggingの初期化
    let log_level = env::var("RUST_LOG").unwrap_or("into".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
        
    let repsitory = TodoRepositoryForMemory::new();
    let app = create_app(repsitory);

    let addr =SocketAddr::from(([0 , 0 , 0 , 0 ],3000));
    tracing::debug!("liseting on {}",addr);

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();

}
fn create_app<T: TodoRepository>(repostory:T)-> Router {
    Router::new()
    .route("/",get(root))
    .route("/todos", post(create_todo::<T>))
    .layer(Extension(arc::new(repostory)))
}


pub async fn create_todo<T: TodoRepository>(
    Json(payload):Json<createTodo>,
    Extension(repository):Extension<Arc<T>>,
) -> impl IntoResponse{
    let todo - repository.create(payload);
    (StatusCode::CREATED,Json(todo))
}




async fn root() -> &'static str{
    "Hello world"
}

//下記の部分はStructのUserのserializeと関連
//[derive(Deserialize)]の場合動かない
//予想としてSirializeするものがなかったためえらー？
async fn create_user(
    //ここでdeserialize
    Json(payload): Json<CreateUser>
) -> impl IntoResponse {
    let user = User{
        id: 1337,
        username: payload.username,
    };
    //ここでSirialize
    (StatusCode::CREATED, Json(user))
}

#[derive(Serialize,Deserialize,Debug,PartialEq,Eq)]
struct CreateUser{
    username: String,
}
#[derive(Serialize,Deserialize,Debug,PartialEq,Eq)]
struct User{
    id: u64,
    username: String,
}

#[cfg(test)]
mod test{
    use super::*;
    use axum::{
        body::Body, extract::rejection::BodyAlreadyExtracted, http::{header,Method,Request}
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world(){
        let reposutory =TodoRepositoryForMemory::new();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app(reposutory).oneshot(req).await.unwrap();
        let byte = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(byte.to_vec()).unwrap();
        assert_eq!(body,"Hello world");
    }
    #[tokio::test]
    async fn should_return_user_data(){
        let reposutory =TodoRepositoryForMemory::new();
        let req = Request::builder()
        .uri("/users")
        .method(Method::POST)
        .header(header::CONTENT_TYPE,mime::APPLICATION_JSON.as_ref())
        .body(Body::from(r#"{"username":"田中"}"#))
        .unwrap();
        let res = create_app(reposutory).oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("can not conver Uesr instance");
        assert_eq!(
            user,
            User{
                id:1337,
                username:"田中".to_string(),
            }
        );
    }

    
}