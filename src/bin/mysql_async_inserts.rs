use large_sql_inserts::{DB_CONNECTION_URL, init_tests, NUMBER_OF_ITEMS_TO_INSERT, INSERT_REPEAT_TIMES, get_insert_sql, sample_insert_value, FINAL_SLEEP_DURATION_SECONDS};
use mysql_async::{prelude::Queryable, Params, Value};
use tokio::time::Duration;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tests().await;

    let insert_sql = get_insert_sql();
    let insert_values = vec![sample_insert_value(); NUMBER_OF_ITEMS_TO_INSERT].into_iter().flat_map(|sample_value| vec![
        Value::from(sample_value.a),
        Value::from(sample_value.b),
        Value::from(sample_value.c),
        Value::from(sample_value.d),
        Value::from(sample_value.e),
        Value::from(sample_value.f),
        Value::from(sample_value.g),
        Value::from(sample_value.h),
        Value::from(sample_value.i),
        Value::from(sample_value.j),
    ]).collect::<Vec<_>>();

    let pool = mysql_async::Pool::new(DB_CONNECTION_URL);
    let mut tx = pool.start_transaction(Default::default()).await?;

    log::info!("Inserting {} elements {} times", NUMBER_OF_ITEMS_TO_INSERT, INSERT_REPEAT_TIMES);

    for i in 1..INSERT_REPEAT_TIMES + 1 {
        let start = Instant::now();
        tx.exec_drop(insert_sql.as_str(), Params::Positional(insert_values.clone())).await?;
        log::info!("Inserted {}th batch, in {:?}", i, start.elapsed())
    }

    tx.commit().await?;

    log::info!("Successfully inserted the data waiting {} seconds before exiting", FINAL_SLEEP_DURATION_SECONDS);
    std::thread::sleep(Duration::from_secs(FINAL_SLEEP_DURATION_SECONDS));
    Ok(())
}

