# JobIn

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | [**models::FineTuneableModel**](FineTuneableModel.md) |  | 
**training_files** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | A list containing the IDs of uploaded files that contain training data. | 
**validation_files** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | A list containing the IDs of uploaded files that contain validation data.  If you provide these files, the data is used to generate validation metrics periodically during fine-tuning. These metrics can be viewed in `checkpoints` when getting the status of a running fine-tuning job.  The same data should not be present in both train and validation files.  | [optional]
**hyperparameters** | [**models::TrainingParameters**](TrainingParameters.md) |  | 
**suffix** | Option<**String**> | A string that will be added to your fine-tuning model name. For example, a suffix of \"my-great-model\" would produce a model name like `ft:open-mistral-7b:my-great-model:xxx...`  | [optional]
**integrations** | Option<[**Vec<models::WandbIntegration>**](WandbIntegration.md)> | A list of integrations to enable for your fine-tuning job. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


