use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder, HttpResponse,
};
use serde::Deserialize;
use crate::{
    messages::{CreateArticle, CreateUser, FetchUser, FetchUserArticles, FetchUserById}, schema::articles, AppState, DbActor
};
use actix::Addr;
use crate::schema::users;
use diesel::Insertable;
use serde::Serialize;


#[derive(Deserialize)]
struct CreateUserBody{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[get("/test")]
pub async fn test() -> impl Responder {
    format!("Hello test")
    // api_response::ApiResponse::new(200, "test".to_string())
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUser).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}

#[get("/users/{id}")]
pub async fn fetch_user_by_id(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUserById{user_id: id}).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No user found for this id: {id}")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve user articles"),
    }
}

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[post("/add-user")]
pub async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(CreateUser {
        id: body.id,
        first_name: body.first_name.to_string(),
        last_name:body.last_name.to_string() 
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}


#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=articles)]
pub struct NewArticle {
  pub title: String,
  pub content: String,
  pub created_by: i32,
}

#[post("/users/{id}/articles")]
pub async fn create_user_article(state: Data<AppState>, path: Path<i32>, body: Json<CreateArticleBody>) -> impl Responder {
    let id: i32 = path.into_inner();
    // format!("POST /users/{id}/articles")

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(CreateArticle {
        title: body.title.to_string(),
        content: body.content.to_string(),
        created_by: id
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create article"),
    }
}


#[get("/users/{id}/articles")]
pub async fn fetch_user_articles(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUserArticles { user_id: id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No articles for user {id}")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve user articles"),
    }
}