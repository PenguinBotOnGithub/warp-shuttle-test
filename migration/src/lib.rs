pub use sea_orm_migration::prelude::*;

mod m20240225_163138_create_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240225_163138_create_user::Migration)]
    }
}
