# Rust API client for mistral-rouille

Our Chat Completion and Embeddings APIs specification. Create your account on [La Plateforme](https://console.mistral.ai) to get access and read the [docs](https://docs.mistral.ai) to learn how to use it.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.0.2
- Package version: 1.1.2
- Generator version: 7.6.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `mistral-rouille` and add the following to `Cargo.toml` under `[dependencies]`:

Either run:
```shell
cargo add mistral-rouille
```
or add
```toml
mistral-rouille = "1.1.2"
```

## Documentation for API Endpoints

All URIs are relative to *https://api.mistral.ai/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**create_chat_completion**](docs/DefaultApi.md#create_chat_completion) | **POST** /chat/completions | Create Chat Completions
*DefaultApi* | [**create_embedding**](docs/DefaultApi.md#create_embedding) | **POST** /embeddings | Create Embeddings
*DefaultApi* | [**create_fim_completion**](docs/DefaultApi.md#create_fim_completion) | **POST** /fim/completions | Create FIM Completions
*DefaultApi* | [**files_api_routes_delete_file**](docs/DefaultApi.md#files_api_routes_delete_file) | **DELETE** /files/{file_id} | Delete File
*DefaultApi* | [**files_api_routes_list_files**](docs/DefaultApi.md#files_api_routes_list_files) | **GET** /files | List Files
*DefaultApi* | [**files_api_routes_retrieve_file**](docs/DefaultApi.md#files_api_routes_retrieve_file) | **GET** /files/{file_id} | Retrieve File
*DefaultApi* | [**files_api_routes_upload_file**](docs/DefaultApi.md#files_api_routes_upload_file) | **POST** /files | Upload File
*DefaultApi* | [**jobs_api_routes_fine_tuning_cancel_fine_tuning_job**](docs/DefaultApi.md#jobs_api_routes_fine_tuning_cancel_fine_tuning_job) | **POST** /fine_tuning/jobs/{job_id}/cancel | Cancel Fine Tuning Job
*DefaultApi* | [**jobs_api_routes_fine_tuning_create_fine_tuning_job**](docs/DefaultApi.md#jobs_api_routes_fine_tuning_create_fine_tuning_job) | **POST** /fine_tuning/jobs | Create Fine Tuning Job
*DefaultApi* | [**jobs_api_routes_fine_tuning_get_fine_tuning_job**](docs/DefaultApi.md#jobs_api_routes_fine_tuning_get_fine_tuning_job) | **GET** /fine_tuning/jobs/{job_id} | Get Fine Tuning Job
*DefaultApi* | [**jobs_api_routes_fine_tuning_get_fine_tuning_jobs**](docs/DefaultApi.md#jobs_api_routes_fine_tuning_get_fine_tuning_jobs) | **GET** /fine_tuning/jobs | List Fine Tuning Jobs
*DefaultApi* | [**list_models**](docs/DefaultApi.md#list_models) | **GET** /models | List Available Models


## Documentation For Models

 - [ChatCompletionChoices](docs/ChatCompletionChoices.md)
 - [ChatCompletionColonColonCreateRequest](docs/ChatCompletionColonColonCreateRequest.md)
 - [ChatCompletionFnTools](docs/ChatCompletionFnTools.md)
 - [ChatCompletionJsonModeChoices](docs/ChatCompletionJsonModeChoices.md)
 - [ChatCompletionJsonModeUsage](docs/ChatCompletionJsonModeUsage.md)
 - [ChatCompletionMessage](docs/ChatCompletionMessage.md)
 - [ChatCompletionRequest](docs/ChatCompletionRequest.md)
 - [ChatCompletionRequestFunctionCall](docs/ChatCompletionRequestFunctionCall.md)
 - [ChatCompletionRequestFunctionCallMessages](docs/ChatCompletionRequestFunctionCallMessages.md)
 - [ChatCompletionRequestJsonMode](docs/ChatCompletionRequestJsonMode.md)
 - [ChatCompletionRequestMessages](docs/ChatCompletionRequestMessages.md)
 - [ChatCompletionResonseUsage](docs/ChatCompletionResonseUsage.md)
 - [ChatCompletionResponse](docs/ChatCompletionResponse.md)
 - [ChatCompletionResponse200](docs/ChatCompletionResponse200.md)
 - [ChatCompletionResponseFnChoicesMessageTool](docs/ChatCompletionResponseFnChoicesMessageTool.md)
 - [ChatCompletionResponseFunctionCall](docs/ChatCompletionResponseFunctionCall.md)
 - [ChatCompletionResponseJsonMode](docs/ChatCompletionResponseJsonMode.md)
 - [ChatCompletionTools](docs/ChatCompletionTools.md)
 - [ChatCompletionToolsFn](docs/ChatCompletionToolsFn.md)
 - [ChatCompletionUsage](docs/ChatCompletionUsage.md)
 - [CheckpointOut](docs/CheckpointOut.md)
 - [Choices](docs/Choices.md)
 - [CreateFineTuningJobResponse200](docs/CreateFineTuningJobResponse200.md)
 - [DeleteFileOut](docs/DeleteFileOut.md)
 - [DeleteFileResponse](docs/DeleteFileResponse.md)
 - [DetailedJobOut](docs/DetailedJobOut.md)
 - [EmbeddingRequest](docs/EmbeddingRequest.md)
 - [EmbeddingResponse](docs/EmbeddingResponse.md)
 - [EmbeddingResponseData](docs/EmbeddingResponseData.md)
 - [EmbeddingResponseUsage](docs/EmbeddingResponseUsage.md)
 - [Error](docs/Error.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [EventOut](docs/EventOut.md)
 - [FileSchema](docs/FileSchema.md)
 - [FimCompletionChoices](docs/FimCompletionChoices.md)
 - [FimCompletionRequest](docs/FimCompletionRequest.md)
 - [FimCompletionRequestStop](docs/FimCompletionRequestStop.md)
 - [FimCompletionResponse](docs/FimCompletionResponse.md)
 - [FimCompletionResponseMessages](docs/FimCompletionResponseMessages.md)
 - [FimCompletionResponseUsage](docs/FimCompletionResponseUsage.md)
 - [FineTuneableModel](docs/FineTuneableModel.md)
 - [JobIn](docs/JobIn.md)
 - [JobMetadata](docs/JobMetadata.md)
 - [JobOut](docs/JobOut.md)
 - [JobsOut](docs/JobsOut.md)
 - [JsonModeMessages](docs/JsonModeMessages.md)
 - [JsonModeResponseFormat](docs/JsonModeResponseFormat.md)
 - [ListFilesOut](docs/ListFilesOut.md)
 - [ListFilesResponse](docs/ListFilesResponse.md)
 - [MetricOut](docs/MetricOut.md)
 - [Model](docs/Model.md)
 - [ModelList](docs/ModelList.md)
 - [RetrieveFileOut](docs/RetrieveFileOut.md)
 - [RetrieveFileResponse](docs/RetrieveFileResponse.md)
 - [TrainingParameters](docs/TrainingParameters.md)
 - [UploadFileOut](docs/UploadFileOut.md)
 - [UploadFileResponse](docs/UploadFileResponse.md)
 - [WandbIntegration](docs/WandbIntegration.md)
 - [WandbIntegrationOut](docs/WandbIntegrationOut.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

mistral.rouille@govcraft.ai
