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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WandbIntegrationOut {
    #[serde(
        rename = "type",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<Option<serde_json::Value>>,
    /// The name of the project that the new run will be created under.
    #[serde(rename = "project")]
    pub project: String,
    /// A display name to set for the run. If not set, will use the job ID as the name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WandbIntegrationOut {
    /// Creates a new `WandbIntegrationOut` instance.
    ///
    /// # Arguments
    /// * `project`
    ///
    /// # Returns
    ///
    /// A new `WandbIntegrationOut` instance.
    pub fn new(project: String) -> WandbIntegrationOut {
        WandbIntegrationOut {
            r#type: None,
            project,
            name: None,
        }
    }
}
