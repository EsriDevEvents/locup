# \CustomWebmapsApi

All URIs are relative to *https://basemapstyles-api.arcgis.com/arcgis/rest/services/styles/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**webmaps_items_item_id_get**](CustomWebmapsApi.md#webmaps_items_item_id_get) | **GET** /webmaps/items/{item_id} | Portal item as a Webmap



## webmaps_items_item_id_get

> serde_json::Value webmaps_items_item_id_get(item_id, token, f)
Portal item as a Webmap

Returns a web map showing the supplied portal item. The portal item must be of [type](https://developers.arcgis.com/rest/users-groups-and-items/items-and-item-types.htm) Tile Layer (based on either a Map Service or Vector Tile Service).  Once a custom style has been retrieved from the service it may be cached. If you make changes to the style it could take up to 10 minutes for the changes to be available. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Portal item unique identifier representing a single content item in the ArcGIS portal. The portal item must be of [type](https://developers.arcgis.com/rest/users-groups-and-items/items-and-item-types.htm) Tile Layer (based on either a Map Service or Vector Tile Service). See [ArcGIS Glossary](https://developers.arcgis.com/documentation/glossary/item-id/) for more information about portal item IDs. See [ArcGIS item types](https://developers.arcgis.com/rest/users-groups-and-items/items-and-item-types.htm) for more information about item types.  | [required] |
**token** | Option<**String**> | The authentication token, used to access the Basemap styles service.  The `token` parameter can be either an API Key or short-lived token.  Alternatively, you can supply a token in the request header with one of the following keys using the \"Bearer\" scheme:  - `Authorization: Bearer <YOUR_TOKEN>` - `X-Esri-Authorization: Bearer <YOUR_TOKEN>`  The provided `token` must have the necessary `premium:user:basemaps` privilege to use the basemap style service.  **Developer guide**: To learn more, go to [Security and authentication](https://developers.arcgis.com/documentation/mapping-apis-and-services/security/).  |  |
**f** | Option<**String**> | Optional, case-sensitive parameter to specify the format in which responses are given. Can either be json or pjson. |  |[default to json]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ArcGISXEsriAuthorizationHeader](../README.md#ArcGISXEsriAuthorizationHeader), [ArcGISAuthorizationHeader](../README.md#ArcGISAuthorizationHeader), [ArcGISTokenParameter](../README.md#ArcGISTokenParameter)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

