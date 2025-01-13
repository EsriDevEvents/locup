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

    #[tokio::test]
    async fn test_get_elevation() {
        // Check whether or not an ArcGIS API key was set using the environment
        assert!(env::var("arcgis_api_key").is_ok());

        // Create a default configuration
        let mut configuration = apis::configuration::Configuration::default();
        
        // Define the API key
        configuration.api_key = Some(apis::configuration::ApiKey {
            prefix: None,
            key: env::var("arcgis_api_key").expect("arcgis_api_key not set!")
        });
    }
}
