use std::io::{Write, stdin, stdout};

use df_sdk::credentials::Credentials;
use df_sdk::sdk::DeltaForceSdk;
use tokio_stream::StreamExt;

pub async fn list(limit: Option<usize>) {
    print!("请输入 Cookies：");
    stdout().flush().unwrap();

    let mut cookies = String::new();
    stdin().read_line(&mut cookies).unwrap();

    let sdk = DeltaForceSdk::build()
        .with_credentials(Credentials::from_cookies(&cookies).unwrap())
        .build();

    let mut stream = sdk.iter_battle_records().await;
    if let Some(x) = limit {
        stream = Box::pin(stream.take(x));
    }
    while let Some(item) = stream.next().await {
        match item {
            Ok(x) => match serde_json::to_string(&x) {
                Ok(json) => println!("{}", json),
                Err(e) => eprintln!("序列化为 JSON 失败：{}", e),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
