use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::Pool;

mod postgres;

#[get("/healthcheck")]
async fn verify_db_connection(pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.expect("couldn't get postgres client");
    let row = client
        .query_one("SELECT 1", &[])
        .await
        .expect("could not query db");

    let mut response_str = String::from("");

    let result: i32 = row.get(0);
    response_str += if result == 1 {
        "DB connection is good!\n"
    } else {
        "Unexpected result from query.\n"
    };

    let table_exists: bool = postgres::table_exists(&pool, "queue").await;
    response_str += if table_exists {
        "Table 'queue' exists.\n"
    } else {
        "Table 'queue' does not exist.\n"
    };

    HttpResponse::Ok().body(response_str)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Creating pool");
    let pool = postgres::create_pool();

    // create queue table if required
    if !postgres::table_exists(&pool, "queue").await {
        println!("Creating queue table");
        postgres::migrate_up(&pool).await;
    }

    println!("Launching webserver");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Add pool to application state
            .service(verify_db_connection)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
