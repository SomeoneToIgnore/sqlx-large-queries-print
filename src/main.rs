use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    ConnectOptions,
};
use std::{str::FromStr, time::Instant};

const DB_CONNECTION_URL: &str = "mysql://test_user:test_password@localhost:3306/test_database";
const NUMBER_OF_ITEMS_TO_INSERT: usize = 1_000;
const INSERT_REPEAT_TIMES: usize = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tests().await;

    let insert_sql = format!(
        "insert into test_table(a) values {}",
        sql_parameter_groups_string(1, NUMBER_OF_ITEMS_TO_INSERT)
    );
    let insert_values = [1; NUMBER_OF_ITEMS_TO_INSERT];

    let mut connection_options = MySqlConnectOptions::from_str(DB_CONNECTION_URL)?;
    // uncomment this to improve the performance
    // connection_options
    //     .log_statements(log::LevelFilter::Off)
    //     .log_slow_statements(log::LevelFilter::Off, std::time::Duration::default());
    let pool = MySqlPoolOptions::new()
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

    log::info!("Successfully inserted the data",);
    Ok(())
}

pub async fn init_tests() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let pool = MySqlPoolOptions::new()
        .min_connections(10)
        .max_connections(100)
        .connect(DB_CONNECTION_URL)
        .await
        .expect("Failed to connect for the migration");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to perform the test migration");
    log::info!("Migration successful");
}

fn sql_parameter_groups_string(group_size: usize, groups_count: usize) -> String {
    let mut parameters_group = "?,".repeat(group_size);
    parameters_group.pop(); // trailing comma
    let mut parameters_group = format!("({}),", parameters_group).repeat(groups_count);
    parameters_group.pop(); // trailing comma
    parameters_group
}
