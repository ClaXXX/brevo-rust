# UpdateSmsCampaign

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the campaign | [optional]
**sender** | Option<**String**> | Name of the sender. **The number of characters is limited to 11 for alphanumeric characters and 15 for numeric characters**  | [optional]
**content** | Option<**String**> | Content of the message. The **maximum characters used per SMS is 160**, if used more than that, it will be counted as more than one SMS  | [optional]
**recipients** | Option<[**models::CreateSmsCampaignRecipients**](createSmsCampaign_recipients.md)> |  | [optional]
**scheduled_at** | Option<**String**> | UTC date-time on which the campaign has to run (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**  | [optional]
**unicode_enabled** | Option<**bool**> | Format of the message. It indicates whether the content should be treated as unicode or not.  | [optional][default to false]
**organisation_prefix** | Option<**String**> | A recognizable prefix will ensure your audience knows who you are. Recommended by U.S. carriers. This will be added as your Brand Name before the message content. **Prefer verifying maximum length of 160 characters including this prefix in message content to avoid multiple sending of same sms.** | [optional]
**unsubscribe_instruction** | Option<**String**> | Instructions to unsubscribe from future communications. Recommended by U.S. carriers. Must include **STOP** keyword. This will be added as instructions after the end of message content. **Prefer verifying maximum length of 160 characters including this instructions in message content to avoid multiple sending of same sms.** | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


