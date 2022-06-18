use std::env;
use r2d2_sqlite::{self, SqliteConnectionManager};
type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
use rusqlite::{params};
use openapi_sdk::apis::configuration::Configuration;
use openapi_sdk::apis::companies_api;
use openapi_sdk::apis::expense_applications_api::create_expense_application;
use openapi_sdk::models::ExpenseApplicationCreateParams;
use openapi_sdk::models::ExpenseApplicationCreateParamsExpenseApplicationLines;
use rppal::gpio::{Gpio, Trigger};

const GPIO_NO: u8 = 21;

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

    // 環境変数から、事業所IDを取得する
    let company_id = env::var("RUST_API_EXAMPLE_COMPANY_ID").expect("事業所IDの取得に失敗しました");

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

    // 事業所の一覧を取得する
    let companies = companies_api::get_companies(&config).await.expect("事業所一覧の取得に失敗しました");

    for company in &companies.companies {
        println!("- company.id: {}, company.display_name: {}",
                 company.id,
                 company.display_name.as_ref().unwrap_or(&"".to_string())
        );
    }

    // 経費精算申請のパラメータを作成する
    let line = ExpenseApplicationCreateParamsExpenseApplicationLines {
        transaction_date: Some("2022-06-10".to_string()),
        amount: Some(100),
        description: Some("コーヒー代".to_string()),
        receipt_id: None,
        expense_application_line_template_id: None
    };
    let params = ExpenseApplicationCreateParams {
        company_id: company_id.parse().unwrap(),
        title: "会議費".to_string(),
        expense_application_lines: vec!(line),
        approval_flow_route_id: None,
        approver_id: None,
        description: None,
        draft: Some(true),
        issue_date: Some("2022-06-10".to_string()),
        parent_id: None,
        section_id: None,
        segment_1_tag_id: None,
        segment_2_tag_id: None,
        segment_3_tag_id: None,
        tag_ids: None,
    };

    let gpio = Gpio::new().expect("Error in getting Gpio instance.");
    let gpio_port = gpio.get(GPIO_NO).expect("Error in getting GPIO Port.");
    let mut switch = gpio_port.into_input_pullup();
    switch.set_interrupt(Trigger::FallingEdge).expect("Error in setting interrupt.");

    // ループしてGPIOポート信号の立ち下がりで、取引を登録する
    loop {
        let level = switch.poll_interrupt(true, None);
        match level {
            Ok(_) => {
                create_expense_application(&config, Some(params.clone())).await.expect("経費申請の作成に失敗しました");
                println!("経費申請を作成しましました。");
            },
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }
}
