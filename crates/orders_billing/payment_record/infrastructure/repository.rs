use sea_orm::{
    DatabaseConnection, EntityTrait, ActiveModelTrait, ColumnTrait, QueryFilter, Set, NotSet,
};

use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use num_traits::cast::ToPrimitive;

use crate::payment_record::domain::{
    model::{Payment, PaymentMethod, PaymentStatus, Invoice, InvoiceItem},
    repository::OrdersBillingRepository,
};

use shared::entity::{
    payment, order, order_details, product, user, sea_orm_active_enums::*,
};

use async_trait::async_trait;

pub struct SupabaseOrdersBillingRepository {
    pub db: DatabaseConnection,
}

impl SupabaseOrdersBillingRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl OrdersBillingRepository for SupabaseOrdersBillingRepository {
    async fn get_by_order_id(&self, order_id: i32) -> Result<Option<Payment>, String> {
        let result = payment::Entity::find()
            .filter(payment::Column::IdOrder.eq(order_id))
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result.map(|p| Payment {
            id: Some(p.id_payment),
            order_id: p.id_order.expect("El ID del pedido no puede ser None"),
            total_amount: p
                .total_amount
                .expect("Total no puede ser None")
                .to_f64()
                .unwrap_or(0.0),
            payment_date: p.payment_date.expect("Fecha de pago no puede ser None"),
            payment_status: p
                .payment_status
                .and_then(|s| s.try_into().ok())
                .unwrap_or(PaymentStatus::Pending),
            payment_method: p
                .payment_method
                .and_then(|m| m.try_into().ok())
                .unwrap_or(PaymentMethod::Visa),
        }))
    }

    async fn register_payment(&self, payment: Payment) -> Result<(), String> {
        let order_model = order::Entity::find_by_id(payment.order_id)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("El pedido no existe.")?;

        if order_model.order_status != Some(StateOrderEnum::Delivered) {
            return Err("El pedido no está en estado 'Delivered'.".to_string());
        }

        let new_payment = payment::ActiveModel {
            id_payment: NotSet,
            id_order: Set(Some(payment.order_id)),
            total_amount: Set(Some(
                Decimal::from_f64(payment.total_amount).ok_or("Monto inválido")?,
            )),
            payment_date: Set(Some(payment.payment_date)),
            payment_status: Set(Some(payment.payment_status.into())),
            payment_method: Set(Some(payment.payment_method.into())),
        };

        new_payment
            .insert(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn generate_invoice(&self, order_id: i32) -> Result<Invoice, String> {
        let order = order::Entity::find_by_id(order_id)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Pedido no encontrado")?;

        let user_id = order
            .id_user
            .ok_or("El pedido no tiene asignado un usuario")?;

        let customer = user::Entity::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Cliente no encontrado")?;

        let payment = payment::Entity::find()
            .filter(payment::Column::IdOrder.eq(order_id))
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Pago no registrado")?;

        let payment_date = payment
            .payment_date
            .ok_or("La fecha de pago no está registrada")?;

        let details = order_details::Entity::find()
            .filter(order_details::Column::IdOrder.eq(order_id))
            .find_also_related(product::Entity)
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        let mut items = vec![];

        for (detail, maybe_product) in details {
            if let Some(prod) = maybe_product {
                let quantity = detail.amount.expect("Cantidad faltante");
                let unit_price = prod.price.as_ref().and_then(|p| p.to_f64()).unwrap_or(0.0);

                items.push(InvoiceItem {
                    product_name: prod.name.expect("Nombre de producto faltante"),
                    quantity,
                    unit_price,
                    subtotal: unit_price * quantity as f64,
                });
            }
        }

        Ok(Invoice {
            order_id,
            customer_name: customer.name.unwrap_or_default(),
            delivery_address: order.delivery_address.unwrap_or_default(),
            payment_method: payment
                .payment_method
                .and_then(|m| m.try_into().ok())
                .unwrap_or(PaymentMethod::Visa),
            payment_date,
            total_amount: payment
                .total_amount
                .expect("Total faltante")
                .to_f64()
                .unwrap_or(0.0),
            items,
        })
    }
}
