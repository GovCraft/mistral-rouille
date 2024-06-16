#![allow(unused)]
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

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`create_chat_completion`]
#[derive(Clone, Debug)]
pub struct CreateChatCompletionParams {
    pub chat_request: models::ChatRequest,
}

/// struct for passing parameters to the method [`create_embedding`]
#[derive(Clone, Debug)]
pub struct CreateEmbeddingParams {
    pub embedding_request: models::EmbeddingRequest,
}

/// struct for passing parameters to the method [`create_fim_completion`]
#[derive(Clone, Debug)]
pub struct CreateFimCompletionParams {
    pub fim_completion_request: models::FimCompletionRequest,
}

/// struct for passing parameters to the method [`files_api_routes_delete_file`]
#[derive(Clone, Debug)]
pub struct FilesApiRoutesDeleteFileParams {
    pub file_id: String,
}

/// struct for passing parameters to the method [`files_api_routes_retrieve_file`]
#[derive(Clone, Debug)]
pub struct FilesApiRoutesRetrieveFileParams {
    pub file_id: String,
}

/// struct for passing parameters to the method [`files_api_routes_upload_file`]
#[derive(Clone, Debug)]
pub struct FilesApiRoutesUploadFileParams {
    pub purpose: Option<models::serde_json::Value>,
    /// The File object (not file name) to be uploaded.   To upload a file and specify a custom file name you should format your request as such:   ```   file=@path/to/your/file.jsonl;filename=custom_name.jsonl   ```  Otherwise, you can just keep the original file name:   ```   file=@path/to/your/file.jsonl   ```
    pub file: std::path::PathBuf,
}

/// struct for passing parameters to the method [`jobs_api_routes_fine_tuning_cancel_fine_tuning_job`]
#[derive(Clone, Debug)]
pub struct JobsApiRoutesFineTuningCancelFineTuningJobParams {
    pub job_id: String,
}

/// struct for passing parameters to the method [`jobs_api_routes_fine_tuning_create_fine_tuning_job`]
#[derive(Clone, Debug)]
pub struct JobsApiRoutesFineTuningCreateFineTuningJobParams {
    pub job_in: models::JobIn,
    pub dry_run: Option<bool>,
}

/// struct for passing parameters to the method [`jobs_api_routes_fine_tuning_get_fine_tuning_job`]
#[derive(Clone, Debug)]
pub struct JobsApiRoutesFineTuningGetFineTuningJobParams {
    pub job_id: String,
}

/// struct for passing parameters to the method [`jobs_api_routes_fine_tuning_get_fine_tuning_jobs`]
#[derive(Clone, Debug)]
pub struct JobsApiRoutesFineTuningGetFineTuningJobsParams {
    /// The page number of the results to be returned.
    pub page: Option<i32>,
    /// The number of items to return per page.
    pub page_size: Option<i32>,
    /// The model name used for fine-tuning to filter on. When set, the other results are not displayed.
    pub model: Option<String>,
    /// The current job state to filter on. When set, the other results are not displayed.
    pub status: Option<String>,
    pub created_after: Option<String>,
    pub created_by_me: Option<bool>,
    pub wandb_project: Option<String>,
    pub wandb_name: Option<String>,
    pub suffix: Option<String>,
}

/// struct for typed successes of method [`create_chat_completion`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChatCompletionSuccess {
    Status200(models::ChatResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_embedding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEmbeddingSuccess {
    Status200(models::EmbeddingResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_fim_completion`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFimCompletionSuccess {
    Status200(models::FimCompletionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`files_api_routes_delete_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesApiRoutesDeleteFileSuccess {
    Status200(models::DeleteFileOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`files_api_routes_list_files`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesApiRoutesListFilesSuccess {
    Status200(models::ListFilesOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`files_api_routes_retrieve_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesApiRoutesRetrieveFileSuccess {
    Status200(models::RetrieveFileOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`files_api_routes_upload_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesApiRoutesUploadFileSuccess {
    Status200(models::UploadFileOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`jobs_api_routes_fine_tuning_cancel_fine_tuning_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JobsApiRoutesFineTuningCancelFineTuningJobSuccess {
    Status200(models::DetailedJobOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`jobs_api_routes_fine_tuning_create_fine_tuning_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JobsApiRoutesFineTuningCreateFineTuningJobSuccess {
    Status200(models::FineTuningJobResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`jobs_api_routes_fine_tuning_get_fine_tuning_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JobsApiRoutesFineTuningGetFineTuningJobSuccess {
    Status200(models::DetailedJobOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`jobs_api_routes_fine_tuning_get_fine_tuning_jobs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JobsApiRoutesFineTuningGetFineTuningJobsSuccess {
    Status200(models::JobsOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_models`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListModelsSuccess {
    Status200(models::ModelList),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_chat_completion`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChatCompletionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_embedding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEmbeddingError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_fim_completion`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFimCompletionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`files_api_routes_delete_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesApiRoutesDeleteFileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`files_api_routes_list_files`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesApiRoutesListFilesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`files_api_routes_retrieve_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesApiRoutesRetrieveFileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`files_api_routes_upload_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesApiRoutesUploadFileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`jobs_api_routes_fine_tuning_cancel_fine_tuning_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JobsApiRoutesFineTuningCancelFineTuningJobError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`jobs_api_routes_fine_tuning_create_fine_tuning_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JobsApiRoutesFineTuningCreateFineTuningJobError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`jobs_api_routes_fine_tuning_get_fine_tuning_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JobsApiRoutesFineTuningGetFineTuningJobError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`jobs_api_routes_fine_tuning_get_fine_tuning_jobs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JobsApiRoutesFineTuningGetFineTuningJobsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_models`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListModelsError {
    UnknownValue(serde_json::Value),
}

pub async fn create_chat_completion(
    configuration: &configuration::Configuration,
    params: CreateChatCompletionParams,
) -> Result<ResponseContent<CreateChatCompletionSuccess>, Error<CreateChatCompletionError>> {
    let configuration = configuration;

    // unbox the parameters
    let chat_request = params.chat_request;

    let client = &configuration.client;

    let uri_str = format!("{}/chat/completions", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::POST, uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&chat_request);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        let entity: Option<CreateChatCompletionSuccess> = serde_json::from_str(&content).ok();
        let result = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Ok(result)
    } else {
        let entity: Option<CreateChatCompletionError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn create_embedding(
    configuration: &configuration::Configuration,
    params: CreateEmbeddingParams,
) -> Result<ResponseContent<CreateEmbeddingSuccess>, Error<CreateEmbeddingError>> {
    let configuration = configuration;

    // unbox the parameters
    let embedding_request = params.embedding_request;

    let client = &configuration.client;

    let uri_str = format!("{}/embeddings", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::POST, uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&embedding_request);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        let entity: Option<CreateEmbeddingSuccess> = serde_json::from_str(&content).ok();
        let result = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Ok(result)
    } else {
        let entity: Option<CreateEmbeddingError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn create_fim_completion(
    configuration: &configuration::Configuration,
    params: CreateFimCompletionParams,
) -> Result<ResponseContent<CreateFimCompletionSuccess>, Error<CreateFimCompletionError>> {
    let configuration = configuration;

    // unbox the parameters
    let fim_completion_request = params.fim_completion_request;

    let client = &configuration.client;

    let uri_str = format!("{}/fim/completions", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::POST, uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&fim_completion_request);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        let entity: Option<CreateFimCompletionSuccess> = serde_json::from_str(&content).ok();
        let result = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Ok(result)
    } else {
        let entity: Option<CreateFimCompletionError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

/// Delete a file.
pub async fn files_api_routes_delete_file(
    configuration: &configuration::Configuration,
    params: FilesApiRoutesDeleteFileParams,
) -> Result<ResponseContent<FilesApiRoutesDeleteFileSuccess>, Error<FilesApiRoutesDeleteFileError>>
{
    let configuration = configuration;

    // unbox the parameters
    let file_id = params.file_id;

    let client = &configuration.client;

    let uri_str = format!(
        "{}/files/{file_id}",
        configuration.base_path,
        file_id = crate::apis::urlencode(file_id)
    );
    let mut req_builder = client.request(reqwest::Method::DELETE, uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        let entity: Option<FilesApiRoutesDeleteFileSuccess> = serde_json::from_str(&content).ok();
        let result = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Ok(result)
    } else {
        let entity: Option<FilesApiRoutesDeleteFileError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

/// Returns a list of files that belong to the user's organization.
pub async fn files_api_routes_list_files(
    configuration: &configuration::Configuration,
) -> Result<ResponseContent<FilesApiRoutesListFilesSuccess>, Error<FilesApiRoutesListFilesError>> {
    let configuration = configuration;

    // unbox the parameters

    let client = &configuration.client;

    let uri_str = format!("{}/files", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        let entity: Option<FilesApiRoutesListFilesSuccess> = serde_json::from_str(&content).ok();
        let result = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Ok(result)
    } else {
        let entity: Option<FilesApiRoutesListFilesError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

/// Returns information about a specific file.
pub async fn files_api_routes_retrieve_file(
    configuration: &configuration::Configuration,
    params: FilesApiRoutesRetrieveFileParams,
) -> Result<
    ResponseContent<FilesApiRoutesRetrieveFileSuccess>,
    Error<FilesApiRoutesRetrieveFileError>,
> {
    let configuration = configuration;

    // unbox the parameters
    let file_id = params.file_id;

    let client = &configuration.client;

    let uri_str = format!(
        "{}/files/{file_id}",
        configuration.base_path,
        file_id = crate::apis::urlencode(file_id)
    );
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        let entity: Option<FilesApiRoutesRetrieveFileSuccess> = serde_json::from_str(&content).ok();
        let result = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Ok(result)
    } else {
        let entity: Option<FilesApiRoutesRetrieveFileError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

/// Upload a file that can be used across various endpoints.  The size of individual files can be a maximum of 512 MB. The Fine-tuning API only supports .jsonl files.  Please contact us if you need to increase these storage limits.
pub async fn files_api_routes_upload_file(
    configuration: &configuration::Configuration,
    params: FilesApiRoutesUploadFileParams,
) -> Result<ResponseContent<FilesApiRoutesUploadFileSuccess>, Error<FilesApiRoutesUploadFileError>>
{
    let configuration = configuration;

    // unbox the parameters
    let purpose = params.purpose;
    let file = params.file;

    let client = &configuration.client;

    let uri_str = format!("{}/files", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::POST, uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form = reqwest::multipart::Form::new();
    match purpose {
        Some(param_value) => {
            form = form.text("purpose", param_value.to_string());
        }
        None => {
            form = form.text("purpose", "");
        }
    }
    // TODO: support file upload for 'file' parameter
    req_builder = req_builder.multipart(form);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        let entity: Option<FilesApiRoutesUploadFileSuccess> = serde_json::from_str(&content).ok();
        let result = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Ok(result)
    } else {
        let entity: Option<FilesApiRoutesUploadFileError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

/// Request the cancellation of a fine tuning job.
pub async fn jobs_api_routes_fine_tuning_cancel_fine_tuning_job(
    configuration: &configuration::Configuration,
    params: JobsApiRoutesFineTuningCancelFineTuningJobParams,
) -> Result<
    ResponseContent<JobsApiRoutesFineTuningCancelFineTuningJobSuccess>,
    Error<JobsApiRoutesFineTuningCancelFineTuningJobError>,
> {
    let configuration = configuration;

    // unbox the parameters
    let job_id = params.job_id;

    let client = &configuration.client;

    let uri_str = format!(
        "{}/fine_tuning/jobs/{job_id}/cancel",
        configuration.base_path,
        job_id = crate::apis::urlencode(job_id)
    );
    let mut req_builder = client.request(reqwest::Method::POST, uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        let entity: Option<JobsApiRoutesFineTuningCancelFineTuningJobSuccess> =
            serde_json::from_str(&content).ok();
        let result = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Ok(result)
    } else {
        let entity: Option<JobsApiRoutesFineTuningCancelFineTuningJobError> =
            serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

/// Create a new fine tuning job, it will be queued for processing.
pub async fn jobs_api_routes_fine_tuning_create_fine_tuning_job(
    configuration: &configuration::Configuration,
    params: JobsApiRoutesFineTuningCreateFineTuningJobParams,
) -> Result<
    ResponseContent<JobsApiRoutesFineTuningCreateFineTuningJobSuccess>,
    Error<JobsApiRoutesFineTuningCreateFineTuningJobError>,
> {
    let configuration = configuration;

    // unbox the parameters
    let job_in = params.job_in;
    let dry_run = params.dry_run;

    let client = &configuration.client;

    let uri_str = format!("{}/fine_tuning/jobs", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::POST, uri_str.as_str());

    if let Some(ref str) = dry_run {
        req_builder = req_builder.query(&[("dry_run", &str.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&job_in);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        let entity: Option<JobsApiRoutesFineTuningCreateFineTuningJobSuccess> =
            serde_json::from_str(&content).ok();
        let result = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Ok(result)
    } else {
        let entity: Option<JobsApiRoutesFineTuningCreateFineTuningJobError> =
            serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

/// Get a fine tuned job details by its UUID.
pub async fn jobs_api_routes_fine_tuning_get_fine_tuning_job(
    configuration: &configuration::Configuration,
    params: JobsApiRoutesFineTuningGetFineTuningJobParams,
) -> Result<
    ResponseContent<JobsApiRoutesFineTuningGetFineTuningJobSuccess>,
    Error<JobsApiRoutesFineTuningGetFineTuningJobError>,
> {
    let configuration = configuration;

    // unbox the parameters
    let job_id = params.job_id;

    let client = &configuration.client;

    let uri_str = format!(
        "{}/fine_tuning/jobs/{job_id}",
        configuration.base_path,
        job_id = crate::apis::urlencode(job_id)
    );
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        let entity: Option<JobsApiRoutesFineTuningGetFineTuningJobSuccess> =
            serde_json::from_str(&content).ok();
        let result = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Ok(result)
    } else {
        let entity: Option<JobsApiRoutesFineTuningGetFineTuningJobError> =
            serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

/// Get a list of fine tuning jobs for your organization and user.
pub async fn jobs_api_routes_fine_tuning_get_fine_tuning_jobs(
    configuration: &configuration::Configuration,
    params: JobsApiRoutesFineTuningGetFineTuningJobsParams,
) -> Result<
    ResponseContent<JobsApiRoutesFineTuningGetFineTuningJobsSuccess>,
    Error<JobsApiRoutesFineTuningGetFineTuningJobsError>,
> {
    let configuration = configuration;

    // unbox the parameters
    let page = params.page;
    let page_size = params.page_size;
    let model = params.model;
    let status = params.status;
    let created_after = params.created_after;
    let created_by_me = params.created_by_me;
    let wandb_project = params.wandb_project;
    let wandb_name = params.wandb_name;
    let suffix = params.suffix;

    let client = &configuration.client;

    let uri_str = format!("{}/fine_tuning/jobs", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    if let Some(ref str) = page {
        req_builder = req_builder.query(&[("page", &str.to_string())]);
    }
    if let Some(ref str) = page_size {
        req_builder = req_builder.query(&[("page_size", &str.to_string())]);
    }
    if let Some(ref str) = model {
        req_builder = req_builder.query(&[("model", &str.to_string())]);
    }
    if let Some(ref str) = status {
        req_builder = req_builder.query(&[("status", &str.to_string())]);
    }
    if let Some(ref str) = created_after {
        req_builder = req_builder.query(&[("created_after", &str.to_string())]);
    }
    if let Some(ref str) = created_by_me {
        req_builder = req_builder.query(&[("created_by_me", &str.to_string())]);
    }
    if let Some(ref str) = wandb_project {
        req_builder = req_builder.query(&[("wandb_project", &str.to_string())]);
    }
    if let Some(ref str) = wandb_name {
        req_builder = req_builder.query(&[("wandb_name", &str.to_string())]);
    }
    if let Some(ref str) = suffix {
        req_builder = req_builder.query(&[("suffix", &str.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        let entity: Option<JobsApiRoutesFineTuningGetFineTuningJobsSuccess> =
            serde_json::from_str(&content).ok();
        let result = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Ok(result)
    } else {
        let entity: Option<JobsApiRoutesFineTuningGetFineTuningJobsError> =
            serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn list_models(
    configuration: &configuration::Configuration,
) -> Result<ResponseContent<ListModelsSuccess>, Error<ListModelsError>> {
    let configuration = configuration;

    // unbox the parameters

    let client = &configuration.client;

    let uri_str = format!("{}/models", configuration.base_path);
    let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        let entity: Option<ListModelsSuccess> = serde_json::from_str(&content).ok();
        let result = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Ok(result)
    } else {
        let entity: Option<ListModelsError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::ResponseError(error))
    }
}
