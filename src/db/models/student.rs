// 用于存储学生信息的SeaOrm的Model
use anyhow::Result;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};
use serde::{Deserialize, Serialize};

use crate::db::conn::get_db;

#[derive(Default, Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "student")]
pub struct Model {
    // 自增主键
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    // 学号
    pub student_id: Option<String>,
    // 账号
    pub account: Option<String>,
    // 密码
    pub password: String,
    // 姓名
    pub name: Option<String>,
}

impl Model {
    pub async fn join_class(&self, class_pid: i64) -> Result<()> {
        let junction = super::class_student_junction::ActiveModel::new(class_pid, self.id.clone());
        let db = get_db().await?;
        junction.insert(&db).await?;
        Ok(())
    }
}

impl ActiveModel {
    pub fn new(
        student_id: Option<String>,
        account: Option<String>,
        password: String,
        name: Option<String>,
    ) -> Self {
        Self {
            student_id: Set(student_id),
            account: Set(account),
            password: Set(password),
            name: Set(name),
            id: NotSet,
        }
    }

    pub fn encrypt_password(&mut self) -> Result<()> {
        todo!()
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

impl Related<super::class::Entity> for Entity {
    // 两条路径相加之后，得到的结果是从当前学生Entity到Class的Entity

    // 2. 从ClassStudentJunction的Entity到Class的Entity
    fn to() -> RelationDef {
        super::class_student_junction::Relation::Class.def()
    }
    // 1. 从当前学生Entity到ClassStudentJunction的Entity
    fn via() -> Option<RelationDef> {
        Some(super::class_student_junction::Relation::Student.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
