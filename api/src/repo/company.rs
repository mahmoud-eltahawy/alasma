use sqlx::{query, query_as};
use std::error::Error;
use uuid::Uuid;

use crate::{AppState, Company, Name};

pub async fn fetch_company_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<Company, Box<dyn Error>> {
    let record = query!(
        r#"
        select *
        from company WHERE id = $1 LIMIT 4"#,
        id
    )
    .fetch_one(&state.db)
    .await?;
    Ok(Company {
        id: record.id,
        the_name: record.the_name,
    })
}

pub async fn fetch_company_id_by_name(
    state: &AppState,
    the_name : String,
) -> Result<Uuid, Box<dyn Error>> {
    let record = query!(
        r#"
        select id
        from company WHERE the_name = $1"#,
	the_name,
    )
    .fetch_one(&state.db)
    .await?;
    Ok(record.id)
}

pub async fn search_company_by_name(
    state: &AppState,
    name: String,
) -> Result<Vec<Name>, Box<dyn Error>> {
    let name = format!("%{name}%");
    let coms = query_as!(
        Name,
        r#"
        select *
        from company WHERE the_name LIKE $1 LIMIT 5"#,
        name
    )
    .fetch_all(&state.db)
    .await?;
    Ok(coms)
}

pub async fn delete_company_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<(), Box<dyn Error>> {
    query!(
        r#"
        DELETE
        FROM company WHERE id = $1"#,
        id
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn save_company(
    state: &AppState,
    company: Company,
) -> Result<(), Box<dyn Error>> {
    let Company { id, the_name } = company;
    query!(
        r#"
        INSERT INTO company(
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

pub async fn update_company(
    state: &AppState,
    company: Company,
) -> Result<(), Box<dyn Error>> {
    let Company { id, the_name } = company;
    query!(
        r#"
        UPDATE company SET
	    the_name = $2
        WHERE id = $1"#,
        id,
        the_name,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}
