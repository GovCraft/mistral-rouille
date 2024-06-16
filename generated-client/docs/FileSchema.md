# FileSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The file identifier, which can be referenced in the API endpoints | 
**object** | **String** | The object type, which is always `file`. | 
**bytes** | **i32** | The size of the file, in bytes. | 
**created_at** | **i32** | The UNIX timestamp (in seconds) for when the file was created. | 
**filename** | **String** | The name of the file | 
**purpose** | **String** | The intended purpose of the file. Only supports `fine-tune` for now. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


