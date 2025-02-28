# CreateCouponCollectionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the coupons collection | 
**default_coupon** | **String** | Default coupons collection name | 
**expiration_date** | Option<**String**> | Specify an expiration date for the coupon collection in RFC3339 format. Use null to remove the expiration date. | [optional]
**remaining_days_alert** | Option<**i32**> | Send a notification alert (email) when the remaining days until the expiration date are equal or fall bellow this number. Use null to disable alerts. | [optional]
**remaining_coupons_alert** | Option<**i32**> | Send a notification alert (email) when the remaining coupons count is equal or fall bellow this number. Use null to disable alerts. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


