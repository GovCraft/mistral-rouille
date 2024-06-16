#!/bin/bash

# Define directories and files relative to the script's location
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
PROJECT_ROOT="${SCRIPT_DIR}/.."
OPENAPI_URL="https://docs.mistral.ai/redocusaurus/plugin-redoc-0.yaml"
OPENAPI_FILE="${PROJECT_ROOT}/openapi/plugin-redoc-0.yaml"
CONFIG_FILE="${PROJECT_ROOT}/config/config.yaml"
TEMPLATES_DIR="${PROJECT_ROOT}/templates"
OUTPUT_DIR="${PROJECT_ROOT}/generated-client"

rm -rf $OUTPUT_DIR/src
# Ensure the output directory exists
mkdir -p $OUTPUT_DIR

# Download the latest OpenAPI spec
wget -O $OPENAPI_FILE $OPENAPI_URL

# Replace all instances of 'type: bool' with 'type: boolean' in the OpenAPI spec
sed -i 's/\btype: bool\b/type: boolean/g' $OPENAPI_FILE

# Run OpenAPI Generator
java -jar /usr/local/bin/openapi-generator-cli.jar generate \
    -i $OPENAPI_FILE \
    -g rust \
    -c $CONFIG_FILE \
    -t $TEMPLATES_DIR \
    -o $OUTPUT_DIR \
    --skip-validate-spec \
    --inline-schema-name-mappings  CreateChatCompletionParams=ChatParams,ChatCompletionResponseFunctionCall_choices_inner_message_tool_calls_inner=ChatCompletionResponseFnChoicesMessageTool,ChatCompletionResponse_usage=ChatUsage,ChatCompletionResponseFunctionCall_choices_inner_message_tool_calls_inner_function=ChatCompletionFnTools,ChatCompletionResponseJSONMode_usage=JsonModeUsage,ResponseJSONMode_choices_inner=JsonModeChoices,RequestFunctionCall_tools_inner=Tools,ChatCompletionRequestFunctionCall_tools_inner_function=ToolsFn,ChatCompletionResponseFunctionCall_choices_inner_message=Message,ChatCompletionResponseFunctionCall_choices_inner=Choices,ChatCompletionResponseFunctionCall_usage=Usage,FIMCompletionResponse_usage=FimUsage,FIMCompletionResponse_choices_inner=FimChoices,FIMCompletionResponse_choices_inner_message=FimMessages,EmbeddingResponse_usage=EmbeddingUsage,EmbeddingResponse_data_inner=EmbeddingData,ChatCompletionResponse_choices_inner=Choices,FIMCompletionRequest_stop=FimStop,ChatCompletionRequestFunctionCall_messages_inner=ChatFnMessages,ChatCompletionRequestJSONMode_response_format=JsonModeResponseFormat,ChatCompletionRequestJSONMode_messages_inner=JsonModeMessages,ChatCompletionRequest_messages_inner=ChatMessages,jobs_api_routes_fine_tuning_create_fine_tuning_job_200_response=FineTuningJobResponse200,createChatCompletion_200_response=ChatResponse200,ChatCompletionResponse_choices_inner_message=ChatChoices,ChatCompletionResponseJSONMode_choices_inner_message=ChatMessage,createChatCompletion_request=ChatRequest \






# Check if rustfmt is installed, and install if not
if ! command -v rustfmt &> /dev/null
then
    echo "rustfmt could not be found, installing..."
    rustup component add rustfmt
fi

# Format the generated code
find generated-client -name "*.rs" | xargs rustfmt --edition 2021

# Copy the generated README to the workspace root
cp ./generated-client/README.md ./README.md