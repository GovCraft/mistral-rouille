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
pub struct EventOut {
    /// The name of the event.
    #[serde(rename = "name")]
    pub name: String,
    /// The status of the fine-tuning job at the time of the event
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    /// The UNIX timestamp (in seconds) of the event.
    #[serde(rename = "created_at")]
    pub created_at: i32,
}

impl EventOut {
    /// Creates a new `EventOut` instance.
    ///
    /// # Arguments
    /// * `name`
    /// * `created_at`
    ///
    /// # Returns
    ///
    /// A new `EventOut` instance.
    pub fn new(name: String, created_at: i32) -> EventOut {
        EventOut {
            name,
            data: None,
            created_at,
        }
    }
}
/// The status of the fine-tuning job at the time of the event
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Data {
    #[serde(rename = "QUEUED")]
    Queued,
    #[serde(rename = "STARTED")]
    Started,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "CANCELLATION_REQUESTED")]
    CancellationRequested,
}

impl Default for Data {
    fn default() -> Data {
        Self::Queued
    }
}
