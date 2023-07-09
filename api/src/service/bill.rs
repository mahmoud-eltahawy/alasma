use crate::{repo::bill::*, AppState, Bill};
use actix_web::{
    delete, get, post, put,
    web::{self, Data},
    HttpResponse, Responder, Scope,
};
use uuid::Uuid;

pub fn scope() -> Scope {
    web::scope("/bill")
        .service(get_by_id)
        .service(save)
        .service(update)
        .service(delete_by_id)
}

#[get("/{id}")]
async fn get_by_id(
    state: Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match fetch_bill_by_id(&state, id.into_inner()).await {
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
    match delete_bill_by_id(&state, id).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[post("/")]
async fn save(state: Data<AppState>, dep: web::Json<Bill>) -> impl Responder {
    let dep = dep.into_inner();
    match save_bill(&state, dep).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[put("/")]
async fn update(state: Data<AppState>, dep: web::Json<Bill>) -> impl Responder {
    let dep = dep.into_inner();
    match update_bill(&state, dep).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
