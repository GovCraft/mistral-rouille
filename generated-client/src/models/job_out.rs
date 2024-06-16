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
pub struct JobOut {
    /// The ID of the job.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "hyperparameters")]
    pub hyperparameters: models::TrainingParameters,
    #[serde(rename = "model")]
    pub model: models::FineTuneableModel,
    /// The current status of the fine-tuning job.
    #[serde(rename = "status")]
    pub status: Status,
    /// The type of job (`FT` for fine-tuning).
    #[serde(rename = "job_type")]
    pub job_type: String,
    /// The UNIX timestamp (in seconds) for when the fine-tuning job was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The UNIX timestamp (in seconds) for when the fine-tuning job was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: i32,
    /// A list containing the IDs of uploaded files that contain training data.
    #[serde(rename = "training_files")]
    pub training_files: Vec<uuid::Uuid>,
    /// A list containing the IDs of uploaded files that contain validation data.
    #[serde(rename = "validation_files", skip_serializing_if = "Option::is_none")]
    pub validation_files: Option<Vec<uuid::Uuid>>,
    #[serde(
        rename = "object",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub object: Option<Option<serde_json::Value>>,
    /// The name of the fine-tuned model that is being created. The value will be `null` if the fine-tuning job is still running.
    #[serde(rename = "fine_tuned_model", skip_serializing_if = "Option::is_none")]
    pub fine_tuned_model: Option<String>,
    /// A list of integrations enabled for your fine-tuning job.
    #[serde(rename = "integrations", skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<models::WandbIntegrationOut>>,
}

impl JobOut {
    /// Creates a new `JobOut` instance.
    ///
    /// # Arguments
    /// * `id`
    /// * `hyperparameters`
    /// * `model`
    /// * `status`
    /// * `job_type`
    /// * `created_at`
    /// * `modified_at`
    /// * `training_files`
    ///
    /// # Returns
    ///
    /// A new `JobOut` instance.
    pub fn new(
        id: uuid::Uuid,
        hyperparameters: models::TrainingParameters,
        model: models::FineTuneableModel,
        status: Status,
        job_type: String,
        created_at: i32,
        modified_at: i32,
        training_files: Vec<uuid::Uuid>,
    ) -> JobOut {
        JobOut {
            id,
            hyperparameters,
            model,
            status,
            job_type,
            created_at,
            modified_at,
            training_files,
            validation_files: None,
            object: None,
            fine_tuned_model: None,
            integrations: None,
        }
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
