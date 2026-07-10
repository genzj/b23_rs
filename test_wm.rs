#[tokio::main]
async fn main() {
    let s = b23_rs::test_helpers::setup_mock_server().await;
    let url = format!("{}/rlUSCcz", s.uri());
    println!("Requesting: {}", url);
    let resp = reqwest::get(&url).await.unwrap();
    println!("Status: {}", resp.status());
}
