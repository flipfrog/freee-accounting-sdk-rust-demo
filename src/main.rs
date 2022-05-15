use r2d2_sqlite::{self, SqliteConnectionManager};
type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
use rusqlite::{params};
use openapi_sdk::apis::configuration::Configuration;
use openapi_sdk::apis::companies_api;

#[tokio::main]
async fn main() {
    // connect to SQLite
    let manager = SqliteConnectionManager::file("../freee-accounting-sdk-rust/examples/web/token.db");
    let pool = Pool::new(manager).unwrap();

    // fetch api access token
    let conn = pool.get()
        .expect("DB接続の取得に失敗しました");
    let mut statement = conn.prepare("SELECT token FROM token WHERE application_id = 'EXAMPLE'").unwrap();
    let rows = statement.query_map(params![], |row| {
        let token: String = row.get(0).unwrap();
        Ok(token)
    })
        .expect("DBからのアクセストークンの取得に失敗しました");
    let token = rows.reduce(|_a, t| t).unwrap().unwrap();

    // api configuration
    let config = Configuration {
        base_path: "https://api.freee.co.jp".to_string(),
        user_agent: None,
        client: reqwest::Client::new(),
        basic_auth: None,
        oauth_access_token: Some(token),
        bearer_access_token: None,
        api_key: None
    };

    // APIで事業所の一覧を取得する
    let companies = companies_api::get_companies(&config).await
        .expect("事業所一覧の取得に失敗しました");

    for company in &companies.companies {
        println!("- company.id: {}, company.display_name: {}",
                 company.id,
                 company.display_name.as_ref().unwrap_or(&"".to_string())
        );
    }

    // TODO: ループしてGPIOのポートがhighになったら、取引を登録する
}
