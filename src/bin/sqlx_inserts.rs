use large_sql_inserts::{
    get_insert_sql, init_tests, DB_CONNECTION_URL, FINAL_SLEEP_DURATION_SECONDS,
    INSERT_REPEAT_TIMES, NUMBER_OF_ITEMS_TO_INSERT,
};
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    ConnectOptions,
};
use std::{str::FromStr, time::Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tests().await;

    let insert_sql = get_insert_sql();
    let insert_values = [1; NUMBER_OF_ITEMS_TO_INSERT];

    let mut connection_options = MySqlConnectOptions::from_str(DB_CONNECTION_URL)?;
    // uncomment this to improve the performance
    // connection_options
    //     .log_statements(log::LevelFilter::Off)
    //     .log_slow_statements(log::LevelFilter::Off, std::time::Duration::default());
    let pool = MySqlPoolOptions::new()
        // for consistency, put the same limits as the mysql_async default ones
        .min_connections(10)
        .max_connections(100)
        .connect_with(connection_options)
        .await?;
    let mut tx = pool.begin().await?;

    log::info!(
        "Inserting {} elements {} times",
        NUMBER_OF_ITEMS_TO_INSERT,
        INSERT_REPEAT_TIMES
    );

    for i in 1..INSERT_REPEAT_TIMES + 1 {
        let query = insert_values
            .iter()
            .fold(sqlx::query(&insert_sql), |query, sample_value| {
                query.bind(sample_value)
            });
        let start = Instant::now();
        query.execute(&mut tx).await?;
        log::info!("Inserted {}th batch, in {:?}", i, start.elapsed())
    }

    tx.commit().await?;

    log::info!(
        "Successfully inserted the data waiting {} seconds before exiting",
        FINAL_SLEEP_DURATION_SECONDS
    );
    Ok(())
}
