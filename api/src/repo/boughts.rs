use sqlx::query;
use std::error::Error;
use uuid::Uuid;

use crate::{AppState, BuyBill};

pub async fn fetch_buy_bill_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<BuyBill, Box<dyn Error>> {
    let record = query!(
        r#"
        select *
        from buy_bill WHERE id = $1"#,
        id
    )
    .fetch_one(&state.db)
    .await?;
    Ok(BuyBill {
        id: record.id,
        cargo_name: record.cargo_name,
        bill_id: record.bill_id,
        quantity: record.quantity,
        one_cost: record.one_cost,
    })
}

pub async fn delete_buy_bill_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<(), Box<dyn Error>> {
    query!(
        r#"
        DELETE
        FROM buy_bill WHERE id = $1"#,
        id
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn save_buy_bill(
    state: &AppState,
    buy_bill: BuyBill,
) -> Result<(), Box<dyn Error>> {
    let BuyBill {
        id,
        cargo_name,
        bill_id,
        quantity,
        one_cost,
    } = buy_bill;
    query!(
        r#"
        INSERT INTO buy_bill(
            id,
	    cargo_name,
	    bill_id,
	    quantity,
	    one_cost
        ) VALUES ($1,$2,$3,$4,$5)"#,
        id,
        cargo_name,
        bill_id,
        quantity,
        one_cost,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn update_buy_bill(
    state: &AppState,
    buy_bill: BuyBill,
) -> Result<(), Box<dyn Error>> {
    let BuyBill {
        id,
        cargo_name,
        bill_id,
        quantity,
        one_cost,
    } = buy_bill;
    query!(
        r#"
        UPDATE buy_bill SET
	    cargo_name = $2,
	    bill_id = $3,
	    quantity = $4,
	    one_cost = $5
        WHERE id = $1"#,
        id,
        cargo_name,
        bill_id,
        quantity,
        one_cost,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}
