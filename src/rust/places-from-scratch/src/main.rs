use std::env;

async fn get_request_body() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let token = env::var("TOKENNN")?;

    let url = format!("https://places-api.arcgis.com/arcgis/rest/services/places-service/v1/places/near-point?x=174.778&y=-41.292&pageSize=20&f=pjson&token={token}");

    let request = client.request(reqwest::Method::GET, url);

    let response = request.send().await?;
    Ok(response.text().await?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = get_request_body().await;

    match body {
        Ok(result) => println!("{result}"),
        Err(err) => println!("Something went wrong! Error: {err}"),
    }

    Ok(())
}
