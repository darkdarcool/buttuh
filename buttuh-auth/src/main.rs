use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeviceCodeResponse {
    pub user_code: String,
    pub device_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

const CLIENT_ID: &str = "00000000441cc96b";

#[tokio::main]
async fn main() {
    let client = Client::new();

    let resp = client
        .post("https://login.live.com/oauth20_connect.srf")
        .form(&vec![
            ("scope", "service::user.auth.xboxlive.com::MBI_SSL"),
            ("client_id", CLIENT_ID),
            ("response_type", "device_code"),
        ])
        .send()
        .await
        .unwrap()
        .json::<DeviceCodeResponse>()
        .await
        .unwrap();

    println!("{:?}", resp);
}
