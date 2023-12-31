use sqlx::query;
use std::error::Error;
use uuid::Uuid;

use crate::{AppState, SellBill};

pub async fn fetch_sell_bills_by_sheet_id(
    state: &AppState,
    id: Uuid,
) -> Result<Vec<SellBill>, Box<dyn Error>> {
    let records = query!(
        r#"
        select *
        from sell_bill WHERE sheet_id = $1"#,
        id
    )
    .fetch_all(&state.db)
    .await?;
    Ok(records
        .into_iter()
        .map(|record| SellBill {
            bill_id: record.bill_id,
            tax_number: record.tax_number,
            company_id: record.company_id,
            client_id: record.client_id,
            sheet_id: record.sheet_id,
            total_cost: record.total_cost,
            discount: record.discount,
        })
        .collect::<Vec<_>>())
}

pub async fn delete_sell_bill_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<(), Box<dyn Error>> {
    query!(
        r#"
        DELETE
        FROM sell_bill WHERE bill_id = $1"#,
        id
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn save_sell_bill(
    state: &AppState,
    sell_bill: SellBill,
) -> Result<(), Box<dyn Error>> {
    let SellBill {
        bill_id,
        tax_number,
        company_id,
        client_id,
        sheet_id,
        total_cost,
        discount,
    } = sell_bill;
    query!(
        r#"
        INSERT INTO sell_bill(
          bill_id,
	  tax_number,
	  company_id,
	  client_id,
	  sheet_id,
	  total_cost,
	  discount
        ) VALUES ($1,$2,$3,$4,$5,$6,$7)"#,
        bill_id,
        tax_number,
        company_id,
        client_id,
        sheet_id,
        total_cost,
        discount,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn update_sell_bill(
    state: &AppState,
    sell_bill: SellBill,
) -> Result<(), Box<dyn Error>> {
    let SellBill {
        bill_id,
        tax_number,
        company_id,
        client_id,
        sheet_id,
        total_cost,
        discount,
    } = sell_bill;
    query!(
        r#"
        UPDATE sell_bill SET
	  tax_number =$2,
	  company_id = $3,
	  client_id = $4,
	  sheet_id = $5,
	  total_cost = $6,
	  discount = $7
        WHERE bill_id = $1"#,
        bill_id,
        tax_number,
        company_id,
        client_id,
        sheet_id,
        total_cost,
        discount,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}
