use actix_web::{
    error, web, Error, HttpRequest, HttpResponse
};
use crate::db::PgPool;
use crate::models::document::{DocumentList};

pub async fn index(_req: HttpRequest, pg_pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let documents= 
        DocumentList::list(&pg_pool, None)
            .map_err(|e| {
                error!("Database error {}", e.to_string());
                error::ErrorInternalServerError("Database error.")
            })?;

    Ok(HttpResponse::Ok().json(documents))
}

