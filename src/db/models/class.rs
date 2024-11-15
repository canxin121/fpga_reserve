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
    pub class_id: String,
}

impl ActiveModel {
    pub fn new(class_id: String) -> Self {
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
        Some(super::class_student_junction::Relation::Class.def())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod test {
    use crate::db::{
        conn::{get_db, set_db},
        models::student,
    };

    use super::*;
    use sea_orm::Condition;

    async fn init_db() -> Result<()> {
        let _ = tracing_subscriber::fmt::try_init();

        let database_url = "sqlite://test_data/test_class_get_students.db";
        set_db(database_url).await?;
        let db = get_db().await?;

        // 插入一些测试数据
        let students: Vec<student::ActiveModel> = (0..=10)
            .map(|i| {
                student::ActiveModel::new(
                    Some(i.to_string()),
                    Some(i.to_string()),
                    i.to_string(),
                    Some(i.to_string()),
                )
            })
            .collect();
        student::Entity::insert_many(students)
            .exec_without_returning(&db)
            .await
            .expect("insert students");
        let class1 = ActiveModel::new("class1".to_string());
        super::Entity::insert(class1)
            .exec_without_returning(&db)
            .await
            .expect("insert class1");
        let class2 = ActiveModel::new("class2".to_string());
        super::Entity::insert(class2)
            .exec_without_returning(&db)
            .await
            .expect("insert class2");
        let students1 = student::Entity::find()
            .filter(Condition::any().add(student::Column::Id.lte(7)))
            .all(&db)
            .await
            .expect("find student1");

        for student in students1 {
            student.join_class(1).await.expect("join class1");
        }

        let student2 = student::Entity::find()
            .filter(Condition::any().add(student::Column::Id.gt(5)))
            .all(&db)
            .await
            .expect("find student2");

        for student in student2 {
            student.join_class(2).await.expect("join class2");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_class_get_students() {
        init_db().await.expect("init db");
    }
}
