use crate::Numbers;
use crate::models::Number;
use actix_web::{web, HttpResponse, Responder};
use number_types::check_number;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;


#[utoipa::path(
    get,
    path = "/api/numbers/{number}",
    tag = "numbers",
    responses(
        (status=200, description = "Number details", body=Number),
        (status=404, description = "Number not found"),
        )
        
)]
async fn get_number(
    number: web::Path<usize>,
    numbers_db: web::Data<Arc<Mutex<Numbers>>>,
) -> impl Responder {
    let number = number.into_inner();
    let db = numbers_db.lock().unwrap();
    let result = db.numbers.get(&number);
    match result {
        Some(result) => {
            let data= Number::new(number, result.to_string());
            HttpResponse::Ok().json(data)
        }
        None => HttpResponse::NotFound().body("Not found"),
    }
}

#[utoipa::path(
    get,
    path = "/api/numbers",
    tag = "numbers",
    responses(
        (status=200, description = "List of numbers avaiable", body=Numbers),
        )
)]
async fn list_numbers(numbers_db: web::Data<Arc<Mutex<Numbers>>>) -> Numbers {
    let game_data = numbers_db.lock().unwrap();

    match game_data.numbers.len() {
        0 => Numbers {
            numbers: HashMap::new(),
        },
        _ => Numbers {
            numbers: game_data.numbers.clone(),
        },
    }
}


#[utoipa::path(
    post,
    path = "/api/numbers/",
    tag = "numbers",
    request_body(content = usize, content_type = "application/json", description = "Number creation", example = json!("1")),
    responses(
        (status=201, description = "Number creation success", body=usize),
        (status=409, description = "Number already exists"),
        )
        
)]
async fn save_number(
    number: web::Json<usize>,
    numbers_db: web::Data<Arc<Mutex<Numbers>>>,
) -> impl Responder {
    let number = number.into_inner();

    // check if number is on db
    let mut game_data = numbers_db.lock().unwrap();
    if let Some(_) = game_data.numbers.get(&number) {
        return HttpResponse::Conflict().body("Number already exists");
    }

    // insert number into db
    let result = check_number(number);
    game_data.numbers.insert(number, result.clone());
    let number = Number::new(number, result);
    HttpResponse::Created().json(number)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(list_numbers)))
        .service(web::resource("/").route(web::post().to(save_number)))
        .service(web::resource("/{number}").route(web::get().to(get_number)));
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::*;

    #[actix_rt::test]
    async fn test_get_number() {
        let mut numbers_db = Numbers::new();
        numbers_db.numbers.insert(1, "1".to_string());
        let numbers_db = Arc::new(Mutex::new(numbers_db));
        let db_state = web::Data::new(numbers_db);

        let mut app = test::init_service(
            App::new()
                .app_data(db_state.clone())
                .service(web::scope("api/numbers").configure(routes)),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/numbers/1").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let response: Number = serde_json::from_slice(&body).unwrap();
        assert_eq!(response.number, 1);
        assert_eq!(response.result, "1");
    }

    #[actix_rt::test]
    async fn test_list_numbers() {
        let mut numbers_db = Numbers::new();
        numbers_db.numbers.insert(1, "1".to_string());
        let numbers_db = Arc::new(Mutex::new(numbers_db));
        let db_state = web::Data::new(numbers_db);

        let mut app = test::init_service(
            App::new()
                .app_data(db_state.clone())
                .service(web::scope("api/numbers").configure(routes)),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/numbers").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        let response: Numbers = serde_json::from_slice(&body).unwrap();
        assert_eq!(response.numbers.len(), 1);
    }

    #[actix_rt::test]
    async fn test_save_number() {
        let mut numbers_db = Numbers::new();
        numbers_db.numbers.insert(1, "1".to_string());
        let numbers_db = Arc::new(Mutex::new(numbers_db));
        let db_state = web::Data::new(numbers_db);

        let mut app = test::init_service(
            App::new()
                .app_data(db_state.clone())
                .service(web::scope("api/numbers").configure(routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/numbers/")
            .set_json(&2)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 201);

        let body = test::read_body(resp).await;
        let response: Number = serde_json::from_slice(&body).unwrap();
        assert_eq!(response.number, 2);
        assert_eq!(response.result, "2");
    
    }

    #[actix_rt::test]
    async fn test_save_number_bad_request() {
        let mut numbers_db = Numbers::new();
        numbers_db.numbers.insert(1, "1".to_string());
        let numbers_db = Arc::new(Mutex::new(numbers_db));
        let db_state = web::Data::new(numbers_db);

        let mut app = test::init_service(
            App::new()
                .app_data(db_state.clone())
                .service(web::scope("api/numbers").configure(routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/numbers/")
            .set_json(&"invalid_number")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_rt::test]
    async fn test_save_number_conflict() {
        let mut numbers_db = Numbers::new();
        numbers_db.numbers.insert(1, "1".to_string());
        let numbers_db = Arc::new(Mutex::new(numbers_db));
        let db_state = web::Data::new(numbers_db);

        let mut app = test::init_service(
            App::new()
                .app_data(db_state.clone())
                .service(web::scope("api/numbers").configure(routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/numbers/")
            .set_json(&1)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 409);
    }

    #[actix_rt::test]
    async fn test_get_number_not_found() {
        let mut numbers_db = Numbers::new();
        numbers_db.numbers.insert(1, "1".to_string());
        let numbers_db = Arc::new(Mutex::new(numbers_db));
        let db_state = web::Data::new(numbers_db);

        let mut app = test::init_service(
            App::new()
                .app_data(db_state.clone())
                .service(web::scope("api/numbers").configure(routes)),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/numbers/2").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 404);
    }

    #[actix_rt::test]
    async fn test_get_number_bad_request() {
        let mut numbers_db = Numbers::new();
        numbers_db.numbers.insert(1, "1".to_string());
        let numbers_db = Arc::new(Mutex::new(numbers_db));
        let db_state = web::Data::new(numbers_db);

        let mut app = test::init_service(
            App::new()
                .app_data(db_state.clone())
                .service(web::scope("api/numbers").configure(routes)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/numbers/invalid_number")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 404);
    }
}
