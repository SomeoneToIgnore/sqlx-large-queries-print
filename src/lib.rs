use sqlx::mysql::MySqlPoolOptions;

pub const DB_CONNECTION_URL: &str = "mysql://test_user:test_password@localhost:3306/test_database";
pub const NUMBER_OF_ITEMS_TO_INSERT: usize = 6_500;
pub const INSERT_REPEAT_TIMES: usize = 30;
pub const FINAL_SLEEP_DURATION_SECONDS: u64 = 5;

pub async fn init_tests() {
    let mut log_builder = env_logger::builder();
    log_builder.filter_level(log::LevelFilter::Debug);
    log_builder.init();

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

pub fn get_insert_sql() -> String {
    format!(
        "insert into test_table(a) values {}",
        sql_parameter_groups_string(1, NUMBER_OF_ITEMS_TO_INSERT)
    )
}

fn sql_parameter_groups_string(group_size: usize, groups_count: usize) -> String {
    let mut parameters_group = "?,".repeat(group_size);
    parameters_group.pop(); // trailing comma
    let mut parameters_group = format!("({}),", parameters_group).repeat(groups_count);
    parameters_group.pop(); // trailing comma
    parameters_group
}
