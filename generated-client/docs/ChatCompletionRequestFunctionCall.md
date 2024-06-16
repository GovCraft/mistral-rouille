# ChatCompletionRequestFunctionCall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | **String** | ID of the model to use. You can use the [List Available Models](/api#operation/listModels) API to see all of your available models, or see our [Model overview](/models) for model descriptions.  | 
**messages** | [**Vec<models::ChatFnMessages>**](ChatFnMessages.md) | The prompt(s) to generate completions for, encoded as a list of dict with role and content. The first prompt role should be `user` or `system`. When role is `tool`, the properties should contain `tool_call_id` (string or `null`).  | 
**temperature** | Option<**f64**> | What sampling temperature to use, between 0.0 and 1.0. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.  We generally recommend altering this or `top_p` but not both.  | [optional][default to 0.7]
**top_p** | Option<**f64**> | Nucleus sampling, where the model considers the results of the tokens with `top_p` probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.  We generally recommend altering this or `temperature` but not both.  | [optional][default to 1]
**max_tokens** | Option<**u32**> | The maximum number of tokens to generate in the completion.  The token count of your prompt plus `max_tokens` cannot exceed the model's context length.   | [optional]
**stream** | Option<**bool**> | Whether to stream back partial progress. If set, tokens will be sent as data-only server-sent events as they become available, with the stream terminated by a data: [DONE] message. Otherwise, the server will hold the request open until the timeout or until completion, with the response containing the full result as JSON.  | [optional][default to false]
**safe_prompt** | Option<**bool**> | Whether to inject a safety prompt before all conversations.  | [optional][default to false]
**tools** | Option<[**Vec<models::ChatCompletionRequestFunctionCallToolsInner>**](ChatCompletionRequestFunctionCall_tools_inner.md)> | A list of available tools for the model. Use this to specify functions for which the model can generate JSON inputs.  | [optional]
**tool_choice** | Option<**String**> | Specifies if/how functions are called. If set to `none` the model won't call a function and will generate a message instead. If set to `auto` the model can choose to either generate a message or call a function. If set to `any` the model is forced to call a function.  | [optional][default to auto]
**random_seed** | Option<**i32**> | The seed to use for random sampling. If set, different calls will generate deterministic results.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


