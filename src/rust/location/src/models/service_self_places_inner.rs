/*
 * Basemap styles service
 *
 * The basemap styles service provides global basemap layers in a variety of styles to use in mapping applications. Basemap layers give geographic context to a map or scene by serving static tiles containing geographic reference data. This data includes, but is not limited to, topographic features, road networks, footpaths, building footprints, water features, administrative boundaries, and localized language place labels. The JSON structure of a basemap style can be returned as the [Mapbox Style Specification](https://docs.mapbox.com/mapbox-gl-js/style-spec/) or the Esri [Web Map Specification](https://developers.arcgis.com/web-map-specification/).
 *
 * The version of the OpenAPI document: 2.4.7
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSelfPlacesInner {
    /// Name of the places parameter.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Code for the places parameter.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl ServiceSelfPlacesInner {
    pub fn new() -> ServiceSelfPlacesInner {
        ServiceSelfPlacesInner {
            name: None,
            code: None,
        }
    }
}

