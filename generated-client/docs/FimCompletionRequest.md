# FimCompletionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prompt** | **String** | The text/code to complete. | 
**suffix** | Option<**String**> | Optional text/code that adds more context for the model. When given a `prompt` and a `suffix` the model will fill what is between them. When `suffix` is not provided, the model will simply execute completion starting with `prompt`.  | [optional]
**model** | **String** | ID of the model to use. Only compatible for now with:   - `codestral-2405`   - `codestral-latest`   | 
**temperature** | Option<**f64**> | What sampling temperature to use, between 0.0 and 1.0.  Higher values like 0.8 will make the outptu more random, while lower values like 0.2 will make it more focused and deterministic.  We generally recommend altering this or `top_p` but not both.  | [optional][default to 0.7]
**top_p** | Option<**f64**> | Nucleus sampling, where the model considers the results of the tokens with with `top_p` probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.  We generally recommend altering this or `temperature` but not both.  | [optional][default to 1]
**max_tokens** | Option<**u32**> | The maximum number of tokens to generate in the completion.  The token count of your prompt plus `max_tokens` cannot exceed the model's context length.  | [optional]
**min_tokens** | Option<**u32**> | The minimum number of tokens to generate in the completion.  | [optional]
**stream** | Option<**bool**> | Whether to stream back partial progress. If set, tokens will be sent as data-only server-side events as they become available, with the stream terminated by a data: [DONE] message.\"  Otherwise, the server will hold the request open until the timeout or until completion, with the response containing the full result as JSON.  | [optional][default to false]
**random_seed** | Option<**u32**> | The seed to use for random sampling. If set, different calls will generate deterministic results.  | [optional]
**stop** | Option<[**models::FimStop**](FimStop.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


