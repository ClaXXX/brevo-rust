# CreateWhatsAppTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the template | 
**language** | **String** | Language of the template. For Example : **en** for English  | 
**category** | **String** | Category of the template | 
**media_url** | Option<**String**> | Absolute url of the media file **(no local file)** for the header. **Use this field in you want to add media in Template header and headerText is empty**. Allowed extensions for media files are: #### jpeg | png | mp4 | pdf  | [optional]
**body_text** | **String** | Body of the template. **Maximum allowed characters are 1024** | 
**header_text** | Option<**String**> | Text content of the header in the template. **Maximum allowed characters are 45** **Use this field to add text content in template header and if mediaUrl is empty**  | [optional]
**source** | Option<**String**> | source of the template | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


