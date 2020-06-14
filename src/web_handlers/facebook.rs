use actix_web::{
    error, web, Error, HttpResponse
};

use crate::messenger;
use crate::models::web::AppState;

#[derive(Debug, Deserialize)]
pub struct Hub {

    #[serde(rename(deserialize = "hub.mode"))]
    mode: String,
    #[serde(rename(deserialize = "hub.verify_token"))]
    verify_token: String,
    #[serde(rename(deserialize = "hub.challenge"))]
    challenge: String,
}

pub async fn fb_webhook_hub(hub: web::Query<Hub>, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    if hub.mode == "subscribe" && hub.verify_token == (&data).fb_verify_token {
        Ok(HttpResponse::Ok().body(&hub.challenge))
    } else {
        Err(error::ErrorForbidden("Bad token"))
    }

}


pub async fn fb_webhook_event(
    event: web::Json<messenger::Event>,
    data: web::Data<AppState>) -> Result<HttpResponse, Error> {

    if event.object == "page" {
        match messenger::parse_document(event.0) {
            Ok(urls) => {

                debug!("Got urls to download {:#?}", urls);
                let files = crate::core::download_files(&urls, (&data).download_path.as_path()).await; 
                match files.len() {

                0 => Err(error::ErrorNotFound("No file to download")),
                _ => Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json::object! {"status" => "ok" }.dump())),

                }
            }
            _ => Err(error::ErrorBadRequest("Unable to parse event"))
        }
    } else {
        Err(error::ErrorBadRequest(
                format!("Bad object type. Expected 'page' got {}", 
                        event.object)))
    }
}

