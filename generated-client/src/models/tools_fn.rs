/*
 * Mistral AI API
 *
 * Our Chat Completion and Embeddings APIs specification. Create your account on [La Plateforme](https://console.mistral.ai) to get access and read the [docs](https://docs.mistral.ai) to learn how to use it.
 *
 * Build date: 2024-06-15T23:41:00.377209-06:00[America/Mexico_City]
 * Generated using: https://openapi-generator.tech
 * Open API specification v0.0.2 provided by Mistral @ https://docs.mistral.ai/redocusaurus/plugin-redoc-0.yaml
 * Custom generation templates by [GovCraft Ai](https://www.govcraft.ai)
 * Contact: roland@govcraft.ai
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ToolsFn : The function properties.

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolsFn {
    /// The description of the function to help the model determine when and how to invoke it.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the function to be called. Must be a-z,A-Z,0-9 or contain underscores and dashes, with a maximum length of 64.
    #[serde(rename = "name")]
    pub name: String,
    /// The function parameters, defined using a JSON Schema object. If omitted, the function is considered to have an empty parameter list.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

impl ToolsFn {
    /// Creates a new `ToolsFn` instance.
    ///
    /// # Arguments
    /// * `name`
    ///
    /// # Returns
    ///
    /// A new `ToolsFn` instance.
    /// The function properties.
    pub fn new(name: String) -> ToolsFn {
        ToolsFn {
            description: None,
            name,
            parameters: None,
        }
    }
}
