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
pub struct JobIn {
    #[serde(rename = "model")]
    pub model: models::FineTuneableModel,
    /// A list containing the IDs of uploaded files that contain training data.
    #[serde(rename = "training_files")]
    pub training_files: Vec<uuid::Uuid>,
    /// A list containing the IDs of uploaded files that contain validation data.  If you provide these files, the data is used to generate validation metrics periodically during fine-tuning. These metrics can be viewed in `checkpoints` when getting the status of a running fine-tuning job.  The same data should not be present in both train and validation files.
    #[serde(rename = "validation_files", skip_serializing_if = "Option::is_none")]
    pub validation_files: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "hyperparameters")]
    pub hyperparameters: models::TrainingParameters,
    /// A string that will be added to your fine-tuning model name. For example, a suffix of \"my-great-model\" would produce a model name like `ft:open-mistral-7b:my-great-model:xxx...`
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// A list of integrations to enable for your fine-tuning job.
    #[serde(rename = "integrations", skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<models::WandbIntegration>>,
}

impl JobIn {
    /// Creates a new `JobIn` instance.
    ///
    /// # Arguments
    /// * `model`
    /// * `training_files`
    /// * `hyperparameters`
    ///
    /// # Returns
    ///
    /// A new `JobIn` instance.
    pub fn new(
        model: models::FineTuneableModel,
        training_files: Vec<uuid::Uuid>,
        hyperparameters: models::TrainingParameters,
    ) -> JobIn {
        JobIn {
            model,
            training_files,
            validation_files: None,
            hyperparameters,
            suffix: None,
            integrations: None,
        }
    }
}
