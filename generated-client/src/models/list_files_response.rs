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
pub struct ListFilesResponse {
    #[serde(rename = "data")]
    pub data: Vec<models::FileSchema>,
    #[serde(rename = "object")]
    pub object: String,
}

impl ListFilesResponse {
    /// Creates a new `ListFilesResponse` instance.
    ///
    /// # Arguments
    /// * `data`
    /// * `object`
    ///
    /// # Returns
    ///
    /// A new `ListFilesResponse` instance.
    pub fn new(data: Vec<models::FileSchema>, object: String) -> ListFilesResponse {
        ListFilesResponse { data, object }
    }
}
