use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Debug,Deserialize, Serialize)]
pub enum OperatorType {
    GET,
    POST,
    UPDATE,
    DELETE,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "sys_menu")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub path: String,
    pub name: String,
    pub title: String,
    pub domain: String,
    pub icon: String,
    pub state: bool,
    pub parent_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}