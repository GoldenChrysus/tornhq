use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum LossContractType {
    Table,
    #[iden = "loss"]
    Loss,
    #[iden = "escape"]
    Escape,
}

#[derive(Iden)]
pub enum ApprovalStatus {
    Table,
    #[iden = "pending"]
    Pending,
    #[iden = "denied"]
    Denied,
    #[iden = "bypassed"]
    Bypassed,
    #[iden = "approved"]
    Approved,
}
