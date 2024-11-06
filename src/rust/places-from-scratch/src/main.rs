use std::{collections::HashMap, env};

use reqwest::Client;

async fn get_request_body(
    client: &Client,
    request_params: PlacesRequestParams,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    params.insert("x", request_params.x.to_string());
    params.insert("y", request_params.y.to_string());
    params.insert("pageSize", request_params.page_size.to_string());
    params.insert("f", request_params.f);
    let request = client.get("https://places-api.arcgis.com/arcgis/rest/services/places-service/v1/places/near-point").query(&params).bearer_auth(request_params.token);

    let response = request.send().await?;
    Ok(response.text().await?)
}

struct PlacesRequestParams {
    x: f32,
    y: f32,
    page_size: u8,
    f: String,
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;
    let token = env::var("TOKEN")?;

    let request_params = PlacesRequestParams {
        x: 174.778,
        y: -41.292,
        page_size: 20,
        f: "pjson".to_string(),
        token,
    };

    let body = get_request_body(&client, request_params).await;

    match body {
        Ok(result) => println!("{result}"),
        Err(err) => println!("Something went wrong! Error: {err}"),
    }

    Ok(())
}
