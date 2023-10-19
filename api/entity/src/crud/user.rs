use super::super::entities::users::{ActiveModel, Column, Entity, Model};
use crate::{
    deserializers::deserialize_option_option_number_from_string, traits::crud::CRUD,
    utils::EntityNames,
};
use database::Database;
use errors::core::{make_error, mappers::db_error_mapper, Error, ErrorCodes, ErrorStatuses};
use sea_orm::{
    prelude::{async_trait::async_trait, Decimal},
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, ConnectionTrait, EntityTrait,
    ModelTrait, QueryFilter, TransactionTrait, TryIntoModel,
};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct UpsertModel {
    pub id: Option<i32>,
    pub allow_defend_contract_requests: Option<bool>,
    pub allow_auto_defend_contract_link: Option<bool>,
    pub allow_auto_defend_contract_log: Option<bool>,
    #[serde(
        default,
        deserialize_with = "deserialize_option_option_number_from_string"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_defend_contract_duration: Option<Option<i32>>,
    #[serde(
        default,
        deserialize_with = "deserialize_option_option_number_from_string"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_defend_contract_hit_limit: Option<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_defend_contract_price: Option<Decimal>,
    pub allow_loss_contract_requests: Option<bool>,
    pub allow_auto_loss_contract_link: Option<bool>,
    pub allow_auto_loss_contract_log: Option<bool>,
    #[serde(
        default,
        deserialize_with = "deserialize_option_option_number_from_string"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_loss_contract_duration: Option<Option<i32>>,
    #[serde(
        default,
        deserialize_with = "deserialize_option_option_number_from_string"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_loss_contract_hit_limit: Option<Option<i32>>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub auto_loss_contract_price: Option<Decimal>,
}

impl sea_orm::IntoActiveModel<ActiveModel> for UpsertModel {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: if self.id.is_some() {
                ActiveValue::Set(self.id.unwrap())
            } else {
                ActiveValue::NotSet
            },
            torn_api_key: ActiveValue::NotSet,
            allow_defend_contract_requests: if self.allow_defend_contract_requests.is_some() {
                ActiveValue::Set(self.allow_defend_contract_requests.unwrap())
            } else {
                ActiveValue::NotSet
            },
            allow_auto_defend_contract_link: if self.allow_auto_defend_contract_link.is_some() {
                ActiveValue::Set(self.allow_auto_defend_contract_link.unwrap())
            } else {
                ActiveValue::NotSet
            },
            allow_auto_defend_contract_log: if self.allow_auto_defend_contract_log.is_some() {
                ActiveValue::Set(self.allow_auto_defend_contract_log.unwrap())
            } else {
                ActiveValue::NotSet
            },
            auto_defend_contract_duration: if self.auto_defend_contract_duration.is_some() {
                ActiveValue::Set(self.auto_defend_contract_duration.unwrap())
            } else {
                ActiveValue::NotSet
            },
            auto_defend_contract_hit_limit: if self.auto_defend_contract_hit_limit.is_some() {
                ActiveValue::Set(self.auto_defend_contract_hit_limit.unwrap())
            } else {
                ActiveValue::NotSet
            },
            auto_defend_contract_price: if self.auto_defend_contract_price.is_some() {
                ActiveValue::Set(self.auto_defend_contract_price.unwrap())
            } else {
                ActiveValue::NotSet
            },
            allow_loss_contract_requests: if self.allow_loss_contract_requests.is_some() {
                ActiveValue::Set(self.allow_loss_contract_requests.unwrap())
            } else {
                ActiveValue::NotSet
            },
            allow_auto_loss_contract_link: if self.allow_auto_loss_contract_link.is_some() {
                ActiveValue::Set(self.allow_auto_loss_contract_link.unwrap())
            } else {
                ActiveValue::NotSet
            },
            allow_auto_loss_contract_log: if self.allow_auto_loss_contract_log.is_some() {
                ActiveValue::Set(self.allow_auto_loss_contract_log.unwrap())
            } else {
                ActiveValue::NotSet
            },
            auto_loss_contract_duration: if self.auto_loss_contract_duration.is_some() {
                ActiveValue::Set(self.auto_loss_contract_duration.unwrap())
            } else {
                ActiveValue::NotSet
            },
            auto_loss_contract_hit_limit: if self.auto_loss_contract_hit_limit.is_some() {
                ActiveValue::Set(self.auto_loss_contract_hit_limit.unwrap())
            } else {
                ActiveValue::NotSet
            },
            auto_loss_contract_price: if self.auto_loss_contract_price.is_some() {
                ActiveValue::Set(self.auto_loss_contract_price.unwrap())
            } else {
                ActiveValue::NotSet
            },
        }
    }
}

#[async_trait]
impl CRUD<Model, ActiveModel> for UpsertModel {
    type PrimaryKeyType = i32;

    fn invalid_client_columns(
    ) -> Vec<<<ActiveModel as ActiveModelTrait>::Entity as EntityTrait>::Column> {
        vec![]
    }

    fn upsert_id(&self) -> Option<Self::PrimaryKeyType> {
        self.id.to_owned()
    }

    async fn delete<C: ConnectionTrait + TransactionTrait>(
        id: Self::PrimaryKeyType,
        current_user: &Model,
        db: &C,
    ) -> Result<(), Error> {
        let model = Entity::find_by_id(id)
            .filter(Condition::all().add(Column::Id.eq(current_user.id)))
            .one(db)
            .await
            .map_err(db_error_mapper)?
            .ok_or(make_error(ErrorStatuses::NotFound {
                detail: EntityNames::User.to_string(),
            }))?;
        let txn = Database::begin(db).await?;

        model.delete(&txn).await.map_err(db_error_mapper)?;
        Database::commit(txn).await?;
        Ok(())
    }

    async fn on_upsert<C: ConnectionTrait + TransactionTrait>(
        &self,
        active_model: ActiveModel,
        current_user: &Model,
        db: &C,
        insert: bool,
    ) -> Result<Model, Error> {
        if !insert {
            if active_model.id.is_not_set() {
                Err(make_error(ErrorStatuses::ValueError {
                    field: "id".to_string(),
                    reason: ErrorCodes::FieldMissing.to_string(),
                }))?
            }

            Entity::find_by_id(active_model.id.as_ref().to_owned())
                .filter(Condition::all().add(Column::Id.eq(current_user.id)))
                .one(db)
                .await
                .map_err(db_error_mapper)?
                .ok_or(make_error(ErrorStatuses::NotFound {
                    detail: EntityNames::User.to_string(),
                }))?;
        }

        let txn = Database::begin(db).await?;
        let res = active_model
            .save(&txn)
            .await
            .map_err(db_error_mapper)?
            .try_into_model()
            .map_err(db_error_mapper)?;

        Database::commit(txn).await?;
        Ok(res)
    }
}
