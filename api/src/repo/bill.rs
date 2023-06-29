use sqlx::query;
use uuid::Uuid;
use std::error::Error;

use crate::{AppState, Bill};

pub async fn fetch_bill_by_id(
    state: &AppState,
    id : Uuid,
) -> Result<Bill, Box<dyn Error>> {
    let record = query!(
        r#"
        select *
        from bill WHERE id = $1"#,
    id)
    .fetch_one(&state.db)
    .await?;
    Ok(Bill{
	id : record.id,
	bill_number : record.bill_number,
	the_date : record.the_date,
	is_sell : record.is_sell,
    })
}

pub async fn delete_bill_by_id(
    state: &AppState,
    id : Uuid,
) -> Result<(), Box<dyn Error>> {
    query!(
        r#"
        DELETE
        FROM bill WHERE id = $1"#,
    id)
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn save_bill(
    state: &AppState,
    bill : Bill,
) -> Result<(), Box<dyn Error>> {
    let Bill {
	id: _,
	bill_number,
	the_date,
	is_sell,
    } = bill;
    query!(
        r#"
        INSERT INTO bill(
	    bill_number,
	    the_date,
	    is_sell
        ) VALUES ($1,$2,$3)"#,
	bill_number,
	the_date,
	is_sell,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn update_bill(
    state: &AppState,
    bill : Bill,
) -> Result<(), Box<dyn Error>> {
    let Bill {
	id,
	bill_number,
	the_date,
	is_sell,
    } = bill;
    query!(
        r#"
        UPDATE bill SET
	    bill_number = $2,
	    the_date = $3,
	    is_sell = $4
        WHERE id = $1"#,
	id,
	bill_number,
	the_date,
	is_sell,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}
