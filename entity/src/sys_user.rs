use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub account: String,
    pub password: String,
    pub name: String,
    pub domain: String,
    pub avatar: String,
    pub email: String,
    pub phone: String,
    pub pw_reset_count: i32,
    pub state: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}