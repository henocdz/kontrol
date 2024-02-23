use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServiceCategory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServiceCategory::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ServiceCategory::PublicId).uuid().not_null())
                    .col(ColumnDef::new(ServiceCategory::Name).string().not_null())
                    .col(ColumnDef::new(ServiceCategory::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(ServiceCategory::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await;

        manager
            .create_table(
                Table::create()
                    .table(Service::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Service::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Service::PublicId).uuid().not_null())
                    .col(ColumnDef::new(Service::Name).string().not_null())
                    .col(ColumnDef::new(Service::ServiceCategoryId).big_integer().not_null())
                    .col(ColumnDef::new(Service::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(Service::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // Foreign Keys
        manager.create_foreign_key(
            ForeignKey::create()
            .from(Service::Table, Service::ServiceCategoryId)
            .to(ServiceCategory::Table, ServiceCategory::Id)
            .on_delete(ForeignKeyAction::Restrict)
        ).await?;

        // Indexes
        manager.create_index(sea_query::Index::create().if_not_exists().name("idx_service_pubid").table(Service::Table).col(Service::PublicId).to_owned()).await?;
        manager.create_index(sea_query::Index::create().if_not_exists().name("idx_service_servcat_fk").table(Service::Table).col(Service::ServiceCategoryId)).await?;

        manager.create_index(sea_query::Index::create().if_not_exists().name("idx_servicecategory_pubid").table(ServiceCategory::Table).col(Service::PublicId).to_owned()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Dropping tables should drop indexes
        manager
            .drop_table(Table::drop().table(Service::Table).cascade().to_owned())
            .await;

        manager
            .drop_table(Table::drop().table(ServiceCategory::Table).cascade().to_owned())
            .await
    }
}

#[derive(Iden)]
enum Service {
    Table,
    Id,
    PublicId,
    Name,
    ServiceCategoryId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum ServiceCategory {
    Table,
    Id,
    PublicId,
    Name,
    CreatedAt,
    UpdatedAt,
}