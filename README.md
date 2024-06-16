# Mistral Rouille
Mistral Rouille is a Rust crate customized from Mistral's official OpenAPI specification. This crate provides a convenient and type-safe interface for interacting with Mistral's API, using the Reqwest crate for HTTP requests. Designed to be easily integrated into your Rust projects, Mistral Rouille offers robust functionality for all Mistral API endpoints, ensuring seamless communication with the configured endpoint.

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project and then refined with custom templates by [Govcraft](https://www.govcraft.ai). This crate will be regenerated and updated as Mistral updates it's OpenAPI specification. Additional customizations are planned.

- API version: 0.0.2
- Crate version: 1.1.4
    - Build date: 2024-06-15T23:12:35.234496-06:00[America/Mexico_City]
- Generator version: 7.6.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

Either run:
```shell
cargo add mistral-rouille
```
or add
```toml
mistral-rouille = "1.1.4"
```
to Cargo.toml

## Examples
View the tests folder for a basic example executing a chat request on Mistral's newest model Codestral.

## Documentation for API Endpoints

All URIs are relative to *https://api.mistral.ai/v1* or *https://codestral.mistral.ai/v1* if you're using Codestral

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

 - [ChatChoices](docs/ChatChoices.md)
 - [ChatCompletionRequest](docs/ChatCompletionRequest.md)
 - [ChatCompletionRequestFunctionCall](docs/ChatCompletionRequestFunctionCall.md)
 - [ChatCompletionRequestFunctionCallToolsInner](docs/ChatCompletionRequestFunctionCallToolsInner.md)
 - [ChatCompletionRequestJsonMode](docs/ChatCompletionRequestJsonMode.md)
 - [ChatCompletionResponse](docs/ChatCompletionResponse.md)
 - [ChatCompletionResponseFunctionCall](docs/ChatCompletionResponseFunctionCall.md)
 - [ChatCompletionResponseJsonMode](docs/ChatCompletionResponseJsonMode.md)
 - [ChatCompletionResponseJsonModeChoicesInner](docs/ChatCompletionResponseJsonModeChoicesInner.md)
 - [ChatFnMessages](docs/ChatFnMessages.md)
 - [ChatFnTools](docs/ChatFnTools.md)
 - [ChatMessage](docs/ChatMessage.md)
 - [ChatMessages](docs/ChatMessages.md)
 - [ChatRequest](docs/ChatRequest.md)
 - [ChatResponse](docs/ChatResponse.md)
 - [ChatResponseFnTool](docs/ChatResponseFnTool.md)
 - [ChatUsage](docs/ChatUsage.md)
 - [CheckpointOut](docs/CheckpointOut.md)
 - [Choices](docs/Choices.md)
 - [DeleteFileOut](docs/DeleteFileOut.md)
 - [DeleteFileResponse](docs/DeleteFileResponse.md)
 - [DetailedJobOut](docs/DetailedJobOut.md)
 - [EmbeddingData](docs/EmbeddingData.md)
 - [EmbeddingRequest](docs/EmbeddingRequest.md)
 - [EmbeddingResponse](docs/EmbeddingResponse.md)
 - [EmbeddingUsage](docs/EmbeddingUsage.md)
 - [Error](docs/Error.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [EventOut](docs/EventOut.md)
 - [FileSchema](docs/FileSchema.md)
 - [FimChoices](docs/FimChoices.md)
 - [FimCompletionRequest](docs/FimCompletionRequest.md)
 - [FimCompletionResponse](docs/FimCompletionResponse.md)
 - [FimMessages](docs/FimMessages.md)
 - [FimStop](docs/FimStop.md)
 - [FimUsage](docs/FimUsage.md)
 - [FineTuneableModel](docs/FineTuneableModel.md)
 - [FineTuningJobResponse](docs/FineTuningJobResponse.md)
 - [JobIn](docs/JobIn.md)
 - [JobMetadata](docs/JobMetadata.md)
 - [JobOut](docs/JobOut.md)
 - [JobsOut](docs/JobsOut.md)
 - [JsonModeMessages](docs/JsonModeMessages.md)
 - [JsonModeResponseFormat](docs/JsonModeResponseFormat.md)
 - [JsonModeUsage](docs/JsonModeUsage.md)
 - [ListFilesOut](docs/ListFilesOut.md)
 - [ListFilesResponse](docs/ListFilesResponse.md)
 - [Message](docs/Message.md)
 - [MetricOut](docs/MetricOut.md)
 - [Model](docs/Model.md)
 - [ModelList](docs/ModelList.md)
 - [RetrieveFileOut](docs/RetrieveFileOut.md)
 - [RetrieveFileResponse](docs/RetrieveFileResponse.md)
 - [ToolsFn](docs/ToolsFn.md)
 - [TrainingParameters](docs/TrainingParameters.md)
 - [UploadFileOut](docs/UploadFileOut.md)
 - [UploadFileResponse](docs/UploadFileResponse.md)
 - [Usage](docs/Usage.md)
 - [WandbIntegration](docs/WandbIntegration.md)
 - [WandbIntegrationOut](docs/WandbIntegrationOut.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

roland@govcraft.ai

