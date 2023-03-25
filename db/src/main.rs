use sea_orm_migration::prelude::*;

mod migrations;
use dotenv::dotenv;


#[async_std::main]
async fn main() {
    dotenv().ok();
    cli::run_cli(db::Migrator).await;
}
