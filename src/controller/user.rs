use crate::dto::user::{BodyCreateUser, BodyUpdateUser};
use actix_web::web::{Json, Path};
use actix_web::{delete, get, patch, post, HttpResponse};

#[post("/api/v1/account")]
async fn create(body: Json<BodyCreateUser>) -> HttpResponse {
    HttpResponse::Ok().body("Not implemented")
}

#[get("/api/v1/account/{uuid}")]
async fn fetch(path: Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("Not implemented")
}

#[patch("/api/v1/account/{uuid}")]
async fn patch(body: Json<BodyUpdateUser>, path: Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("Not implemented")
}

#[delete("/api/v1/account/{uuid}")]
async fn delete(path: Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("Not implemented")
}

#[get("/api/v1/account/{uuid}/games")]
async fn fetch_games(path: Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("Not implemented")
}
