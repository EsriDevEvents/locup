use std::{collections::HashMap, env};

async fn get_request_body() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let token = env::var("TOKEN")?;

    let mut params = HashMap::new();
    params.insert("x", "174.778");
    params.insert("y", "-41.292");
    params.insert("pageSize", "20");
    params.insert("f", "pjson");
    let request = client.get("https://places-api.arcgis.com/arcgis/rest/services/places-service/v1/places/near-point").query(&params).bearer_auth(token);

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
