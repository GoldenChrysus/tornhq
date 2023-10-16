use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentLogs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PaymentLogs::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PaymentLogs::SenderId).integer().not_null())
                    .col(ColumnDef::new(PaymentLogs::RecipientId).integer().not_null())
                    .col(ColumnDef::new(PaymentLogs::Amount).decimal().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PaymentLogs::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum PaymentLogs {
    Table,
    Id,
    SenderId,
    RecipientId,
    Amount,
}
