# ChatCompletionToolsFn

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the function to help the model determine when and how to invoke it.  | [optional]
**name** | **String** | The name of the function to be called. Must be a-z,A-Z,0-9 or contain underscores and dashes, with a maximum length of 64.  | 
**parameters** | Option<[**serde_json::Value**](.md)> | The function parameters, defined using a JSON Schema object. If omitted, the function is considered to have an empty parameter list.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


