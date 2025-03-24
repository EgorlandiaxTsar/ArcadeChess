use std::path::Path;
use actix_files::{file_extension_to_mime, NamedFile};
use actix_web::{get, Error, HttpRequest, HttpResponse};
use actix_web::http::header::CONTENT_TYPE;

#[get("/static/{filename:.*}")]
async fn assets_provider(req: HttpRequest) -> actix_web::Result<HttpResponse, Error> {
    let path = format!("resources/static/{}", req.match_info().query("filename"));
    let path = Path::new(&path);
    let mime = path
        .extension()
        .and_then(|ext| ext.to_str())
        .and_then(|ext| Option::from(file_extension_to_mime(ext)))
        .unwrap_or(mime::APPLICATION_OCTET_STREAM);
    let mut file: NamedFile = match NamedFile::open(&path) {
        Ok(file) => file,
        Err(_) => return Ok(HttpResponse::NotFound().body("not found")),
    };
    file = file.set_content_type(mime.clone());
    let mut res = file.into_response(&req);
    res.headers_mut().insert(CONTENT_TYPE, mime.to_string().parse()?);
    Ok(res)
}
