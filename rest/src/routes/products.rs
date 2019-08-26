use crate::{AppState};
use crate::routes::{CreateProductRequest};
use crate::routes::utils::{deserialize_jwt, decode_private_key};
use actix_web::{http, web, HttpResponse, HttpRequest};
use crate::blockchain::{serialize_product_payload, serialize_tp_payload};
use crate::blockchain::transaction::{BCTransaction};
use sawtooth_sdk::signing::{CryptoFactory};
use sawtooth_sdk::signing::secp256k1::{Secp256k1PrivateKey};

pub fn create_products(
  item: web::Json<CreateProductRequest>, 
  _request: HttpRequest,
  data: web::Data<AppState>
) -> HttpResponse {
  let product: CreateProductRequest = item.0;

  let header = match _request.headers().get("Authorization".to_string()) {
    Some(x) => x.to_str().unwrap(),
    None => "",
  };

  let (authenticated, username) = deserialize_jwt(data.jwt_sign.as_bytes(), header);

  let db = data.database_connection.get().unwrap();
  let private_key = decode_private_key(username, &db);

  let transaction = BCTransaction::new(
      "trade".to_string(),
      "1.0".to_string(),
      "00".to_string()
      );

  let crypto_factory = CryptoFactory::new(&*transaction.context);
  let key = Secp256k1PrivateKey::from_hex(&hex::encode(private_key.to_vec())).unwrap();

  let signer = crypto_factory.new_signer(&key);
  let payload = serialize_product_payload(
    product.record_id,
    product.title,
  );

  println!("{:?}", payload);
  HttpResponse::Ok().json("Create record transaction submitted")
}