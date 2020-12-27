use large_sql_inserts::{DB_CONNECTION_URL, init_tests, INSERT_REPEAT_TIMES, FINAL_SLEEP_DURATION_SECONDS, NUMBER_OF_ITEMS_TO_INSERT, sample_insert_value, get_insert_sql};
use sqlx::mysql::MySqlPoolOptions;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tests().await;

    let insert_sql = get_insert_sql();
    let insert_values = vec![sample_insert_value(); NUMBER_OF_ITEMS_TO_INSERT];

    let pool = MySqlPoolOptions::new()
        // for consistency, put the same limits as the mysql_async default ones
        .min_connections(10)
        .max_connections(100)
        .connect(DB_CONNECTION_URL)
        .await?;
    let mut tx = pool.begin().await?;

    log::info!("Inserting {} elements {} times", NUMBER_OF_ITEMS_TO_INSERT, INSERT_REPEAT_TIMES);

    for i in 1..INSERT_REPEAT_TIMES + 1 {
        let query = insert_values.iter().fold(sqlx::query(&insert_sql), |query, sample_value| {
            query
                .bind(sample_value.a)
                .bind(sample_value.b)
                .bind(sample_value.c)
                .bind(sample_value.d)
                .bind(sample_value.e)
                .bind(sample_value.f)
                .bind(sample_value.g)
                .bind(sample_value.h)
                .bind(sample_value.i.clone())
                .bind(sample_value.j)
        });
        let start = Instant::now();
        query.execute(&mut tx).await?;
        log::info!("Inserted {}th batch, in {:?}", i, start.elapsed())
    }

    tx.commit().await?;

    log::info!("Successfully inserted the data waiting {} seconds before exiting", FINAL_SLEEP_DURATION_SECONDS);
    std::thread::sleep(Duration::from_secs(FINAL_SLEEP_DURATION_SECONDS));
    Ok(())
}
