use actix_web::HttpResponse;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// We're using a struct so we can implement a conversion from
// Claims to SlimUser, useful in the decode function.
pub struct SlimUser {
    pub email: String,
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser { email: claims.sub }
    }
}

impl Claims {
    fn with_email(email: &str) -> Self {
        Claims {
            sub: email.into(),
            exp: (Local::now() + Duration::hours(24)).timestamp() as usize,
        }
    }
}

pub fn create_token(email: &str, secret: &str) -> Result<String, HttpResponse> {
    let claims = Claims::with_email(email);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn decode_token(token: &str, secret: &str) -> Result<SlimUser, HttpResponse> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims.into())
    .map_err(|e| HttpResponse::Unauthorized().json(e.to_string()))
}
