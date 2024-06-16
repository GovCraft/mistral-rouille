# JobMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**training_steps** | Option<**i32**> | The number of training steps to perform. A training step refers to a single update of the model weights during the fine-tuning process. This update is typically calculated using a batch of samples from the training dataset.  | [optional]
**train_tokens_per_step** | Option<**i32**> | The number of tokens consumed by one training step. | [optional]
**data_tokens** | Option<**i32**> | The total number of tokens in the training dataset. | [optional]
**train_tokens** | Option<**i32**> | The total number of tokens used during the fine-tuning process. | [optional]
**epochs** | Option<**f32**> | The number of complete passes through the entire training dataset. | [optional]
**expected_duration_seconds** | Option<**i32**> | The approximated time (in seconds) for the fine-tuning process to complete. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


