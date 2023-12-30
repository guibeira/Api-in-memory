use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::sync::Arc;
use std::sync::Mutex;
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod routes;
use models::{Number, Numbers};
use routes::routes;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::list_numbers,
        routes::get_number,
        routes::save_number,
    ),
    components(
        schemas(Numbers, Number)
    ),
    tags(
        (name = "Rust REST API", description = "Authentication in Rust Endpoints")
    ),
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();

    let numbers_db = Numbers::new();
    let numbers_db = Arc::new(Mutex::new(numbers_db));
    let db_state = web::Data::new(numbers_db);

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(db_state.clone())
            .service(web::scope("api/numbers").configure(routes))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
