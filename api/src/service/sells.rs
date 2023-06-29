use actix_web::{
  web::{Data, self},
  Responder,
  HttpResponse,
  Scope,
  post,get,delete,put
};
use crate::{AppState, repo::sells::*, SellBill};

pub fn scope() -> Scope{
  web::scope("/sells")
    .service(get_by_number)
    .service(save)
    .service(update)
    .service(delete_by_number)
}

#[get("/{number}")]
async fn get_by_number(state : Data<AppState>,number :web::Path<i64>) -> impl Responder{
  match fetch_sell_bill_by_number(&state,number.into_inner()).await{
    Ok(dep) => HttpResponse::Ok().json(dep),
    Err(_)=> HttpResponse::InternalServerError().into()
  }
}

#[delete("/{id}")]
async fn delete_by_number(state : Data<AppState>,number : web::Path<i64>) -> impl Responder{
  let number = number.into_inner();
  match delete_sell_bill_by_number(&state,number).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/")]
async fn save(state : Data<AppState>,dep : web::Json<SellBill>) -> impl Responder{
  let dep = dep.into_inner();
  match save_sell_bill(&state,dep).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[put("/")]
async fn update(state : Data<AppState>,dep : web::Json<SellBill>) -> impl Responder{
  let dep = dep.into_inner();
  match update_sell_bill(&state,dep).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}
