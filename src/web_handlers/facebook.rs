use actix_web::{
    error, web, Error, HttpResponse
};

use anyhow::Result;

use crate::db::PgPool;
use crate::messenger;
use crate::models::web::AppState;
use crate::models::document::NewDocument;
use crate::errors::SafError::EventParsingError;

use serde_json::json;

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
    data: web::Data<AppState>,
    pg_pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {

    if event.object != "page" {
        Err(EventParsingError{
            message: 
                format!("Bad object type. Expected 'page' got {}", 
                        event.object)})?
    }

    // Try to get urls from FB event
    let urls:Vec<String> = crate::messenger::parse_document(event.0).map_err(|e| {
        EventParsingError{ message: e.to_string()}
    })?;


    // Download files into destination dir
    let files:Vec<Result<String,Box<dyn std::error::Error>>> =
        crate::core::download_files(&urls, (&data).download_path.as_path()).await;


    // Create a document for each downloaded file
    let documents = 
        files
            .iter()
            .map(
            // Map over the Vec of destination files
                |result| result.as_ref().map(
                    // Convert Result<String,_> into Result<Document,_>
                    |download_path| NewDocument::new(download_path.to_owned(), None)
                ).map(|new_doc| new_doc.create(&pg_pool))
            )
            .collect::<Vec<_>>();


    let json = json!({ "status": format!("Downloaded {} files", documents.len()) });

    Ok(HttpResponse::Ok().json(json))
}
