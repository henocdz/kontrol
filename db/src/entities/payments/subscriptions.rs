use sea_orm::entity::prelude::*;

enum PaymentInterval {
    Biweekly(i32),
    Monthly,
    Bimonthly,
    Quarterly,
    Biannually,
    Annually,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "payments_subscription")]
pub struct Subscription {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub public_id: uuid::Uuid,
    pub created_at: DateTime,

    pub name: String,
    pub amount: f64,

    pub interval: PaymentInterval
}
