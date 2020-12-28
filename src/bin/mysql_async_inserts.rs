use large_sql_inserts::{
    get_insert_sql, init_tests, DB_CONNECTION_URL, FINAL_SLEEP_DURATION_SECONDS,
    INSERT_REPEAT_TIMES, NUMBER_OF_ITEMS_TO_INSERT,
};
use mysql_async::{prelude::Queryable, Params, Value};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tests().await;

    let insert_sql = get_insert_sql();
    let query_parameters = Params::Positional(vec![Value::from(1); NUMBER_OF_ITEMS_TO_INSERT]);

    let pool = mysql_async::Pool::new(DB_CONNECTION_URL);
    let mut tx = pool.start_transaction(Default::default()).await?;

    log::info!(
        "Inserting {} elements {} times",
        NUMBER_OF_ITEMS_TO_INSERT,
        INSERT_REPEAT_TIMES
    );

    for i in 1..INSERT_REPEAT_TIMES + 1 {
        let start = Instant::now();
        tx.exec_drop(insert_sql.as_str(), query_parameters.clone())
            .await?;
        log::info!("Inserted {}th batch, in {:?}", i, start.elapsed())
    }

    tx.commit().await?;

    log::info!(
        "Successfully inserted the data waiting {} seconds before exiting",
        FINAL_SLEEP_DURATION_SECONDS
    );
    Ok(())
}
