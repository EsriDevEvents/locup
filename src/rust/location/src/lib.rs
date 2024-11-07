#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate serde_repr;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate reqwest;

pub mod apis;
pub mod models;


mod tests {
    use apis::configuration::ApiKey;

    use super::*;
    use std::env;
    use crate::apis::styles_api::arcgis_community_style_get;

    #[tokio::test]
    async fn test_get_styles() {
        // Check whether or not an ArcGIS API key was set using the environment
        //env::set_var("arcgis_api_key", "test_key");
        assert!(env::var("arcgis_api_key").is_ok());

        // Create a default configuration
        let mut configuration = apis::configuration::Configuration::default();
        
        // Modify the API key
        configuration.api_key = Some(apis::configuration::ApiKey {
            prefix: None,
            key: "<Not-an-API-key!>".to_string(),
        });

        // Make it fail!
        //configuration.base_path += "/no-endpoint";

        let result = apis::self_description_api::service_self_get(&configuration).await;

        // Assert that the result is Ok, and fail the test if it's Err
        if let Err(err) = &result {
            match err {
                apis::Error::ResponseError(response) => {
                    eprintln!("Request failed with status: {}", response.status);
                    eprintln!("Error message: {}", response.content);
                    if let Some(entity) = &response.entity {
                        eprintln!("Error details: {:?}", entity);
                    }
                }
                _ => eprintln!("An unexpected error occurred: {:?}", err),
            }
            panic!("Test failed due to error");
        }

        if let Ok(response) = result {
            // Assuming response is of type ServiceSelf
            println!("Service Name: {}", response.name);
            println!("Service Version: {}", response.version);
            println!("Service Description: {}", response.description);
            println!("Supported Languages: {:?}", response.languages);
            println!("Supported Worldviews: {:?}", response.worldviews);
            println!("Supported Places: {:?}", response.places);
            println!("Self URL: {}", response.self_url);
            println!("Styles URL: {}", response.styles_url);
            println!("Webmaps URL: {}", response.webmaps_url);
        }
    }
}