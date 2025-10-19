use df_sdk::credentials::Credentials;
use df_sdk::sdk::DeltaForceSdk;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;

#[tokio::main]
async fn main() {
    print!("请输入 Cookies：");
    stdout().flush().unwrap();

    let mut cookies = String::new();
    stdin().read_line(&mut cookies).unwrap();

    let sdk = DeltaForceSdk::build()
        .with_credentials(Credentials::from_cookies(&cookies).unwrap())
        .build();

    for battle_record in sdk.iter_battle_records(1).await.unwrap() {
        println!("{:?}", battle_record);
    }
}
