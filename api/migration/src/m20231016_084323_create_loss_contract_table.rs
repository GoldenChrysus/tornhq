use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

use crate::{
    enums::{ApprovalStatus, LossContractType},
    types::Types,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(LossContractType::Table)
                    .values([LossContractType::Loss, LossContractType::Escape])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(ApprovalStatus::Table)
                    .values([
                        ApprovalStatus::Denied,
                        ApprovalStatus::Approved,
                        ApprovalStatus::Bypassed,
                        ApprovalStatus::Pending,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(LossContracts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LossContracts::Id)
                            .custom(Types::Bigserial)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LossContracts::Slug)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(LossContracts::AttackerId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LossContracts::DefenderId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LossContracts::Type)
                            .enumeration(
                                LossContractType::Table,
                                [LossContractType::Loss, LossContractType::Escape],
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(LossContracts::Price).decimal().not_null())
                    .col(ColumnDef::new(LossContracts::HitLimit).integer().null())
                    .col(ColumnDef::new(LossContracts::EndDate).date_time().null())
                    .col(ColumnDef::new(LossContracts::Terminated).boolean().null())
                    .col(
                        ColumnDef::new(LossContracts::AttackerApproval)
                            .enumeration(
                                ApprovalStatus::Table,
                                [
                                    ApprovalStatus::Pending,
                                    ApprovalStatus::Bypassed,
                                    ApprovalStatus::Approved,
                                    ApprovalStatus::Denied,
                                ],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LossContracts::DefenderApproval)
                            .enumeration(
                                ApprovalStatus::Table,
                                [
                                    ApprovalStatus::Pending,
                                    ApprovalStatus::Bypassed,
                                    ApprovalStatus::Approved,
                                    ApprovalStatus::Denied,
                                ],
                            )
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LossContracts::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(ApprovalStatus::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(LossContractType::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum LossContracts {
    Table,
    Id,
    Slug,
    AttackerId,
    DefenderId,
    Type,
    Price,
    HitLimit,
    EndDate,
    Terminated,
    AttackerApproval,
    DefenderApproval,
}
