use crate::api_error::ApiError;

use actix_web::{get, web, HttpResponse};
use std::fs;

async fn read_file(filename: String) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

#[get("paranoi4k/twitch/stream")]
async fn get_twitch() -> Result<HttpResponse, ApiError> {
    let file_data = read_file("paranoi4k_twitch.json".to_owned()).await;
    let serialized: serde_json::Value = serde_json::from_str(&file_data.as_str()).unwrap();

    Ok(HttpResponse::Ok().json(serialized))
}

#[get("/paranoi4k/youtube/lastvideo")]
async fn get_youtube_lastvideo() -> Result<HttpResponse, ApiError> {
    let file_data = read_file("paranoi4k_youtube.json".to_owned()).await;
    let serialized: serde_json::Value = serde_json::from_str(&file_data.as_str()).unwrap();

    Ok(HttpResponse::Ok().json(serialized))
}

#[get("/paranoi4k/youtube")]
async fn get_youtube_racine() -> Result<HttpResponse, ApiError> {
    let error = ApiError::new(404, "Erreur".to_owned());
    Ok(HttpResponse::Ok().json(error))
}

#[get("/paranoi4k/twitch")]
async fn get_twitch_racine() -> Result<HttpResponse, ApiError> {
    let error = ApiError::new(404, "Erreur".to_owned());
    Ok(HttpResponse::Ok().json(error))
}

#[get("/paranoi4k")]
async fn get_racine() -> Result<HttpResponse, ApiError> {
    let error = ApiError::new(404, "Erreur".to_owned());
    Ok(HttpResponse::Ok().json(error))
}

#[get("/")]
async fn get_home() -> Result<HttpResponse, ApiError> {
    let error = ApiError::new(404, "Erreur".to_owned());
    Ok(HttpResponse::Ok().json(error))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_home);
    cfg.service(get_racine);
    cfg.service(get_twitch_racine);
    cfg.service(get_youtube_racine);
    cfg.service(get_youtube_lastvideo);
    cfg.service(get_twitch);
}