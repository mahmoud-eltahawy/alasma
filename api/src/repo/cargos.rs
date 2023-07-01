use sqlx::query;
use uuid::Uuid;
use std::error::Error;

use crate::{AppState, Cargo};

pub async fn fetch_cargo_by_id(
    state: &AppState,
    id : Uuid,
) -> Result<Cargo, Box<dyn Error>> {
    let record = query!(
        r#"
        select *
        from cargo WHERE id = $1"#,
    id)
    .fetch_one(&state.db)
    .await?;
    Ok(Cargo {
	id : record.id,
	cargo_name : record.cargo_name,
	cargo_number : record.cargo_number,
    })
}

pub async fn delete_cargo_by_id(
    state: &AppState,
    id : Uuid,
) -> Result<(), Box<dyn Error>> {
    query!(
        r#"
        DELETE
        FROM cargo WHERE id = $1"#,
    id)
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn save_cargo(
    state: &AppState,
    cargo_bill : Cargo,
) -> Result<(), Box<dyn Error>> {
    let Cargo {
	id : _,
	cargo_name,
	cargo_number,
    } = cargo_bill;
    query!(
        r#"
        INSERT INTO cargo(
	cargo_name,
	cargo_number
        ) VALUES ($1,$2)"#,
	cargo_name,
	cargo_number,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn update_cargo(
    state: &AppState,
    cargo_bill : Cargo,
) -> Result<(), Box<dyn Error>> {
    let Cargo {
	id,
	cargo_name,
	cargo_number,
    } = cargo_bill;
    query!(
        r#"
        UPDATE cargo SET
	    cargo_name = $2,
	    cargo_number = $3
        WHERE id = $1"#,
	id,
	cargo_name,
	cargo_number,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}
