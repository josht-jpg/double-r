use actix_web::{
    http::header::ContentType, middleware, options, post, web, App, HttpResponse, HttpServer,
    Responder,
};
use reqwest::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
};

use serde::{Deserialize, Serialize};

#[options("/")]
async fn options() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct IncrementInput {
    count: i64,
}

#[derive(Serialize)]
struct IncrementOutput {
    count: i64,
}

#[options("/increment")]
async fn increment_preflight() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/increment")]
async fn increment(input: web::Json<IncrementInput>) -> impl Responder {
    let output = IncrementOutput {
        count: input.count + 1,
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(output)
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // let db_pool = PgPoolOptions::new()
    //     .max_connections(10)
    //     .connect(&env::var("DATABASE_URL").unwrap())
    //     .await
    //     .expect("Failed to connect to Postgres");

    // let db_pool = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .wrap(
                middleware::DefaultHeaders::new()
                    .add((ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:3000"))
                    .add((
                        ACCESS_CONTROL_ALLOW_METHODS,
                        "GET, PUT, POST, DELETE, OPTIONS",
                    ))
                    .add((
                        ACCESS_CONTROL_ALLOW_HEADERS,
                        "content-type, Authorization, X-Requested-With",
                    )),
            )
            .service(options)
            .service(increment)
            .service(increment_preflight)
        // .app_data(db_pool.clone())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}
