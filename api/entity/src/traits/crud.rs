use super::super::entities::users::Model as UserModel;
use errors::core::{make_error, Error, ErrorCodes, ErrorStatuses};
use sea_orm::{
    prelude::async_trait::async_trait, ActiveModelTrait, ConnectionTrait, EntityTrait,
    IntoActiveModel, ModelTrait, TransactionTrait,
};

#[async_trait]
pub trait CRUD<T, A>
where
    T: ModelTrait + std::marker::Send + std::marker::Sync,
    A: ActiveModelTrait + std::marker::Send + std::marker::Sync,
    Self: IntoActiveModel<A> + std::marker::Send + std::marker::Sync + Clone,
{
    type PrimaryKeyType: Eq + std::marker::Send + std::marker::Sync;

    async fn delete<C: ConnectionTrait + TransactionTrait>(
        id: Self::PrimaryKeyType,
        current_user: &UserModel,
        db: &C,
    ) -> Result<(), Error>;
    async fn on_upsert<C: ConnectionTrait + TransactionTrait>(
        &self,
        mut active_model: A,
        current_user: &UserModel,
        db: &C,
        insert: bool,
    ) -> Result<T, Error>;
    fn invalid_client_columns() -> Vec<<A::Entity as EntityTrait>::Column>;
    fn upsert_id(&self) -> Option<Self::PrimaryKeyType>;

    async fn upsert<C: ConnectionTrait + TransactionTrait>(
        &self,
        current_user: &UserModel,
        db: &C,
        path_id: Option<Self::PrimaryKeyType>,
        insert: bool,
        client: bool,
    ) -> Result<T, Error>
    where
        Self::PrimaryKeyType: 'async_trait,
    {
        Self::validate_ids(self.upsert_id(), path_id, insert)?;

        let active_model = self.to_owned().into_active_model();

        if client {
            for column in Self::invalid_client_columns().iter().clone() {
                if active_model.get(*column).is_set() {
                    Err(make_error(ErrorStatuses::ValueError {
                        field: format!("{:#?}", column),
                        reason: ErrorCodes::FieldInvalid.to_string(),
                    }))?
                }
            }
        }

        self.on_upsert(active_model, current_user, db, insert).await
    }

    fn validate_ids(
        data_id: Option<Self::PrimaryKeyType>,
        path_id: Option<Self::PrimaryKeyType>,
        insert: bool,
    ) -> Result<(), Error> {
        if insert {
            if path_id.is_some() || data_id.is_some() {
                Err(make_error(errors::core::ErrorStatuses::InvalidInput))?
            }
        } else if path_id != data_id {
            Err(make_error(errors::core::ErrorStatuses::InvalidInput))?
        }

        Ok(())
    }

    fn raise_value_error(column: &str, reason: ErrorCodes) -> Result<(), Error> {
        Err(make_error(ErrorStatuses::ValueError {
            field: column.to_string(),
            reason: reason.to_string(),
        }))?
    }
}
