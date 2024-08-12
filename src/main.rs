use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // with env to setting host serve and port
    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("8999".to_string());
    let serve_port = format!("{}:{}", host, port);

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    log::info!("Server running at http://{}", serve_port);

    // create server
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default() // actix_cors cors
                    .send_wildcard() // .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"]) // support GET, POST
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600), // 60 min -> 1 hour
            )
            .wrap(Logger::default()) // actix-web logger
            .configure(routes::config_service)
            .service(web::scope("/api").configure(routes::config_api_service))
    })
    .bind(serve_port)?
    .run()
    .await
}
