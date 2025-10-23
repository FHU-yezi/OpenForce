use constants::map::Map;
use df_sdk::sdk::DeltaForceSdk;

fn output(room_password: Vec<(Map, u16)>) {
    for (map, password) in room_password {
        println!("{}：{}", map.as_str(), password);
    }
}

pub async fn get(sdk: DeltaForceSdk) {
    let room_password = match sdk.get_room_password().await {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    output(room_password);
}
