use std::io::{Write, stdin, stdout};

use df_sdk::credentials::Credentials;
use df_sdk::sdk::DeltaForceSdk;
use tokio_stream::StreamExt;

pub async fn export_battle_records(limit: Option<usize>) {
    print!("请输入 Cookies：");
    stdout().flush().unwrap();

    let mut cookies = String::new();
    stdin().read_line(&mut cookies).unwrap();

    let sdk = DeltaForceSdk::build()
        .with_credentials(Credentials::from_cookies(&cookies).unwrap())
        .build();

    let mut battle_records_stream = sdk.iter_battle_records().await;
    if let Some(x) = limit {
        battle_records_stream = Box::pin(battle_records_stream.take(x));
    }
    while let Some(battle_record) = battle_records_stream.next().await {
        match battle_record {
            Ok(x) => match serde_json::to_string(&x) {
                Ok(json) => println!("{}", json),
                Err(e) => eprintln!("序列化为 JSON 失败：{}", e),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
