use crate::blockchain::transaction::BCTransaction;
use crate::database::{create_product, fetch_auth_resource, fetch_products};
use crate::routes::utils::{decode_private_key, deserialize_jwt};
use crate::routes::CreateProductRequest;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use sawtooth_sdk::signing::secp256k1::Secp256k1PrivateKey;
use sawtooth_sdk::signing::CryptoFactory;

pub fn create_product_endpoint(
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

    let (_, username) = deserialize_jwt(data.jwt_sign.as_bytes(), header);

    // Fetching user data in the database
    let db = data.database_connection.get().unwrap();
    let private_key = decode_private_key(username.clone(), &db);
    let auth_info = fetch_auth_resource(username.clone(), &db);

    // Create transaction
    let transaction = BCTransaction::new("trade".to_string(), "1.0".to_string());
    let crypto_factory = CryptoFactory::new(&*transaction.context);

    // PrivateKey setup
    let key = &Secp256k1PrivateKey::from_hex(&hex::encode(private_key.to_vec())).unwrap();
    let signer = crypto_factory.new_signer(key);

    // Serialize payload and send to TP
    let batch = transaction.store_product(signer, auth_info.public_key.clone(), &product);
    transaction.send_zeromq(data.sawtooth_connection.clone(), batch);

    create_product(
        auth_info.id,
        &product.record_id.clone(),
        &product.title.clone(),
        product.price,
        product.latitude,
        product.longitude,
        &db
    );

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

    let (_, username) = deserialize_jwt(data.jwt_sign.as_bytes(), header);
    let db = data.database_connection.get().unwrap();
    let auth_info = fetch_auth_resource(username.clone(), &db);

    HttpResponse::Ok().json(fetch_products(auth_info.id, &db))
}
