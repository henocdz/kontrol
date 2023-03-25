use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subscription::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subscription::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Subscription::PublicId).uuid().not_null())
                    .col(ColumnDef::new(Subscription::Name).string().not_null())
                    .col(ColumnDef::new(Subscription::Amount).decimal().not_null())
                    .col(ColumnDef::new(Subscription::Interval).integer().not_null())
                    .col(ColumnDef::new(Subscription::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subscription::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Subscription {
    Table,
    Id,
    PublicId,
    Name,
    Amount,
    Interval,
    CreatedAt,
}