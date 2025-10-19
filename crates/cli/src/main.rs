use df_sdk::credentials::Credentials;
use df_sdk::sdk::DeltaForceSdk;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    print!("请输入 Cookies：");
    stdout().flush().unwrap();

    let mut cookies = String::new();
    stdin().read_line(&mut cookies).unwrap();

    let sdk = DeltaForceSdk::build()
        .with_credentials(Credentials::from_cookies(&cookies).unwrap())
        .build();

    let mut battle_records_stream = sdk.iter_battle_records().await;
    while let Some(battle_record) = battle_records_stream.next().await {
        println!("{:#?}", battle_record.unwrap());
    }
}
