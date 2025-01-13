# \ElevationValuesApi

All URIs are relative to *https://elevation-api.arcgis.com/arcgis/rest/services/elevation-service/beta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**elevation_at_many_points_post**](ElevationValuesApi.md#elevation_at_many_points_post) | **POST** /elevation/at-many-points | Returns elevations for up to 100 input coordinates.
[**elevation_at_point_get**](ElevationValuesApi.md#elevation_at_point_get) | **GET** /elevation/at-point | Returns the elevation from a given input coordinate.



## elevation_at_many_points_post

> models::ElevationAtManyPointsPost200Response elevation_at_many_points_post(elevation_at_many_points_post_request)
Returns elevations for up to 100 input coordinates.

Returns elevations in meters at given longitudes and latitudes within the WGS84 coordinate system.  The order of the points returned by this request will be the same as the order of the points passed in the `coordinates` parameter.  If the distance between the furthest West and furthest East coordinate exceeds 50km, the service will return a `400` HTTP response as the distance between these points is too large.  If the distance between the furthest North and furthest South coordinate exceeds 50km, the service will return a `400` HTTP response as the distance between these points is too large.  If any of the points are otherwise invalid, a `400` HTTP response will be returned.  By default the elevation is measured with respect to the Earth's mean sea level. It takes into account the local variations in gravity and provides a consistent vertical reference.  If the `relativeTo` parameter in the body is set to `ellipsoid`, the elevation will be measured with respect to the ellipsoid. This is a mathematical model that approximates the shape of the Earth. It does not consider local variations in gravity and is commonly used in GPS positioning.  Note: You cannot permanently store elevations. Please see the [Terms of use](https://developers.arcgis.com/documentation/mapping-apis-and-services/deployment/terms-of-use/).  The Post Body content type must be either: - JSON with content type of `application/json`, or - form URL encoded key-value pairs with content type `application/x-www-form-urlencoded`.  The following parameters are used to fetch elevations for multiple coordinates:  **coordinates**   - (Required) Array of (longitude, latitude) pairs in the WGS84 spatial reference. Maximum size of 100 coordinates. The order of each pair must be     - longitude in the range `-179.99` to `179.99` representing the east/west or x-axis     - latitude in the range `-85.05` to `85.05` representing the north/south or y-axis   - For example: `[[31.134167, 29.979167], [31.130833, 29.976111], [31.128333, 29.9725]]`  **f**   - (Optional) Case-sensitive parameter to specify the format in which responses are given. Can either be `json` or `pjson`.  **relativeTo**   - (Optional) The reference position (datum) from which to measure elevation. The valid values are:      - **meanSeaLevel**: The elevation above or below the WGS84 geoid reference surface, which is approximately the mean sea level. It takes into account the local variations in gravity and provides a consistent vertical reference.     - **ellipsoid**: Ellipsoidal height is measured with respect to an ellipsoid, which is a mathematical model that approximates the shape of the Earth. It does not consider local variations in gravity and is commonly used in GPS positioning.  **token**   - (Optional) The authentication token, used to access the elevation service. Alternatively, you can supply a token in the request header with either the `Authorization` or `X-Esri-Authorization` key, using the \"Bearer\" scheme. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**elevation_at_many_points_post_request** | [**ElevationAtManyPointsPostRequest**](ElevationAtManyPointsPostRequest.md) |  | [required] |

### Return type

[**models::ElevationAtManyPointsPost200Response**](ElevationAtManyPointsPost_200_response.md)

### Authorization

[ArcGISXEsriAuthorizationHeader](../README.md#ArcGISXEsriAuthorizationHeader), [ArcGISAuthorizationHeader](../README.md#ArcGISAuthorizationHeader)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## elevation_at_point_get

> models::ElevationAtPointGet200Response elevation_at_point_get(lon, lat, relative_to, token, f)
Returns the elevation from a given input coordinate.

Returns the elevation in meters at a given longitude and latitude within the WGS84 coordinate system.  By default the elevation is measured with respect to the Earth's mean sea level. It takes into account the local variations in gravity and provides a consistent vertical reference.  If the `relativeTo` query parameter is set to `ellipsoid`, the elevation will be measured with respect to the ellipsoid. This is a mathematical model that approximates the shape of the Earth. It does not consider local variations in gravity and is commonly used in GPS positioning.  Note: You cannot permanently store elevations. Please see the [Terms of use](https://developers.arcgis.com/documentation/mapping-apis-and-services/deployment/terms-of-use/). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lon** | **f64** | The longitude of the specified point. | [required] |
**lat** | **f64** | The latitude of the specified point. | [required] |
**relative_to** | Option<**String**> | The reference position (datum) from which to measure elevation. The valid values are: - meanSeaLevel: The elevation above or below the WGS84 geoid reference surface, which is approximately the mean sea level.                 It takes into account the local variations in gravity and provides a consistent vertical reference. - ellipsoid: Ellipsoidal height is measured with respect to an ellipsoid, which is a mathematical model that approximates the shape of the Earth.               It does not consider local variations in gravity and is commonly used in GPS positioning.  |  |[default to meanSeaLevel]
**token** | Option<**String**> | The authentication token, used to access the elevation service.  The `token` parameter can be either an API Key or short-lived token.  Alternatively, you can supply a token in the request header with one of the following keys using the \"Bearer\" scheme:  - `Authorization: Bearer <YOUR_TOKEN>` - `X-Esri-Authorization: Bearer <YOUR_TOKEN>`  The provided `token` must be created from an ArcGIS Location Platform account and have the necessary `premium:user:elevation` privilege to use the elevation service.  **Developer guide**: To learn more, go to [Security and authentication](https://developers.arcgis.com/documentation/mapping-apis-and-services/security/).  |  |
**f** | Option<[**Format**](.md)> | Optional, case-sensitive parameter to specify the format in which responses are given. Can either be `json` or `pjson`. |  |

### Return type

[**models::ElevationAtPointGet200Response**](ElevationAtPointGet_200_response.md)

### Authorization

[ArcGISXEsriAuthorizationHeader](../README.md#ArcGISXEsriAuthorizationHeader), [ArcGISAuthorizationHeader](../README.md#ArcGISAuthorizationHeader), [ArcGISTokenParameter](../README.md#ArcGISTokenParameter)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

