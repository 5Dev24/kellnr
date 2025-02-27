//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "crate_author")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Text", unique)]
    pub author: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::crate_author_to_crate::Entity")]
    CrateAuthorToCrate,
}

impl Related<super::crate_author_to_crate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CrateAuthorToCrate.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
