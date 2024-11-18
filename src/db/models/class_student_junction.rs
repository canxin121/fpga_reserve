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

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::{
        db_conn::{close_db, get_db, reinit_db, set_db},
        models::{class, student},
    };
    use anyhow::Result;
    use sea_orm::Condition;

    async fn init_db() -> Result<()> {
        let _ = tracing_subscriber::fmt::try_init();
        // let database_url = "sqlite::memory:";
        let database_url = "mysql://test:testpasswd@localhost:3306/fpga_reserve";

        let _ = close_db().await;
        set_db(database_url).await?;
        reinit_db().await?;
        let db = get_db().await?;

        // 插入一些测试数据
        let students: Vec<student::ActiveModel> =
            futures::future::join_all((0..=10).map(|i| async move {
                student::ActiveModel::new_encrypted(
                    Some(i.to_string()),
                    Some(i.to_string()),
                    i.to_string(),
                    Some(i.to_string()),
                )
                .await
                .unwrap()
            }))
            .await;
        student::Entity::insert_many(students)
            .exec_without_returning(&db)
            .await
            .expect("insert students");
        let class1 = class::ActiveModel::new(None);
        class::Entity::insert(class1)
            .exec_without_returning(&db)
            .await
            .expect("insert class1");
        let class2 = class::ActiveModel::new(None);
        class::Entity::insert(class2)
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

    async fn test_class_get_students() {
        init_db().await.expect("init db");
        let db = get_db().await.expect("get db");
        let student1 = student::Entity::find()
            .filter(Condition::any().add(student::Column::Id.eq(6)))
            .one(&db)
            .await
            .unwrap()
            .unwrap();
        let class = student1.find_related(class::Entity).all(&db).await.unwrap();
        assert!(class.len() == 2);
    }

    async fn test_student_get_classes() {
        init_db().await.expect("init db");
        let db = get_db().await.expect("get db");
        let class1 = class::Entity::find()
            .filter(Condition::any().add(class::Column::Id.eq(1)))
            .one(&db)
            .await
            .unwrap()
            .unwrap();
        let students = class1.find_related(student::Entity).all(&db).await.unwrap();
        assert!(students.len() == 7);
    }

    async fn test_student_leave_class() {
        init_db().await.expect("init db");
        let db = get_db().await.expect("get db");
        let student1 = student::Entity::find()
            .filter(Condition::any().add(student::Column::Id.eq(6)))
            .one(&db)
            .await
            .unwrap()
            .unwrap();

        let student1_classes = student1.find_related(class::Entity).all(&db).await.unwrap();
        assert!(student1_classes.len() == 2);
        let class1_students = class::Entity::find()
            .filter(Condition::any().add(class::Column::Id.eq(1)))
            .one(&db)
            .await
            .unwrap()
            .unwrap()
            .find_related(student::Entity)
            .all(&db)
            .await
            .unwrap();
        assert!(class1_students.len() == 7);

        student1.leave_class(1).await.expect("leave class1");

        let class1 = class::Entity::find()
            .filter(Condition::any().add(class::Column::Id.eq(1)))
            .one(&db)
            .await
            .unwrap()
            .unwrap();
        let students = class1.find_related(student::Entity).all(&db).await.unwrap();
        assert!(students.len() == 6);
        let student1_classes = student1.find_related(class::Entity).all(&db).await.unwrap();
        assert!(student1_classes.len() == 1);
    }

    #[tokio::test]
    async fn test_all() {
        test_class_get_students().await;
        test_student_get_classes().await;
        test_student_leave_class().await;
    }
}
