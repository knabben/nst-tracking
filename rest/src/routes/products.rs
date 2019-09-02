use crate::blockchain::serialize_product_payload;
use crate::blockchain::transaction::BCTransaction;
use crate::blockchain::utils;
use crate::database::{create_auth, fetch_auth_resource, fetch_products};
use crate::routes::utils::{decode_private_key, deserialize_jwt};
use crate::routes::CreateProductRequest;
use crate::AppState;
use actix_web::{http, web, HttpRequest, HttpResponse};
use protobuf::RepeatedField;
use sawtooth_sdk::messages::client_batch_submit::ClientBatchSubmitRequest;
use sawtooth_sdk::messages::validator::Message_MessageType;
use sawtooth_sdk::signing::secp256k1::Secp256k1PrivateKey;
use sawtooth_sdk::signing::CryptoFactory;
use uuid::Uuid;

pub fn create_products(
    item: web::Json<CreateProductRequest>,
    _request: HttpRequest,
    data: web::Data<AppState>,
) -> HttpResponse {
    // Authentication
    let product: CreateProductRequest = item.0;
    let header = match _request.headers().get("Authorization".to_string()) {
        Some(x) => x.to_str().unwrap(),
        None => "",
    };

    let (authenticated, username) = deserialize_jwt(data.jwt_sign.as_bytes(), header);

    // Fetching user data in the database
    let db = data.database_connection.get().unwrap();
    let private_key = decode_private_key(username.clone(), &db);
    let auth_info = fetch_auth_resource(username.clone(), &db);

    // Create transaction
    let transaction = BCTransaction::new("trade".to_string(), "1.0".to_string(), "00".to_string());
    let crypto_factory = CryptoFactory::new(&*transaction.context);

    // PrivateKey setup
    let key = &Secp256k1PrivateKey::from_hex(&hex::encode(private_key.to_vec())).unwrap();
    let signer = crypto_factory.new_signer(key);

    // Serialize payload and send to TP
    let batch = transaction.store_product(signer, auth_info.public_key.clone(), &product);
    transaction.send_zeromq(data.sawtooth_connection.clone(), batch);

    HttpResponse::Ok().json("Create record transaction submitted")
}

pub fn list_products(
    _request: HttpRequest,
    data: web::Data<AppState>,
) -> HttpResponse {
    let header = match _request.headers().get("Authorization".to_string()) {
        Some(x) => x.to_str().unwrap(),
        None => "",
    };

    let (authenticated, username) = deserialize_jwt(data.jwt_sign.as_bytes(), header);

    let db = data.database_connection.get().unwrap();
    let private_key = decode_private_key(username.clone(), &db);
    let auth_info = fetch_auth_resource(username.clone(), &db);
    
    fetch_products(auth_info.id, &db);

    HttpResponse::Ok().json("")
}
