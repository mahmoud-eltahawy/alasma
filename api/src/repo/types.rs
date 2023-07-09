use sqlx::query;
use std::error::Error;
use uuid::Uuid;

use crate::{AppState, TypeRow};

pub async fn fetch_cargo_bill_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<TypeRow, Box<dyn Error>> {
    let record = query!(
        r#"
        select *
        from cargo_bill WHERE id = $1"#,
        id
    )
    .fetch_one(&state.db)
    .await?;
    Ok(TypeRow {
        id: record.id,
        cargo_id: record.cargo_id,
        bill_id: record.bill_id,
        quantity: record.quantity,
        one_cost: record.one_cost,
    })
}

pub async fn delete_cargo_bill_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<(), Box<dyn Error>> {
    query!(
        r#"
        DELETE
        FROM cargo_bill WHERE id = $1"#,
        id
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn save_cargo_bill(
    state: &AppState,
    cargo_bill: TypeRow,
) -> Result<(), Box<dyn Error>> {
    let TypeRow {
        id,
        cargo_id,
        bill_id,
        quantity,
        one_cost,
    } = cargo_bill;
    query!(
        r#"
        INSERT INTO cargo_bill(
        id,
	cargo_id,
	bill_id,
	quantity,
	one_cost
        ) VALUES ($1,$2,$3,$4,$5)"#,
        id,
        cargo_id,
        bill_id,
        quantity,
        one_cost,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn update_cargo_bill(
    state: &AppState,
    cargo_bill: TypeRow,
) -> Result<(), Box<dyn Error>> {
    let TypeRow {
        id,
        cargo_id,
        bill_id,
        quantity,
        one_cost,
    } = cargo_bill;
    query!(
        r#"
        UPDATE cargo_bill SET
	    cargo_id = $2,
	    bill_id = $3,
	    quantity = $4,
	    one_cost = $5
        WHERE id = $1"#,
        id,
        cargo_id,
        bill_id,
        quantity,
        one_cost,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}
