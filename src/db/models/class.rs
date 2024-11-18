// 用于存储班级信息的Model

use anyhow::Result;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "class")]
pub struct Model {
    // 自增主键
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    // 班级名称
    pub class_id: Option<String>,
}

impl Model {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: NotSet,
            class_id: Set(self.class_id),
        }
    }
}

impl ActiveModel {
    pub fn new(class_id: Option<String>) -> Self {
        Self {
            class_id: Set(class_id),
            id: NotSet,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            _ => panic!("unreachable"),
        }
    }
}

impl Related<super::student::Entity> for Entity {
    // 两条路径相加之后，得到的结果是从当前班级Entity到Student的Entity
    // 2. 从ClassStudentJunction的Entity到Student的Entity
    fn to() -> RelationDef {
        super::class_student_junction::Relation::Student.def()
    }

    // 1. 从当前班级Entity到ClassStudentJunction的Entity
    fn via() -> Option<RelationDef> {
        Some(super::class_student_junction::Relation::Class.def().rev())
    }
}

impl Related<super::teacher::Entity> for Entity {
    // 两条路径相加之后，得到的结果是从当前班级Entity到Student的Entity
    // 2. 从ClassStudentJunction的Entity到Student的Entity
    fn to() -> RelationDef {
        super::class_teacher_junction::Relation::Teacher.def()
    }

    // 1. 从当前班级Entity到ClassStudentJunction的Entity
    fn via() -> Option<RelationDef> {
        Some(super::class_teacher_junction::Relation::Class.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
