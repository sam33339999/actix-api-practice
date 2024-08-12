use actix_web::{web, Responder};
use serde::Serialize;

pub(crate) fn config_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").to(|| async { "Hello, world!" }));
}

pub(crate) fn config_api_service(cfg: &mut web::ServiceConfig) {
    // cfg.service(web::resource("/health").to(|| async { "Healthy !" }));
    // cfg.service(web::get("/healthz").to(healthz))

    cfg.route("healthz", web::get().to(healthz));
    cfg.service(web::resource("/config").to(|| async { "Route: /app/config" }));
}

#[derive(Debug, Serialize)]
pub struct ServiceInfo {
    version: String,
    status: String,
}
pub async fn healthz() -> impl Responder {
    web::Json(ServiceInfo {
        version: "0.0.1".to_string(),
        status: "Healthy".to_string(),
    })
}
