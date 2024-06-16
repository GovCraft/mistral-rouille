# TrainingParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**training_steps** | **u32** | The number of training steps to perform. A training step refers to a single update of the model weights during the fine-tuning process. This update is typically calculated using a batch of samples from the training dataset.  | 
**learning_rate** | Option<**f64**> | A parameter describing how much to adjust the pre-trained model's weights in response to the estimated error each time the weights are updated during the fine-tuning process.  | [optional][default to 1.0E-4]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


