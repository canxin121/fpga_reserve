pub mod class;
pub mod class_student_junction;
pub mod student;
pub mod teacher;

use async_trait::async_trait;
use sea_orm_migration::*;

pub(crate) struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(class::Migration),
            Box::new(class_student_junction::Migration),
            Box::new(student::Migration),
        ]
    }
}
