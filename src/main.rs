mod keypair;
mod token;
mod message;
mod transfer;
mod error;

use actix_web::{web, App, HttpServer, Responder, HttpResponse, http::Method};
use serde::{Serialize};
use std::env;
use keypair::generate_keypair;
use token::{CreateTokenRequest, MintTokenRequest, create_token_instruction, mint_token_instruction};
use message::{SignMessageRequest, VerifyMessageRequest, sign_message, verify_message};
use transfer::{SendSolRequest, SendTokenRequest, create_sol_transfer_instruction, create_token_transfer_instruction};
use error::ApiError;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_cors::Cors;


#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(error: String) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::success("Server is running"))
}

async fn handle_generate_keypair() -> impl Responder {
    let keypair = generate_keypair();
    HttpResponse::Ok().json(ApiResponse::success(keypair))
}

async fn handle_create_token(
    request: web::Json<CreateTokenRequest>,
) -> impl Responder {
    match create_token_instruction(request.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

async fn handle_mint_token(
    request: web::Json<MintTokenRequest>,
) -> impl Responder {
    match mint_token_instruction(request.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

async fn handle_sign_message(
    request: web::Json<SignMessageRequest>,
) -> impl Responder {
    match sign_message(request.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

async fn handle_verify_message(
    request: web::Json<VerifyMessageRequest>,
) -> impl Responder {
    match verify_message(request.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

async fn handle_send_sol(
    request: web::Json<SendSolRequest>,
) -> Result<HttpResponse, ApiError> {
    match create_sol_transfer_instruction(request.into_inner()) {
        Ok(response) => Ok(HttpResponse::Ok().json(ApiResponse::success(response))),
        Err(e) => Err(ApiError::ValidationError(e)),
    }
}

async fn handle_send_token(
    request: web::Json<SendTokenRequest>,
) -> impl Responder {
    match create_token_transfer_instruction(request.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("0.0.0.0:{}", port);

    log::info!("Starting server on {}", address);

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(10)
        .burst_size(5)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .wrap(DefaultHeaders::new()
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("X-Frame-Options", "DENY"))
                .add(("X-XSS-Protection", "1; mode=block"))
                .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
            )
            .wrap(Governor::new(&governor_conf))
            .service(web::resource("/health").to(health_check))
            .service(web::resource("/keypair")
                .route(web::post().to(handle_generate_keypair))
                .route(web::method(Method::OPTIONS).to(|| HttpResponse::Ok()))
            )
            .service(web::resource("/token/create")
                .route(web::post().to(handle_create_token))
                .route(web::method(Method::OPTIONS).to(|| HttpResponse::Ok()))
            )
            .service(web::resource("/token/mint")
                .route(web::post().to(handle_mint_token))
                .route(web::method(Method::OPTIONS).to(|| HttpResponse::Ok()))
            )
            .service(web::resource("/message/sign")
                .route(web::post().to(handle_sign_message))
                .route(web::method(Method::OPTIONS).to(|| HttpResponse::Ok()))
            )
            .service(web::resource("/message/verify")
                .route(web::post().to(handle_verify_message))
                .route(web::method(Method::OPTIONS).to(|| HttpResponse::Ok()))
            )
            .service(web::resource("/send/sol")
                .route(web::post().to(handle_send_sol))
                .route(web::method(Method::OPTIONS).to(|| HttpResponse::Ok()))
            )
            .service(web::resource("/send/token")
                .route(web::post().to(handle_send_token))
                .route(web::method(Method::OPTIONS).to(|| HttpResponse::Ok()))
            )
    })
    .bind(&address)?
    .run()
    .await
}

