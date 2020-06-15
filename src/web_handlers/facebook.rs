use actix_web::{
    error, web, Error, HttpResponse
};
use actix_web::http::StatusCode;

use anyhow::Result;

use crate::db::PgPool;
use crate::messenger;
use crate::models::web::AppState;
use crate::models::document::NewDocument;
use crate::errors::SafError::BadObjectError;

use serde_json::{json, to_string_pretty};

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

    if event.object == "page" {

        let urls = crate::messenger::parse_document(event.0);

            /*
        let urls = match crate::messenger::parse_document(event.0) {
            Ok(urls) => {
                let files:Vec<Result<String,Box<dyn std::error::Error>>> =
                    crate::core::download_files(&urls, (&data).download_path.as_path()).await;
                files
            },
            Err(_) => vec![]
        };


        let documents = 
            urls
                .iter()
                .map(
                // Map over the Vec of destination files
                    |result| result.as_ref().map(
                        // Convert Result<String,_> into Result<Document,_>
                        |download_path| NewDocument::new(download_path.to_owned(), Some("Test description".to_owned()))
                    ).map(|new_doc| new_doc.create(&pg_pool))
                )
                .collect::<Vec<_>>();
            */

        //Err(error::ErrorBadRequest("Unable to parse event"))
        /*
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
        */
    } else {
        Err(BadObjectError{
            object: 
                format!("Bad object type. Expected 'page' got {}", 
                        event.object)})?
    }


    let json = json!({ "status": "Everything's fine baby" });

    Ok(HttpResponse::Ok().json(json))
}
