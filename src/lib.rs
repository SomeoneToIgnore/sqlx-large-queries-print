use sqlx::mysql::MySqlPoolOptions;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rust_decimal::Decimal;

pub const DB_CONNECTION_URL: &str = "mysql://test_user:test_password@localhost:3306/test_database";
pub const NUMBER_OF_ITEMS_TO_INSERT: usize = 6_500;
pub const INSERT_REPEAT_TIMES: usize = 30;
pub const FINAL_SLEEP_DURATION_SECONDS: u64 = 5;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct TestTable {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
    pub e: NaiveDate,
    pub f: NaiveDate,
    pub g: NaiveDateTime,
    pub h: NaiveDateTime,
    pub i: String,
    pub j: Decimal,
}

pub async fn init_tests() {
    let mut log_builder = env_logger::builder();
    log_builder.filter_level(log::LevelFilter::Debug);
    log_builder.init();

    let pool = MySqlPoolOptions::new()
        .min_connections(10)
        .max_connections(100)
        .connect(DB_CONNECTION_URL)
        .await.expect("Failed to connect for the migration");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await.expect("Failed to perform the test migration");
    log::info!("Migration successful");
}

const INSERT_SQL_START: &str = "insert into test_table(a, b, c, d, e, f, g, h, i, j) values ";

pub fn get_insert_sql() -> String {
    // TODO kb very implicit dependency on the number of fields in struct
    format!("{}{}", INSERT_SQL_START, sql_parameter_groups_string(10, NUMBER_OF_ITEMS_TO_INSERT))
}

pub fn sample_insert_value() -> TestTable {
    TestTable {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: NaiveDate::from_ymd(2020, 1, 1),
        f: NaiveDate::from_ymd(2021, 1, 1),
        g: NaiveDateTime::new(NaiveDate::from_ymd(2020, 1, 1), NaiveTime::from_hms(1, 1, 1)),
        h: NaiveDateTime::new(NaiveDate::from_ymd(2021, 1, 1), NaiveTime::from_hms(1, 1, 1)),
        i: "test".to_string(),
        j: Decimal::new(2021, 2),
    }
}

fn sql_parameter_groups_string(group_size: usize, groups_count: usize) -> String {
    let mut parameters_group = "?,".repeat(group_size);
    parameters_group.pop(); // trailing comma
    let mut parameters_group = format!("({}),", parameters_group).repeat(groups_count);
    parameters_group.pop(); // trailing comma
    parameters_group
}

