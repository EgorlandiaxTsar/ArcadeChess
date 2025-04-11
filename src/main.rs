mod controller;
mod database;
mod dto;
mod service;
mod util;

use actix_web::{guard, web, App, HttpResponse, HttpServer};
use controller::{r#static, template, user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(r#static::assets_provider)
            .service(template::index)
            .service(template::profile)
            .service(template::self_profile)
            .service(template::archive)
            .service(template::game)
            .service(template::active_games)
            .service(template::spectate)
            .service(template::play)
            .service(user::create)
            .service(user::fetch)
            .service(user::patch)
            .service(user::delete)
            .service(user::fetch_games)
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(HttpResponse::MethodNotAllowed),
            )
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
