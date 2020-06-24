use crate::db;
use crate::db::PgPool;
use crate::schema::users;
use chrono::NaiveDateTime; // This type is used for date field in Diesel.

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i64,
    pub email: String,
    #[serde(skip)] // we're removing password from being show in the response
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

use crate::errors::SafError;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Local;

impl User {
    pub fn create(register_user: RegisterUser, pool: &PgPool) -> Result<User, SafError> {
        use diesel::RunQueryDsl;

        Ok(diesel::insert_into(users::table)
            .values(NewUser {
                email: register_user.email,
                password: Self::hash_password(register_user.password)?,
                created_at: Local::now().naive_local(),
            })
            .get_result(&db::get_conn(pool)?)?)
    }

    // This might look kind of weird,
    // but if something fails it would chain
    // to our SafError Error,
    // otherwise it will gives us the hash,
    // we still need to return a result
    // so we wrap it in an Ok variant from the Result type.
    pub fn hash_password(plain: String) -> Result<String, SafError> {
        Ok(hash(plain, DEFAULT_COST)?)
    }
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

impl RegisterUser {
    pub fn validates(self) -> Result<RegisterUser, SafError> {
        if self.password == self.password_confirmation {
            Ok(self)
        } else {
            Err(SafError::PasswordNotMatch(
                "Password and Password Confirmation does not match".to_string(),
            ))
        }
    }
}

#[derive(Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String,
}

impl AuthUser {
    // The good thing about ? syntax and have a custom error is
    // that the code would look very straightforward, I mean,
    // the other way would imply a lot of pattern matching
    // making it look ugly.
    pub fn login(&self, pool: &PgPool) -> Result<User, SafError> {
        use crate::schema::users::dsl::email;
        use bcrypt::verify;
        use diesel::ExpressionMethods;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let mut records = users::table
            .filter(email.eq(&self.email))
            .load::<User>(&db::get_conn(pool)?)?;

        let user = records
            .pop()
            .ok_or(SafError::DBError(diesel::result::Error::NotFound))?;

        let verify_password = verify(&self.password, &user.password).map_err(|_error| {
            SafError::WrongPassword("Wrong password, check again please".to_string())
        })?;

        if verify_password {
            Ok(user)
        } else {
            Err(SafError::WrongPassword(
                "Wrong password, check again please".to_string(),
            ))
        }
    }
}
