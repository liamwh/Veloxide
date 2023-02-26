pub use sea_orm_migration::prelude::*;

mod m20230225_191443_create_project_table;
mod m20230225_201427_create_todo_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230225_191443_create_project_table::Migration),
            Box::new(m20230225_201427_create_todo_table::Migration),
        ]
    }
}
