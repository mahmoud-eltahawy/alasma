use crate::{repo::client::*, AppState, Client};
use actix_web::{
    delete, get, post, put,
    web::{self, Data},
    HttpResponse, Responder, Scope,
};
use uuid::Uuid;

pub fn scope() -> Scope {
    web::scope("/client")
        .service(get_by_id)
        .service(get_id_by_name)
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
    match fetch_client_by_id(&state, id.into_inner()).await {
        Ok(dep) => HttpResponse::Ok().json(dep),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/{name}/id")]
async fn get_id_by_name(
    state: Data<AppState>,
    name: web::Path<String>,
) -> impl Responder {
    match fetch_client_id_by_name(&state, name.into_inner()).await {
        Ok(dep) => HttpResponse::Ok().json(dep),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/{name}/find")]
async fn search_by_name(
    state: Data<AppState>,
    name: web::Path<String>,
) -> impl Responder {
    match search_client_by_name(&state, name.into_inner()).await {
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
    match delete_client_by_id(&state, id).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[post("/")]
async fn save(state: Data<AppState>, dep: web::Json<Client>) -> impl Responder {
    let dep = dep.into_inner();
    match save_client(&state, dep).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[put("/")]
async fn update(
    state: Data<AppState>,
    dep: web::Json<Client>,
) -> impl Responder {
    let dep = dep.into_inner();
    match update_client(&state, dep).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
