use actix_api::posts;
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

    // api for posts

    // cfg.route("posts", web::get().to(posts::get_posts_handle));
    cfg.service(
        web::scope("/posts").configure(posts::setup_route), // .route("", web::get().to(posts::get_posts_handle))
                                                            // .route("", web::post().to(posts::create_post_handle)),
    );
    // cfg.service(web::scope("/posts")) 這邊要注意，我自己很容易寫著寫著沒包進去裡面， service( 的 scope().configurate() ) 要記住要包進去在裡面
    //     .configure(posts::setup_route);
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
