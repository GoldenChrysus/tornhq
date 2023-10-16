use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LossLogs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LossLogs::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LossLogs::LossContractId).big_integer().not_null())
                    .col(ColumnDef::new(LossLogs::AttackerId).integer().not_null())
                    .col(ColumnDef::new(LossLogs::DefenderId).integer().not_null())
                    .col(ColumnDef::new(LossLogs::LogTime).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-loss_logs__contract_id")
                            .from(LossLogs::Table, LossLogs::LossContractId)
                            .to(LossContracts::Table, LossContracts::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LossLogs::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum LossLogs {
    Table,
    Id,
    LossContractId,
    AttackerId,
    DefenderId,
    LogTime,
}

#[derive(Iden)]
enum LossContracts {
    Table,
    Id,
}
