pub use sea_orm_migration::prelude::*;

pub mod enums;
pub mod types;

mod m20220101_000001_create_users_table;
mod m20231016_084323_create_loss_contract_table;
mod m20231016_085512_create_loss_log_table;
mod m20231016_085845_create_payment_log_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20231016_084323_create_loss_contract_table::Migration),
            Box::new(m20231016_085512_create_loss_log_table::Migration),
            Box::new(m20231016_085845_create_payment_log_table::Migration),
        ]
    }
}
