use crate::{
    lib::{
        errors::{error_mapper, make_error, APIError},
        http::make_json_response,
        middleware::auth::JWT,
    },
    state::AppState,
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Result,
};
use entity::entities::users::{ActiveModel as ActiveUser, Entity as User};
use errors::core::{mappers::db_error_mapper, ErrorStatuses};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    api_key: String,
}

#[post("/login")]
pub async fn login(
    state: Data<AppState>,
    req: Json<LoginRequest>,
) -> Result<HttpResponse, APIError> {
    let api_key = req.0.api_key;

    // TODO: API key lookup to get user ID and key info

    let torn_id = 1962321;
    let user = User::find_by_id(torn_id)
        .one(&state.postgres)
        .await
        .map_err(db_error_mapper)
        .map_err(error_mapper)?;

    if user.is_none() {
        let active_model = ActiveUser {
            id: ActiveValue::Set(torn_id), // TODO: Encrypt this
            torn_api_key: ActiveValue::Set(api_key),
            allow_auto_defend_contract_link: ActiveValue::NotSet,
            allow_auto_defend_contract_log: ActiveValue::NotSet,
            allow_defend_contract_requests: ActiveValue::NotSet,
            auto_defend_contract_duration: ActiveValue::NotSet,
            auto_defend_contract_hit_limit: ActiveValue::NotSet,
            auto_defend_contract_price: ActiveValue::NotSet,
            allow_auto_loss_contract_link: ActiveValue::NotSet,
            allow_auto_loss_contract_log: ActiveValue::NotSet,
            allow_loss_contract_requests: ActiveValue::NotSet,
            auto_loss_contract_duration: ActiveValue::NotSet,
            auto_loss_contract_hit_limit: ActiveValue::NotSet,
            auto_loss_contract_price: ActiveValue::NotSet,
        };

        active_model
            .save(&state.postgres)
            .await
            .map_err(db_error_mapper)
            .map_err(error_mapper)?;
    } else {
        let user = user.unwrap();

        if user.torn_api_key != api_key {
            let mut active_model = user.into_active_model();

            active_model.torn_api_key = ActiveValue::Set(api_key); // TODO: Encrypt this

            active_model
                .save(&state.postgres)
                .await
                .map_err(db_error_mapper)
                .map_err(error_mapper)?;
        }
    }

    let now = chrono::offset::Utc::now().timestamp();
    let jwt = JWT {
        sub: torn_id,
        iat: now,
        exp: now + (60 * 60 * 24 * 30),
        jti: "".to_string(),
    };
    let jwt = encode::<JWT>(
        &Header::new(Algorithm::HS512),
        &jwt,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    )
    .map_err(|_| make_error(ErrorStatuses::Unauthorized))?;

    Ok(make_json_response(jwt))
}
