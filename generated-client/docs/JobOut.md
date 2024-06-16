# JobOut

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The ID of the job. | 
**hyperparameters** | [**models::TrainingParameters**](TrainingParameters.md) |  | 
**model** | [**models::FineTuneableModel**](FineTuneableModel.md) |  | 
**status** | **String** | The current status of the fine-tuning job. | 
**job_type** | **String** | The type of job (`FT` for fine-tuning). | 
**created_at** | **i32** | The UNIX timestamp (in seconds) for when the fine-tuning job was created. | 
**modified_at** | **i32** | The UNIX timestamp (in seconds) for when the fine-tuning job was last modified. | 
**training_files** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | A list containing the IDs of uploaded files that contain training data. | 
**validation_files** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | A list containing the IDs of uploaded files that contain validation data. | [optional][default to []]
**object** | Option<[**serde_json::Value**](.md)> |  | [optional]
**fine_tuned_model** | Option<**String**> | The name of the fine-tuned model that is being created. The value will be `null` if the fine-tuning job is still running. | [optional]
**integrations** | Option<[**Vec<models::WandbIntegrationOut>**](WandbIntegrationOut.md)> | A list of integrations enabled for your fine-tuning job. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


