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

/// JsonModeResponseFormat : An object specifying the format that the model must output. Setting to `{ \"type\": \"json_object\" }` enables JSON mode, which guarantees the message the model generates is in JSON. When using JSON mode you MUST also instruct the model to produce JSON yourself with a system or a user message.

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonModeResponseFormat {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl JsonModeResponseFormat {
    /// Creates a new `JsonModeResponseFormat` instance.
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// A new `JsonModeResponseFormat` instance.
    /// An object specifying the format that the model must output. Setting to `{ \"type\": \"json_object\" }` enables JSON mode, which guarantees the message the model generates is in JSON. When using JSON mode you MUST also instruct the model to produce JSON yourself with a system or a user message.
    pub fn new() -> JsonModeResponseFormat {
        JsonModeResponseFormat { r#type: None }
    }
}
