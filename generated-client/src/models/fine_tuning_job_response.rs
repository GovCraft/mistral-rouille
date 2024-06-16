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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuningJobResponse {
    JobOut(models::JobOut),
    JobMetadata(models::JobMetadata),
}

impl Default for FineTuningJobResponse {
    fn default() -> Self {
        Self::JobOut(Default::default())
    }
}
/// The current status of the fine-tuning job.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
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

impl Default for Status {
    fn default() -> Status {
        Self::Queued
    }
}
