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
pub struct ChatFnTools {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

impl ChatFnTools {
    /// Creates a new `ChatFnTools` instance.
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// A new `ChatFnTools` instance.
    pub fn new() -> ChatFnTools {
        ChatFnTools {
            name: None,
            arguments: None,
        }
    }
}
