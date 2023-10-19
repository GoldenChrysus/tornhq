//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use super::sea_orm_active_enums::ApprovalStatus;
use super::sea_orm_active_enums::LossContractType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "loss_contracts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub slug: Uuid,
    pub attacker_id: i32,
    pub defender_id: i32,
    pub r#type: LossContractType,
    pub price: Decimal,
    pub hit_limit: Option<i32>,
    pub end_date: Option<DateTime>,
    pub terminated: Option<bool>,
    pub attacker_approval: ApprovalStatus,
    pub defender_approval: ApprovalStatus,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::loss_logs::Entity")]
    LossLogs,
}

impl Related<super::loss_logs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LossLogs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
