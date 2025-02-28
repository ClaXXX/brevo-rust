# BalanceOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **f64** | Order amount (must not be zero). | 
**balance_definition_id** | Option<**String**> | Optional unique identifier (UUID) of the associated balance definition. | [optional]
**contact_id** | **i32** | Unique identifier of the contact placing the order (must be ≥ 1). | 
**created_at** | **String** | RFC3339 timestamp indicating when the order was created. | 
**due_at** | **String** | RFC3339 timestamp specifying when the order is due in the future. | 
**expires_at** | Option<**String**> | Optional RFC3339 timestamp defining order expiration in the future. | [optional]
**id** | Option<**String**> | Unique identifier for the balance order. | [optional]
**loyalty_program_id** | **String** | Unique identifier of the loyalty program associated with the order. | 
**meta** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Optional metadata associated with the order. | [optional]
**processed_at** | Option<**String**> | Optional RFC3339 timestamp indicating when the order was processed. | [optional]
**transactionid** | Option<**String**> | Optional reference to the associated transaction ID. | [optional]
**updated_at** | **String** | RFC3339 timestamp indicating the last update to the order. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


