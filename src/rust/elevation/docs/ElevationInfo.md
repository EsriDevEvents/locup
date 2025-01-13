# ElevationInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**relative_to** | **String** | The reference position (datum) from which to measure elevation. The valid values are: - meanSeaLevel: The elevation above or below the WGS84 geoid reference surface, which is approximately the mean sea level.                 It takes into account the local variations in gravity and provides a consistent vertical reference. - ellipsoid: Ellipsoidal height is measured with respect to an ellipsoid, which is a mathematical model that approximates the shape of the Earth.               It does not consider local variations in gravity and is commonly used in GPS positioning.  | [default to MeanSeaLevel]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


