use crate::{repo::sheet::*, AppState, Sheet};
use actix_web::{
    delete, get, post, put,
    web::{self, Data},
    HttpResponse, Responder, Scope,
};
use models::backend_api::{SheetShearchParams, Name};
use uuid::Uuid;

pub fn scope() -> Scope {
    web::scope("/sheet")
        .service(get_by_id)
        .service(search)
        .service(save)
        .service(update)
        .service(update_name)
        .service(delete_by_id)
}

#[get("/{id}")]
async fn get_by_id(
    state: Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match fetch_sheet_by_id(&state, id.into_inner()).await {
        Ok(dep) => HttpResponse::Ok().json(dep),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/search")]
async fn search(
    state: Data<AppState>,
    json: web::Json<SheetShearchParams>,
) -> impl Responder {
    let SheetShearchParams {
        offset,
        begin,
        end,
        name,
        sheet_type,
    } = json.into_inner();
    match search_sheets(&state, offset, name, begin, end, sheet_type).await {
        Ok(dep) => HttpResponse::Ok().json(dep),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[delete("/{id}")]
async fn delete_by_id(
    state: Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let id = id.into_inner();
    match delete_sheet_by_id(&state, id).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[post("/")]
async fn save(state: Data<AppState>, dep: web::Json<Sheet>) -> impl Responder {
    let dep = dep.into_inner();
    match save_sheet(&state, dep).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[put("/")]
async fn update(
    state: Data<AppState>,
    dep: web::Json<Sheet>,
) -> impl Responder {
    let dep = dep.into_inner();
    match update_sheet(&state, dep).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[put("/name")]
async fn update_name(
    state: Data<AppState>,
    dep: web::Json<Name>,
) -> impl Responder {
    let dep = dep.into_inner();
    match update_sheet_name(&state, dep).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
