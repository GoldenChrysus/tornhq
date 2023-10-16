use sea_orm_migration::{prelude::*, sea_orm::prelude::Decimal};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(Users::TornAPIKey).string().not_null())
                    .col(
                        ColumnDef::new(Users::AllowDefendContractRequests)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::AllowAutoDefendContractLink)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::AllowAutoDefendContractLog)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::AutoDefendContractDuration)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Users::AutoDefendContractHitLimit)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Users::AutoDefendContractPrice)
                            .decimal()
                            .not_null()
                            .default(Decimal::ZERO),
                    )
                    .col(
                        ColumnDef::new(Users::AllowLossContractRequests)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::AllowAutoLossContractLink)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::AllowAutoLossContractLog)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::AutoLossContractDuration)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Users::AutoLossContractHitLimit)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Users::AutoLossContractPrice)
                            .decimal()
                            .not_null()
                            .default(Decimal::ZERO),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Users {
    Table,
    Id,
    TornAPIKey,
    AllowDefendContractRequests,
    AllowAutoDefendContractLink,
    AllowAutoDefendContractLog,
    AutoDefendContractDuration,
    AutoDefendContractHitLimit,
    AutoDefendContractPrice,
    AllowLossContractRequests,
    AllowAutoLossContractLink,
    AllowAutoLossContractLog,
    AutoLossContractDuration,
    AutoLossContractHitLimit,
    AutoLossContractPrice,
}
