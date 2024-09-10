mod bert;
mod error_enums;
mod message_structs;
mod postgres;

use deadpool_postgres::Pool as DeadPool;
use message_structs::QueueEntry;
use tokio::time::{sleep, Duration};

fn main() {
    bert::main();
}

/*
async fn process_queue(tasks: Vec<QueueEntry>, pool: &DeadPool) {
    for task in tasks {
        if !task.being_processed && !task.complete {
            /*match model::run_inference(task.input) {
                Ok(output) => postgres::update_row(task.id, output, pool),
                Err(_) => {},
            }*/
        }
    }
}

async fn runner(pool: DeadPool) {
    // loop every second
    loop {
        match postgres::view_jobs_in_queue(&pool).await {
            Ok(queue) => {
                let tasks: Vec<QueueEntry> = queue.queue_entries;
                // find most recent unperformed job
                process_queue(tasks, &pool).await;
            }
            Err(_) => {}
        }
        sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    let pool = postgres::create_pool();
    if !(postgres::check_postgres_connection(&pool)
        .await
        .expect("could not connect to db")
        && postgres::table_exists(&pool, "queue")
            .await
            .expect("could not verify table exists"))
    {
        panic!("could not verify postgres")
    }
    println!("connected to database");

    runner(pool).await;
}*/
