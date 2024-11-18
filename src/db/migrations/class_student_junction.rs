use async_trait::async_trait;
use sea_orm_migration::{prelude::*, schema::big_integer};

use crate::db::models::{class, class_student_junction::Column, student};

use super::{class::ClassTable, student::StudentTable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ClassStudentJunctionTable::ClassStudentJunction)
                    .col(big_integer(Column::ClassPid).not_null())
                    .col(big_integer(Column::StudentPid).not_null())
                    .primary_key(
                        Index::create()
                            .table(ClassStudentJunctionTable::ClassStudentJunction)
                            .col(Column::ClassPid)
                            .col(Column::StudentPid),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ClassStudentJunctionTable::ClassStudentJunction,
                                Column::ClassPid,
                            )
                            .to(ClassTable::Class, class::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ClassStudentJunctionTable::ClassStudentJunction,
                                Column::StudentPid,
                            )
                            .to(StudentTable::Student, student::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .if_not_exists()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ClassStudentJunctionTable::ClassStudentJunction)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ClassStudentJunctionTable {
    #[sea_orm(iden = "class_student_junction")]
    ClassStudentJunction,
}
