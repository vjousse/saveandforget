use crate::db::PgPool;
use crate::errors::SafError;
use crate::utils::jwt::create_token;
use actix_identity::Identity;
use actix_web::web;
use actix_web::HttpResponse;
use csrf_token::CsrfTokenGenerator;
use hex;

use crate::models::user::{AuthUser, RegisterUser, User};

// We get a new connection pool, validates the data,
// `password` and `password_confirmation` should be the same,
// finally we create the user and return it.
pub fn register(
    new_user: web::Json<RegisterUser>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let register_user = new_user
        .into_inner()
        .validates()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))?;
    User::create(register_user, &pg_pool)
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

// We get a new connection pool, then look up for the user,
// If there is no user a NotFound error would raise otherwise
// this would just through an InternalServerError.
pub fn login(
    auth_user: web::Json<AuthUser>,
    id: Identity,
    pg_pool: web::Data<PgPool>,
    generator: web::Data<CsrfTokenGenerator>,
) -> Result<HttpResponse, HttpResponse> {
    let user = auth_user.login(&pg_pool).map_err(|e| match e {
        SafError::DBError(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().json(e.to_string())
        }
        _ => HttpResponse::InternalServerError().json(e.to_string()),
    })?;

    // This is the jwt token we will send in a cookie.
    let token = create_token(&user.email, "secret")?;

    id.remember(token);

    // Finally our response will have a csrf token for security.
    let response = HttpResponse::Ok()
        .header("X-CSRF-TOKEN", hex::encode(generator.generate()))
        .json(user);
    Ok(response)
}

pub fn logout(id: Identity) -> Result<HttpResponse, HttpResponse> {
    id.forget();
    Ok(HttpResponse::Ok().into())
}
