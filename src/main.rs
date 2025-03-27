mod controllers;
mod database;
mod util;

use actix_web::{guard, web, App, HttpResponse, HttpServer};
use controllers::{static_controller, templates_controller};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(static_controller::assets_provider)
        .service(templates_controller::index)
        .service(templates_controller::profile)
        .service(templates_controller::self_profile)
        .service(templates_controller::archive)
        .service(templates_controller::game)
        .service(templates_controller::active_games)
        .service(templates_controller::spectate)
        .service(templates_controller::play)
        .default_service(web::route()
            .guard(guard::Not(guard::Get()))
            .to(HttpResponse::MethodNotAllowed)
        )
    )
        .bind(("127.0.0.1", 80))?
        .run()
        .await
}
