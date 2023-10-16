use errors::core::{mappers::db_error_mapper, Error};
use sea_orm::{ConnectionTrait, DatabaseTransaction, TransactionTrait};

pub struct Database;

impl Database {
    pub async fn begin<C: ConnectionTrait + TransactionTrait>(
        db: &C,
    ) -> Result<DatabaseTransaction, Error> {
        Ok(db.begin().await.map_err(db_error_mapper)?)
    }

    pub async fn commit(txn: DatabaseTransaction) -> Result<(), Error> {
        Ok(txn.commit().await.map_err(db_error_mapper)?)
    }
}
