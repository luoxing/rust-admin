use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "cipher_slot")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub key_type: u32,
    pub sign_key: Vec<u8>,
    pub enc_key: Vec<u8>,
    pub kek: Vec<u8>,
    pub sign_cert: Vec<u8>,
    pub enc_cert: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}