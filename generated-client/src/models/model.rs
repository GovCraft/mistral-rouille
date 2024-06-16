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

/// Model : Model object.

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "created")]
    pub created: i32,
    #[serde(rename = "owned_by")]
    pub owned_by: String,
}

impl Model {
    /// Creates a new `Model` instance.
    ///
    /// # Arguments
    /// * `id`
    /// * `object`
    /// * `created`
    /// * `owned_by`
    ///
    /// # Returns
    ///
    /// A new `Model` instance.
    /// Model object.
    pub fn new(id: String, object: String, created: i32, owned_by: String) -> Model {
        Model {
            id,
            object,
            created,
            owned_by,
        }
    }
}
