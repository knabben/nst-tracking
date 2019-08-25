use crate::{AppState};
use crate::routes::{CreateProductRequest};
use crate::routes::utils::{deserialize_jwt, decode_private_key};
use actix_web::{http, web, HttpResponse, HttpRequest};

pub fn create_products(
  item: web::Json<CreateProductRequest>, 
  _request: HttpRequest,
  data: web::Data<AppState>
) -> HttpResponse {

  let header = match _request.headers().get("Authorization".to_string()) {
    Some(x) => x.to_str().unwrap(),
    None => "",
  };

  let (authenticated, username) = deserialize_jwt(data.jwt_sign.as_bytes(), header);

  let db = data.database_connection.get().unwrap();
  decode_private_key(username, &db);

  HttpResponse::Ok().json("")
}