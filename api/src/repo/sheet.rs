use chrono::NaiveDate;
use models::backend_api::{SheetType, Name};
use sqlx::query;
use std::error::Error;
use uuid::Uuid;

use crate::{AppState, Sheet};

pub async fn fetch_sheet_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<Sheet, Box<dyn Error>> {
    let record = query!(
        r#"
        select *
        from sheet WHERE id = $1"#,
        id
    )
    .fetch_one(&state.db)
    .await?;
    Ok(Sheet {
        id: record.id,
        the_name: record.the_name,
        the_date: record.the_date,
        the_type: serde_json::from_str(&record.the_type).unwrap(),
    })
}

pub async fn search_sheets(
    state: &AppState,
    offset: i64,
    name: Option<String>,
    begin_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    sheet_type: SheetType,
) -> Result<Vec<Sheet>, Box<dyn Error>> {
    let sheet_type = serde_json::json!(sheet_type).to_string();
    let sheets = match (name,begin_date,end_date) {
	(Some(name),None,None) =>{
	    let name = format!("%{name}%");
	    query!(
		r#"
		select * from sheet
                WHERE the_name LIKE $1 AND the_type = $2 OFFSET $3 LIMIT 5"#,
	        name,
		sheet_type,
		offset,
	    )
	    .fetch_all(&state.db)
	    .await?.into_iter().map(|record| Sheet {
		id: record.id,
		the_name: record.the_name,
		the_date: record.the_date,
		the_type: serde_json::from_str(&record.the_type).unwrap(),
	    }).collect::<Vec<_>>()
	},
	(Some(name),Some(begin),None) =>{
	    let name = format!("%{name}%");
	    query!(
		r#"
		select * from sheet
                WHERE the_name LIKE $1 AND
                  (the_date > $2 AND the_type = $3) offset $4 LIMIT 5"#,
	        name,
		begin,
		sheet_type,
		offset,
	    )
	    .fetch_all(&state.db)
	    .await?.into_iter().map(|record| Sheet {
		id: record.id,
		the_name: record.the_name,
		the_date: record.the_date,
		the_type: serde_json::from_str(&record.the_type).unwrap(),
	    }).collect::<Vec<_>>()
	},
	(Some(name),None,Some(end)) =>{
	    let name = format!("%{name}%");
	    query!(
		r#"
		select * from sheet
                WHERE the_name LIKE $1 AND
                  (the_date < $2 AND the_type = $3) offset $4 LIMIT 5"#,
	        name,
		end,
		sheet_type,
		offset,
	    )
	    .fetch_all(&state.db)
	    .await?.into_iter().map(|record| Sheet {
		id: record.id,
		the_name: record.the_name,
		the_date: record.the_date,
		the_type: serde_json::from_str(&record.the_type).unwrap(),
	    }).collect::<Vec<_>>()
	},
	(Some(name),Some(begin),Some(end)) =>{
	    let name = format!("%{name}%");
	    query!(
		r#"
		select * from sheet
                WHERE the_name LIKE $1 AND
                  (the_date BETWEEN $2 AND $3 AND the_type = $4) offset $5 LIMIT 5"#,
	        name,
		begin,
		end,
		sheet_type,
		offset,
	    )
	    .fetch_all(&state.db)
	    .await?.into_iter().map(|record| Sheet {
		id: record.id,
		the_name: record.the_name,
		the_date: record.the_date,
		the_type: serde_json::from_str(&record.the_type).unwrap(),
	    }).collect::<Vec<_>>()
	},
	(None,Some(begin),Some(end)) =>{
	    query!(
		r#"
		select * from sheet
                WHERE the_type = $1 AND the_date BETWEEN $2 AND $3 offset $4 LIMIT 5"#,
		sheet_type,
		begin,
		end,
		offset,
	    )
	    .fetch_all(&state.db)
	    .await?.into_iter().map(|record| Sheet {
		id: record.id,
		the_name: record.the_name,
		the_date: record.the_date,
		the_type: serde_json::from_str(&record.the_type).unwrap(),
	    }).collect::<Vec<_>>()
	},
	(None,Some(begin),None) =>{
	    query!(
		r#"
		select * from sheet
                WHERE the_type = $1 AND the_date >= $2 offset $3 LIMIT 5"#,
		sheet_type,
		begin,
		offset,
	    )
	    .fetch_all(&state.db)
	    .await?.into_iter().map(|record| Sheet {
		id: record.id,
		the_name: record.the_name,
		the_date: record.the_date,
		the_type: serde_json::from_str(&record.the_type).unwrap(),
	    }).collect::<Vec<_>>()
	},
	(None,None,Some(end)) =>{
	    query!(
		r#"
		select * from sheet
                WHERE the_type = $1 AND the_date <= $2 offset $3 LIMIT 5"#,
		sheet_type,
		end,
		offset,
	    )
	    .fetch_all(&state.db)
	    .await?.into_iter().map(|record| Sheet {
		id: record.id,
		the_name: record.the_name,
		the_date: record.the_date,
		the_type: serde_json::from_str(&record.the_type).unwrap(),
	    }).collect::<Vec<_>>()
	},
	(None,None,None) => query!(
		r#"
		select *
		from sheet WHERE the_type = $1 offset $2 LIMIT 5"#,
	        sheet_type,
		offset,
	    )
	    .fetch_all(&state.db)
	    .await?.into_iter().map(|record| Sheet {
		id: record.id,
		the_name: record.the_name,
		the_date: record.the_date,
		the_type: serde_json::from_str(&record.the_type).unwrap(),
	    }).collect::<Vec<_>>()
    };
    Ok(sheets)
}

pub async fn delete_sheet_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<(), Box<dyn Error>> {
    query!(
        r#"
        DELETE
        FROM sheet WHERE id = $1"#,
        id
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn save_sheet(
    state: &AppState,
    sheet: Sheet,
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
    sheet: Sheet,
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

pub async fn update_sheet_name(
    state: &AppState,
    name: Name,
) -> Result<(), Box<dyn Error>> {
    let Name { id, the_name } = name;
    query!(
        r#"
        UPDATE sheet SET
	    the_name = $2
        WHERE id = $1"#,
        id,
        the_name,
    )
    .execute(&state.db)
    .await?;
    Ok(())
}
