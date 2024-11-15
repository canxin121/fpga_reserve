// class和student的多对多关系 连接表

use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "class_student_junction"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub class_pid: i64,
    pub student_pid: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    ClassPid,
    StudentPid,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    ClassPid,
    StudentPid,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i64, i64);

    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Class,
    Student,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Class => Entity::belongs_to(super::class::Entity)
                .from(Column::ClassPid)
                .to(super::class::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
            Relation::Student => Entity::belongs_to(super::student::Entity)
                .from(Column::StudentPid)
                .to(super::student::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
        }
    }
}

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl Related<super::class::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Class.def()
    }
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Column::ClassPid => {
                sea_orm::prelude::ColumnTypeTrait::def(sea_orm::prelude::ColumnType::Integer)
            }
            Column::StudentPid => {
                sea_orm::prelude::ColumnTypeTrait::def(sea_orm::prelude::ColumnType::Integer)
            }
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(class_pid: i64, student_pid: i64) -> Self {
        Self {
            class_pid: Set(class_pid),
            student_pid: Set(student_pid),
        }
    }
}
