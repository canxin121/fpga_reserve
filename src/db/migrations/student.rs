use crate::db::models::student::Column;
use async_trait::async_trait;
use sea_orm_migration::{
    prelude::*,
    schema::{big_integer, string, string_null},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StudentTable::Student)
                    .col(
                        big_integer(Column::Id)
                            .auto_increment()
                            .primary_key()
                            .not_null(),
                    )
                    .col(string_null(Column::StudentId).unique_key())
                    .col(string_null(Column::Account).unique_key())
                    .col(string(Column::PasswordHash).not_null())
                    .col(string_null(Column::Name))
                    .if_not_exists()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(StudentTable::Student)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum StudentTable {
    Student,
}
