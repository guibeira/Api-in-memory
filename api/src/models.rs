use std::collections::HashMap;

use actix_web::Responder;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Numbers {
    pub numbers: HashMap<usize, String>,
}

impl Numbers {
    pub fn new() -> Numbers {
        Numbers {
            numbers: HashMap::new(),
        }
    }
}

impl Responder for Numbers {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        let body = serde_json::to_string(&self).unwrap();
        actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Number {
    pub number: usize,
    pub result: String,
}

impl Number {
    pub fn new(number: usize, result: String) -> Self {
        Self { number, result }
    }
}

impl Responder for Number {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        let body = serde_json::to_string(&self).unwrap();
        actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}
