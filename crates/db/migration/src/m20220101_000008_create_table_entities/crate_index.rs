//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "crate_index")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub vers: String,
    pub deps: Option<Json>,
    #[sea_orm(column_type = "Text")]
    pub cksum: String,
    pub features: Option<Json>,
    pub yanked: bool,
    #[sea_orm(column_type = "Text", nullable)]
    pub links: Option<String>,
    pub v: i32,
    pub crate_fk: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::krate::Entity",
        from = "Column::CrateFk",
        to = "super::krate::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Krate,
}

impl Related<super::krate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Krate.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}