/*
 * Mistral AI API
 *
 * Our Chat Completion and Embeddings APIs specification. Create your account on [La Plateforme](https://console.mistral.ai) to get access and read the [docs](https://docs.mistral.ai) to learn how to use it.
 *
 * Build date: 2024-06-15T23:59:31.144857-06:00[America/Mexico_City]
 * Generated using: https://openapi-generator.tech
 * Open API specification v0.0.2 provided by Mistral @ https://docs.mistral.ai/redocusaurus/plugin-redoc-0.yaml
 * Custom generation templates by [GovCraft Ai](https://www.govcraft.ai)
 * Contact: roland@govcraft.ai
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    /// ID of the model to use. You can use the [List Available Models](/api#operation/listModels) API to see all of your available models, or see our [Model overview](/models) for model descriptions.
    #[serde(rename = "model")]
    pub model: String,
    /// The prompt(s) to generate completions for, encoded as a list of dict with role and content.
    #[serde(rename = "messages")]
    pub messages: Vec<models::ChatMessages>,
    /// What sampling temperature to use, between 0.0 and 1.0. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.  We generally recommend altering this or `top_p` but not both.
    #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Nucleus sampling, where the model considers the results of the tokens with `top_p` probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.  We generally recommend altering this or `temperature` but not both.
    #[serde(rename = "top_p", skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// The maximum number of tokens to generate in the completion.  The token count of your prompt plus `max_tokens` cannot exceed the model's context length.  
    #[serde(rename = "max_tokens", skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Whether to stream back partial progress. If set, tokens will be sent as data-only server-sent events as they become available, with the stream terminated by a data: [DONE] message. Otherwise, the server will hold the request open until the timeout or until completion, with the response containing the full result as JSON.
    #[serde(rename = "stream", skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Whether to inject a safety prompt before all conversations.
    #[serde(rename = "safe_prompt", skip_serializing_if = "Option::is_none")]
    pub safe_prompt: Option<bool>,
    /// The seed to use for random sampling. If set, different calls will generate deterministic results.
    #[serde(rename = "random_seed", skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<i32>,
}

impl ChatCompletionRequest {
    /// Creates a new `ChatCompletionRequest` instance.
    ///
    /// # Arguments
    /// * `model`
    /// * `messages`
    ///
    /// # Returns
    ///
    /// A new `ChatCompletionRequest` instance.
    pub fn new(model: String, messages: Vec<models::ChatMessages>) -> ChatCompletionRequest {
        ChatCompletionRequest {
            model,
            messages,
            temperature: None,
            top_p: None,
            max_tokens: None,
            stream: None,
            safe_prompt: None,
            random_seed: None,
        }
    }
}
