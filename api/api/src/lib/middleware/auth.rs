use crate::{
    lib::errors::{error_mapper, make_error, APIError},
    state::AppState,
};
use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use entity::entities::users::{Entity as User, Model as UserModel};
use errors::core::{mappers::db_error_mapper, ErrorStatuses};
use futures::Future;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

#[derive(Debug, Serialize, Deserialize)]
pub struct JWT {
    pub sub: i32,
    pub iat: i64,
    pub exp: i64,
    pub jti: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticatedUser {
    pub user: UserModel,
}

impl FromRequest for AuthenticatedUser {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output = Result<AuthenticatedUser, APIError>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let cloned_req = req.clone();

        Box::pin(async move {
            let (jwt, state) = parse_request(&cloned_req)?;

            let user = User::find_by_id(jwt.claims.sub)
                .one(&state.postgres)
                .await
                .map_err(db_error_mapper)
                .map_err(error_mapper)?
                .ok_or(make_error(ErrorStatuses::Unauthorized))?;
            let authenticated_user = AuthenticatedUser { user };

            Ok(authenticated_user)
        })
    }
}

fn parse_request(req: &HttpRequest) -> Result<(TokenData<JWT>, &Data<AppState>), APIError> {
    let state_option = req.app_data::<Data<AppState>>();
    let auth_option = req.headers().get("X-Auth");

    if auth_option.is_none() || state_option.is_none() {
        Err(make_error(ErrorStatuses::Unauthorized))?
    }

    let auth = auth_option.unwrap();
    let state = state_option.unwrap();

    match auth.to_str() {
        Ok(val) => {
            let jwt_result = decode::<JWT>(
                val,
                &DecodingKey::from_secret(
                    std::env::var("JWT_SECRET")
                        .expect("JWT_SECRET is not defined.")
                        .as_ref(),
                ),
                &Validation::new(Algorithm::HS512),
            );

            Ok((
                jwt_result.map_err(|_| make_error(ErrorStatuses::Unauthorized))?,
                state,
            ))
        }
        Err(_) => Err(make_error(ErrorStatuses::Unauthorized)),
    }
}
