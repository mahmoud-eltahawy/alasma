use sqlx::query;
use uuid::Uuid;
use std::error::Error;

use crate::{AppState, Sheet};

pub async fn fetch_sheet_by_id(
    state: &AppState,
    id : Uuid,
) -> Result<Sheet, Box<dyn Error>> {
    let record = query!(
        r#"
        select *
        from sheet WHERE id = $1"#,
    id)
    .fetch_one(&state.db)
    .await?;
    Ok(Sheet {
	id: record.id,
	the_name: record.the_name,
	the_date: record.the_date,
	the_type: serde_json::from_str(&record.the_type).unwrap()
    })
}

pub async fn delete_sheet_by_id(
    state: &AppState,
    id : Uuid,
) -> Result<(), Box<dyn Error>> {
    query!(
        r#"
        DELETE
        FROM sheet WHERE id = $1"#,
    id)
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn save_sheet(
    state: &AppState,
    sheet : Sheet,
) -> Result<(), Box<dyn Error>> {
    let Sheet {
	id,
	the_name,
	the_date,
	the_type,
    } = sheet;
    query!(
        r#"
        INSERT INTO sheet(
	id,
	the_name,
	the_date,
	the_type
        ) VALUES ($1,$2,$3,$4)"#,
	id,
	the_name,
	the_date,
	serde_json::json!(the_type).to_string(),
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn update_sheet(
    state: &AppState,
    sheet : Sheet,
) -> Result<(), Box<dyn Error>> {
    let Sheet {
	id,
	the_name,
	the_date,
	the_type,
    } = sheet;
    query!(
        r#"
        UPDATE sheet SET
	    the_name = $2,
	    the_date = $3,
	    the_type = $4
        WHERE id = $1"#,
	id,
	the_name,
	the_date,
	serde_json::json!(the_type).to_string(),
    )
    .execute(&state.db)
    .await?;
    Ok(())
}
