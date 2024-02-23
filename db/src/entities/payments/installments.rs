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

    pub service_id: i32,
    pub payment_method: i32,

    pub amount: f64,
    pub interval: PaymentInterval,
    pub payment_date: Date,

    pub created_at: DateTime,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "payments_installmentpayment")]
pub struct InstallmentPayment {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub public_id: uuid::Uuid,

    pub product_id: i32,

    pub payment_date: Date,

    pub total_amount: f64,
    pub installment_amount: f64,
    pub installments : i32,
    pub interval: PaymentInterval,

    pub created_at: DateTime,
}
