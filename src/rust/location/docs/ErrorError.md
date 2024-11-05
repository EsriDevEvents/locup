# ErrorError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **i32** | A code identifying the type of error, either an HTTP error code, `498` (signifying invalid or expired token), or `499` (signifying missing token). | 
**message** | **String** | A message describing the error. | 
**details** | Option<**Vec<String>**> | List of details about the error. | [optional]
**rest_info_url** | Option<**String**> | URL that provides the basemap-styles service information. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


