#[path = "lib/mod.rs"]
mod lib;
mod routes;
mod state;

use actix_cors::Cors;
use actix_web::{error, http::StatusCode, web, App, HttpResponse, HttpServer};
use errors::core::ErrorStatuses;
use lib::errors::make_error;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use state::AppState;

fn get_env_filename() -> String {
    let env = (|| match std::env::var("ENVIRONMENT") {
        Ok(val) => val,
        _ => return String::from("development"),
    })();
    let mut base_filename = String::from(".env");

    return match env.as_str() {
        "development" => base_filename,
        _ => {
            base_filename.push_str(".");
            base_filename.push_str(&env);
            return base_filename;
        }
    };
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenvy::from_filename(get_env_filename())
        .ok()
        .expect("env file not found.");
    env_logger::init();

    let mut opt =
        ConnectOptions::new(std::env::var("DATABASE_URL").expect("DATABASE_URL not defined."))
            .to_owned();

    opt.max_connections(20).min_connections(1);

    let db: DatabaseConnection = Database::connect(opt).await.unwrap();
    let state: AppState = AppState {
        postgres: db,
        jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET not defined."),
    };
    let json_config = web::JsonConfig::default().error_handler(|err, _req| {
        error::InternalError::from_response(
            err,
            HttpResponse::build(StatusCode::BAD_REQUEST)
                .content_type("application/json")
                .json(make_error(ErrorStatuses::InvalidContent)),
        )
        .into()
    });
    let path_config = web::PathConfig::default().error_handler(|err, _req| {
        error::InternalError::from_response(
            err,
            HttpResponse::build(StatusCode::BAD_REQUEST)
                .content_type("application/json")
                .json(make_error(ErrorStatuses::InvalidPath)),
        )
        .into()
    });

    Migrator::up(&state.postgres, None).await.ok();
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .app_data(json_config.clone())
            .app_data(path_config.clone())
            .service(web::scope("/api").service(
                web::scope("/v1").service(web::scope("/auth").service(routes::auth::login)),
            ))
    })
    .bind((std::net::Ipv4Addr::LOCALHOST, 1338))?
    .run()
    .await
}
