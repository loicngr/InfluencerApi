use crate::api_error::ApiError;

use actix_web::{get, web, HttpResponse};
use std::fs;

async fn read_file(filename: String) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

#[get("dreamsbird/twitch/stream")]
async fn get_twitch() -> Result<HttpResponse, ApiError> {
    let file_data = read_file("dreamsbird_twitch.json".to_owned()).await;
    let serialized: serde_json::Value = serde_json::from_str(&file_data.as_str()).unwrap();

    Ok(HttpResponse::Ok().json(serialized))
}

#[get("/dreamsbird/youtube/lastvideo")]
async fn get_youtube_lastvideo() -> Result<HttpResponse, ApiError> {
    let file_data = read_file("dreamsbird_youtube.json".to_owned()).await;
    let serialized: serde_json::Value = serde_json::from_str(&file_data.as_str()).unwrap();

    Ok(HttpResponse::Ok().json(serialized))
}

#[get("/dreamsbird/youtube")]
async fn get_youtube_racine() -> Result<HttpResponse, ApiError> {
    let error = ApiError::new(404, "Erreur".to_owned());
    Ok(HttpResponse::Ok().json(error))
}

#[get("/dreamsbird/twitch")]
async fn get_twitch_racine() -> Result<HttpResponse, ApiError> {
    let error = ApiError::new(404, "Erreur".to_owned());
    Ok(HttpResponse::Ok().json(error))
}

#[get("/dreamsbird")]
async fn get_racine() -> Result<HttpResponse, ApiError> {
    let error = ApiError::new(404, "Erreur".to_owned());
    Ok(HttpResponse::Ok().json(error))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_racine);
    cfg.service(get_twitch_racine);
    cfg.service(get_youtube_racine);
    cfg.service(get_youtube_lastvideo);
    cfg.service(get_twitch);
}