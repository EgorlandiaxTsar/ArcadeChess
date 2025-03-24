use actix_files::NamedFile;
use actix_web::{get, Result};

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(route("resources/templates/index.html"))
}

#[get("/user/@{user_uuid}")]
async fn profile() -> Result<NamedFile> {
    Ok(route("resources/templates/profile.html"))
}

#[get("/me")]
async fn self_profile() -> Result<NamedFile> {
    Ok(route("resources/templates/me.html"))
}

#[get("/game")]
async fn archive() -> Result<NamedFile> {
    Ok(route("resources/templates/archive.html"))
}

#[get("/game/{game_uuid}")]
async fn game() -> Result<NamedFile> {
    Ok(route("resources/templates/game.html"))
}

#[get("/spectate")]
async fn active_games() -> Result<NamedFile> {
    Ok(route("resources/templates/active.html"))
}

#[get("/spectate/{game_uuid}")]
async fn spectate() -> Result<NamedFile> {
    Ok(route("resources/templates/spectate.html"))
}

#[get("/play/{uuid}")]
async fn play() -> Result<NamedFile> {
    Ok(route("resources/templates/play.html"))
}

fn route(path: &str) -> NamedFile {
    NamedFile::open(path)
        .unwrap_or(NamedFile::open("resources/templates/not-found.html").unwrap())
}
