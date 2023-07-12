use sqlx::{query, query_as};
use std::error::Error;
use uuid::Uuid;

use crate::{AppState, Client, Name};

pub async fn fetch_client_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<Client, Box<dyn Error>> {
    let record = query!(
        r#"
        select *
        from client WHERE id = $1"#,
        id
    )
    .fetch_one(&state.db)
    .await?;
    Ok(Client {
        id: record.id,
        the_name: record.the_name,
    })
}

pub async fn search_client_by_name(
    state: &AppState,
    name: String,
) -> Result<Vec<Name>, Box<dyn Error>> {
    let name = format!("%{name}%");
    let coms = query_as!(
        Name,
        r#"
        select *
        from client WHERE the_name LIKE $1 LIMIT 5"#,
        name
    )
    .fetch_all(&state.db)
    .await?;
    Ok(coms)
}

pub async fn delete_client_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<(), Box<dyn Error>> {
    query!(
        r#"
        DELETE
        FROM client WHERE id = $1"#,
        id
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn save_client(
    state: &AppState,
    client: Client,
) -> Result<(), Box<dyn Error>> {
    let Client { id, the_name } = client;
    query!(
        r#"
        INSERT INTO client(
	id,
	the_name
        ) VALUES ($1,$2)"#,
        id,
        the_name,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn update_client(
    state: &AppState,
    client: Client,
) -> Result<(), Box<dyn Error>> {
    let Client { id, the_name } = client;
    query!(
        r#"
        UPDATE client SET
	    the_name = $2
        WHERE id = $1"#,
        id,
        the_name,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}
