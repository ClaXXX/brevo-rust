# GetExtendedList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | ID of the list | 
**name** | **String** | Name of the list | 
**total_blacklisted** | **i64** | Number of blacklisted contacts in the list | 
**total_subscribers** | **i64** | Number of contacts in the list | 
**unique_subscribers** | **i64** | Number of unique contacts in the list | 
**folder_id** | **i64** | ID of the folder | 
**created_at** | **String** | Creation UTC date-time of the list (YYYY-MM-DDTHH:mm:ss.SSSZ) | 
**campaign_stats** | Option<[**Vec<models::GetExtendedListAllOfCampaignStats>**](getExtendedList_allOf_campaignStats.md)> |  | [optional]
**dynamic_list** | Option<**bool**> | Status telling if the list is dynamic or not (true=dynamic, false=not dynamic) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


