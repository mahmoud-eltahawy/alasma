use crate::{repo::company::*, AppState, Company};
use actix_web::{
    delete, get, post, put,
    web::{self, Data},
    HttpResponse, Responder, Scope,
};
use uuid::Uuid;

pub fn scope() -> Scope {
    web::scope("/company")
        .service(get_by_id)
        .service(search_by_name)
        .service(save)
        .service(update)
        .service(delete_by_id)
}

#[get("/{id}")]
async fn get_by_id(
    state: Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match fetch_company_by_id(&state, id.into_inner()).await {
        Ok(dep) => HttpResponse::Ok().json(dep),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/{name}/find")]
async fn search_by_name(
    state: Data<AppState>,
    name: web::Path<String>,
) -> impl Responder {
    match search_company_by_name(&state, name.into_inner()).await {
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
    match delete_company_by_id(&state, id).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[post("/")]
async fn save(
    state: Data<AppState>,
    dep: web::Json<Company>,
) -> impl Responder {
    let dep = dep.into_inner();
    match save_company(&state, dep).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[put("/")]
async fn update(
    state: Data<AppState>,
    dep: web::Json<Company>,
) -> impl Responder {
    let dep = dep.into_inner();
    match update_company(&state, dep).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
