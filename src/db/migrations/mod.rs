pub mod class;
pub mod class_student_junction;
pub mod class_teacher_junction;
pub mod experiment;
pub mod experiment_student_junction;
pub mod experiment_teacher_junction;
pub mod experiment_time_ranges;
pub mod experiment_time_ranges_student_junction;
pub mod student;
pub mod student_refresh_token;
pub mod teacher;
pub mod teacher_refresh_token;

use async_trait::async_trait;
use sea_orm_migration::*;

pub(crate) struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(class::Migration),
            Box::new(student::Migration),
            Box::new(teacher::Migration),
            Box::new(experiment::Migration),
            Box::new(experiment_time_ranges::Migration),
            Box::new(student_refresh_token::Migration),
            Box::new(teacher_refresh_token::Migration),
            // 连接表
            Box::new(experiment_teacher_junction::Migration),
            Box::new(experiment_student_junction::Migration),
            Box::new(experiment_time_ranges_student_junction::Migration),
            Box::new(class_student_junction::Migration),
            Box::new(class_teacher_junction::Migration),
        ]
    }
}
