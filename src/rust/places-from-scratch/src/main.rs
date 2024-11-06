async fn get_request_body() -> String {
    let client = reqwest::Client::builder().build().unwrap();

    let url = format!("https://places-api.arcgis.com/arcgis/rest/services/places-service/v1/places/near-point?x=174.778&y=-41.292&pageSize=20&f=pjson&token=TOKEN");

    let request = client.request(reqwest::Method::GET, url);

    let response = request.send().await.unwrap();
    response.text().await.unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = get_request_body().await;
    println!("{}", body);

    Ok(())
}
