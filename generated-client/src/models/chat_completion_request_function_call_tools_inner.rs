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
pub struct ChatCompletionRequestFunctionCallToolsInner {
    /// The type of the tool. Currently, only `function` is supported.
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "function")]
    pub function: models::ToolsFn,
}

impl ChatCompletionRequestFunctionCallToolsInner {
    /// Creates a new `ChatCompletionRequestFunctionCallToolsInner` instance.
    ///
    /// # Arguments
    /// * `r#type`
    /// * `function`
    ///
    /// # Returns
    ///
    /// A new `ChatCompletionRequestFunctionCallToolsInner` instance.
    pub fn new(
        r#type: String,
        function: models::ToolsFn,
    ) -> ChatCompletionRequestFunctionCallToolsInner {
        ChatCompletionRequestFunctionCallToolsInner { r#type, function }
    }
}