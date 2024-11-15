// 用于存储教室信息的SeaOrm的Model

use anyhow::Result;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "student")]
pub struct Model {
    // 自增主键
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    // 工号
    pub teacher_id: Option<String>,
    // 账号
    pub account: Option<String>,
    // 密码
    pub password: String,
    // 姓名
    pub name: Option<String>,
}

impl ActiveModel {
    pub fn new(
        teacher_id: Option<String>,
        account: Option<String>,
        password: String,
        name: Option<String>,
    ) -> Self {
        Self {
            teacher_id: Set(teacher_id),
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
pub enum Relation {
    // 一个教师可能有多个课程
    Class,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            _ => todo!(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
