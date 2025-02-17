mod error_enums;
mod message_structs;
mod postgres;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::Pool;
use message_structs::{ErrorResponse, HealthCheckResponse, SuccessResponse};

#[get("/api/healthcheck")]
async fn healthcheck_endpoint(pool: web::Data<Pool>) -> impl Responder {
    let connection_status: bool = match postgres::check_postgres_connection(&pool).await {
        Ok(connection_status) => connection_status,
        Err(_) => false,
    };

    let table_exists: bool = match postgres::table_exists(&pool, "queue").await {
        Ok(table_exists) => table_exists,
        Err(_) => false,
    };

    HttpResponse::Ok().json(HealthCheckResponse {
        db_connection: connection_status,
        queue_table_exists: table_exists,
    })
}

#[post("/api/add-job")]
async fn add_job_endpoint(request_body: String, pool: web::Data<Pool>) -> impl Responder {
    match serde_json::from_str(&request_body) {
        Ok(parsed_request) => match postgres::add_job_to_queue(parsed_request, &pool).await {
            Ok(_) => HttpResponse::Ok().json(SuccessResponse { success: true }),
            Err(_) => HttpResponse::Ok().json(SuccessResponse { success: false }),
        },
        Err(_) => HttpResponse::BadRequest().json(ErrorResponse {
            error: String::from("Cannot deserialise request"),
        }),
    }
}

#[get["/api/view-jobs"]]
async fn view_jobs_endpoint(pool: web::Data<Pool>) -> impl Responder {
    match postgres::view_jobs_in_queue(&pool).await {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(_) => HttpResponse::Ok().json(SuccessResponse { success: false }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Creating pool");
    let pool = postgres::create_pool();

    // create queue table if required
    if !postgres::table_exists(&pool, "queue")
        .await
        .expect("could not verify if queue table exists to run migration")
    {
        println!("Creating queue table");
        postgres::migrate_up(&pool).await;
    }

    println!("Launching webserver");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Add pool to application state
            .service(healthcheck_endpoint)
            .service(add_job_endpoint)
            .service(view_jobs_endpoint)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
