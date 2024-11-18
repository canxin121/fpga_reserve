use crate::db::{db_conn::get_db_str_result, hash_password, ARGON2};
use anyhow::Result;
use argon2::password_hash::{PasswordHash, PasswordVerifier};
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Condition, Set};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "teacher")]
pub struct Model {
    // 自增主键
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    // 工号
    pub teacher_id: Option<String>,
    // 账号
    pub account: Option<String>,
    // 密码
    pub password_hash: String,
    // 姓名
    pub name: Option<String>,
}

impl Model {
    pub async fn into_active_model_encrypted(self) -> Result<ActiveModel, String> {
        let password_hash = hash_password(self.password_hash).await?;
        Ok(ActiveModel {
            id: NotSet,
            teacher_id: Set(self.teacher_id),
            account: Set(self.account),
            password_hash: Set(password_hash),
            name: Set(self.name),
        })
    }

    pub async fn join_class_with_db<C>(
        &self,
        class_pid: i64,
        admin: bool,
        db: &C,
    ) -> Result<(), String>
    where
        C: ConnectionTrait,
    {
        let junction =
            super::class_teacher_junction::ActiveModel::new(class_pid, self.id.clone(), admin);
        junction
            .insert(db)
            .await
            .map_err(|e| format!("Failed to join class with db: {:?}", e))?;
        Ok(())
    }

    pub async fn join_class(&self, class_pid: i64, admin: bool) -> Result<(), String> {
        let junction =
            super::class_teacher_junction::ActiveModel::new(class_pid, self.id.clone(), admin);
        let db = get_db_str_result().await?;
        junction
            .insert(&db)
            .await
            .map_err(|e| format!("Failed to join class: {:?}", e))?;
        Ok(())
    }

    pub async fn leave_class(&self, class_pid: i64) -> Result<(), String> {
        let db = get_db_str_result().await?;
        let junction =
            super::class_teacher_junction::Entity::find_by_id((class_pid, self.id.clone()))
                .one(&db)
                .await
                .map_err(|e| format!("Failed to find junction: {:?}", e))?;

        if let Some(junction) = junction {
            junction
                .delete(&db)
                .await
                .map_err(|e| format!("Failed to delete junction: {:?}", e))?;
            Ok(())
        } else {
            return Err("Teacher is not in the class".to_string());
        }
    }

    pub async fn find_by_teacher_id_or_account(
        teacher_id_or_account: String,
    ) -> Result<Self, String> {
        let db = get_db_str_result().await?;

        Entity::find()
            .filter(
                Condition::any()
                    .add(Column::TeacherId.eq(teacher_id_or_account.clone()))
                    .add(Column::Account.eq(teacher_id_or_account)),
            )
            .one(&db)
            .await
            .map_err(|e| format!("{:?}", e))?
            .ok_or("Teacher not found".to_string())
    }

    pub fn verify_password(&self, password: String) -> Result<(), String> {
        let password_hash = PasswordHash::new(&self.password_hash)
            .map_err(|e| format!("Failed to create PasswordHash: {:?}", e))?;

        ARGON2
            .verify_password(password.as_bytes(), &password_hash)
            .map_err(|e| format!("Invalid password: {:?}", e.to_string()))
    }
}

impl ActiveModel {
    pub async fn new_encrypted(
        teacher_id: Option<String>,
        account: Option<String>,
        password: String,
        name: Option<String>,
    ) -> Result<Self, String> {
        let password_hash = hash_password(password).await?;
        Ok(Self {
            teacher_id: Set(teacher_id),
            account: Set(account),
            password_hash: Set(password_hash),
            name: Set(name),
            id: NotSet,
        })
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

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::class::Entity> for Entity {
    // 两条路径相加之后，得到的结果是从当前学生Entity到Class的Entity

    // 2. 从ClassTeacherJunction的Entity到Class的Entity
    fn to() -> RelationDef {
        super::class_teacher_junction::Relation::Class.def()
    }
    // 1. 从当前学生Entity到ClassTeacherJunction的Entity
    fn via() -> Option<RelationDef> {
        Some(super::class_teacher_junction::Relation::Teacher.def().rev())
    }
}

impl Related<super::experiment::Entity> for Entity {
    fn to() -> RelationDef {
        super::experiment_teacher_junction::Relation::Experiment.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::experiment_teacher_junction::Relation::Teacher
                .def()
                .rev(),
        )
    }
}
