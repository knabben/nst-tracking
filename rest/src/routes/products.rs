use crate::{AppState};
use crate::routes::{CreateProductRequest};
use actix_web::{http, web, HttpResponse};

pub fn create_products(
  item: web::Json<CreateProductRequest>, data: web::Data<AppState>
) -> HttpResponse {
  HttpResponse::Ok().json("")
}