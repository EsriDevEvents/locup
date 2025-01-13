# \SelfDescriptionApi

All URIs are relative to *https://basemapstyles-api.arcgis.com/arcgis/rest/services/styles/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**service_self_get**](SelfDescriptionApi.md#service_self_get) | **GET** /self | Describes the Basemap Styles service.
[**styles_self_get_0**](SelfDescriptionApi.md#styles_self_get_0) | **GET** /styles/self | Describes service styles.
[**webmaps_self_get_0**](SelfDescriptionApi.md#webmaps_self_get_0) | **GET** /webmaps/self | Describes service webmaps.



## service_self_get

> models::ServiceSelf service_self_get()
Describes the Basemap Styles service.

Returns JSON describing the Basemap Styles Service. Includes a list of all supported languages, worldviews, and places.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServiceSelf**](ServiceSelf.md)

### Authorization

[ArcGISXEsriAuthorizationHeader](../README.md#ArcGISXEsriAuthorizationHeader), [ArcGISAuthorizationHeader](../README.md#ArcGISAuthorizationHeader), [ArcGISTokenParameter](../README.md#ArcGISTokenParameter)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## styles_self_get_0

> models::StylesSelf styles_self_get_0()
Describes service styles.

Returns JSON describing the styles endpoint including supported styles. Includes a list of all supported languages, worldviews, and places. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::StylesSelf**](StylesSelf.md)

### Authorization

[ArcGISXEsriAuthorizationHeader](../README.md#ArcGISXEsriAuthorizationHeader), [ArcGISAuthorizationHeader](../README.md#ArcGISAuthorizationHeader), [ArcGISTokenParameter](../README.md#ArcGISTokenParameter)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webmaps_self_get_0

> models::WebmapsSelf webmaps_self_get_0()
Describes service webmaps.

Returns JSON describing the webmaps endpoint including supported styles. Includes a list of all supported languages, worldviews, and places. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebmapsSelf**](WebmapsSelf.md)

### Authorization

[ArcGISXEsriAuthorizationHeader](../README.md#ArcGISXEsriAuthorizationHeader), [ArcGISAuthorizationHeader](../README.md#ArcGISAuthorizationHeader), [ArcGISTokenParameter](../README.md#ArcGISTokenParameter)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

