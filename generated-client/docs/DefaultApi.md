# \DefaultApi

All URIs are relative to *https://api.mistral.ai/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_chat_completion**](DefaultApi.md#create_chat_completion) | **POST** /chat/completions | Create Chat Completions
[**create_embedding**](DefaultApi.md#create_embedding) | **POST** /embeddings | Create Embeddings
[**create_fim_completion**](DefaultApi.md#create_fim_completion) | **POST** /fim/completions | Create FIM Completions
[**files_api_routes_delete_file**](DefaultApi.md#files_api_routes_delete_file) | **DELETE** /files/{file_id} | Delete File
[**files_api_routes_list_files**](DefaultApi.md#files_api_routes_list_files) | **GET** /files | List Files
[**files_api_routes_retrieve_file**](DefaultApi.md#files_api_routes_retrieve_file) | **GET** /files/{file_id} | Retrieve File
[**files_api_routes_upload_file**](DefaultApi.md#files_api_routes_upload_file) | **POST** /files | Upload File
[**jobs_api_routes_fine_tuning_cancel_fine_tuning_job**](DefaultApi.md#jobs_api_routes_fine_tuning_cancel_fine_tuning_job) | **POST** /fine_tuning/jobs/{job_id}/cancel | Cancel Fine Tuning Job
[**jobs_api_routes_fine_tuning_create_fine_tuning_job**](DefaultApi.md#jobs_api_routes_fine_tuning_create_fine_tuning_job) | **POST** /fine_tuning/jobs | Create Fine Tuning Job
[**jobs_api_routes_fine_tuning_get_fine_tuning_job**](DefaultApi.md#jobs_api_routes_fine_tuning_get_fine_tuning_job) | **GET** /fine_tuning/jobs/{job_id} | Get Fine Tuning Job
[**jobs_api_routes_fine_tuning_get_fine_tuning_jobs**](DefaultApi.md#jobs_api_routes_fine_tuning_get_fine_tuning_jobs) | **GET** /fine_tuning/jobs | List Fine Tuning Jobs
[**list_models**](DefaultApi.md#list_models) | **GET** /models | List Available Models



## create_chat_completion

> models::ChatResponse create_chat_completion(chat_request)
Create Chat Completions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_request** | [**ChatRequest**](ChatRequest.md) |  | [required] |

### Return type

[**models::ChatResponse**](ChatResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_embedding

> models::EmbeddingResponse create_embedding(embedding_request)
Create Embeddings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**embedding_request** | [**EmbeddingRequest**](EmbeddingRequest.md) |  | [required] |

### Return type

[**models::EmbeddingResponse**](EmbeddingResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_fim_completion

> models::FimCompletionResponse create_fim_completion(fim_completion_request)
Create FIM Completions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fim_completion_request** | [**FimCompletionRequest**](FimCompletionRequest.md) |  | [required] |

### Return type

[**models::FimCompletionResponse**](FIMCompletionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_api_routes_delete_file

> models::DeleteFileOut files_api_routes_delete_file(file_id)
Delete File

Delete a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** |  | [required] |

### Return type

[**models::DeleteFileOut**](DeleteFileOut.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_api_routes_list_files

> models::ListFilesOut files_api_routes_list_files()
List Files

Returns a list of files that belong to the user's organization.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListFilesOut**](ListFilesOut.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_api_routes_retrieve_file

> models::RetrieveFileOut files_api_routes_retrieve_file(file_id)
Retrieve File

Returns information about a specific file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** |  | [required] |

### Return type

[**models::RetrieveFileOut**](RetrieveFileOut.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_api_routes_upload_file

> models::UploadFileOut files_api_routes_upload_file(purpose, file)
Upload File

Upload a file that can be used across various endpoints.  The size of individual files can be a maximum of 512 MB. The Fine-tuning API only supports .jsonl files.  Please contact us if you need to increase these storage limits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purpose** | Option<[**serde_json::Value**](serde_json::Value.md)> |  | [required] |
**file** | **std::path::PathBuf** | The File object (not file name) to be uploaded.   To upload a file and specify a custom file name you should format your request as such:   ```   file=@path/to/your/file.jsonl;filename=custom_name.jsonl   ```  Otherwise, you can just keep the original file name:   ```   file=@path/to/your/file.jsonl   ```  | [required] |

### Return type

[**models::UploadFileOut**](UploadFileOut.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_api_routes_fine_tuning_cancel_fine_tuning_job

> models::DetailedJobOut jobs_api_routes_fine_tuning_cancel_fine_tuning_job(job_id)
Cancel Fine Tuning Job

Request the cancellation of a fine tuning job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DetailedJobOut**](DetailedJobOut.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_api_routes_fine_tuning_create_fine_tuning_job

> models::FineTuningJobResponse jobs_api_routes_fine_tuning_create_fine_tuning_job(job_in, dry_run)
Create Fine Tuning Job

Create a new fine tuning job, it will be queued for processing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_in** | [**JobIn**](JobIn.md) |  | [required] |
**dry_run** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::FineTuningJobResponse**](FineTuningJobResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_api_routes_fine_tuning_get_fine_tuning_job

> models::DetailedJobOut jobs_api_routes_fine_tuning_get_fine_tuning_job(job_id)
Get Fine Tuning Job

Get a fine tuned job details by its UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DetailedJobOut**](DetailedJobOut.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_api_routes_fine_tuning_get_fine_tuning_jobs

> models::JobsOut jobs_api_routes_fine_tuning_get_fine_tuning_jobs(page, page_size, model, status, created_after, created_by_me, wandb_project, wandb_name, suffix)
List Fine Tuning Jobs

Get a list of fine tuning jobs for your organization and user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page number of the results to be returned. |  |[default to 0]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]
**model** | Option<**String**> | The model name used for fine-tuning to filter on. When set, the other results are not displayed. |  |
**status** | Option<**String**> | The current job state to filter on. When set, the other results are not displayed. |  |
**created_after** | Option<**String**> |  |  |
**created_by_me** | Option<**bool**> |  |  |[default to false]
**wandb_project** | Option<**String**> |  |  |
**wandb_name** | Option<**String**> |  |  |
**suffix** | Option<**String**> |  |  |

### Return type

[**models::JobsOut**](JobsOut.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_models

> models::ModelList list_models()
List Available Models

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ModelList**](ModelList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

