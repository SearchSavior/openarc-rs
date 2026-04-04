use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ModelStatusEnum {
    Loaded,
    Loading,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ModelType {
    Llm,
    Vlm,
    Whisper,
    Kokoro,
    Emb,
    Rerank,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum EngineType {
    Optimum,
    Ovgenai,
    Openvino,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelStatus {
    pub model_name: String,
    pub status: ModelStatusEnum,
    #[serde(rename = "model_type")]
    pub mtype: ModelType,
    pub engine: EngineType,
    pub device: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_loaded: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerStatusResponse {
    pub total_loaded_models: u32,
    pub models: Vec<ModelStatus>,
    pub openai_model_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoadModelRequest {
    pub model_path: String,
    pub model_name: String,
    pub model_type: ModelType,
    pub engine: EngineType,
    pub device: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlm_type: Option<String>,
    pub runtime_config: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_model_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_assistant_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_confidence_threshold: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoadModelResponse {
    pub status: String,
    pub model_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnloadModelRequest {
    pub model_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnloadModelResponse {
    pub status: String,
    pub model_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BenchmarkRequest {
    pub model: String,
    pub input_ids: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BenchmarkMetrics {
    #[serde(alias = "decode_throughput (tokens/s)")]
    pub decode_throughput: f32,
    #[serde(alias = "decode_duration (s)")]
    pub decode_duration: f32,
    pub total_token: u32,
    #[serde(alias = "prefill_throughput (tokens/s)")]
    pub prefill_throughput: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BenchmarkResponse {
    pub metrics: BenchmarkMetrics,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionResponse {
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetricCPU {
    pub id: String,
    pub name: String,
    pub cores: u32,
    pub threads: u32,
    pub usage: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetricGPU {
    pub id: String,
    pub name: String,
    pub total_vram: u32,
    pub used_vram: u32,
    pub usage: f32,
    pub is_shared: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetricNPU {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetricsResponse {
    pub cpus: Vec<MetricCPU>,
    pub total_ram: u32,
    pub used_ram: u32,
    pub gpus: Vec<MetricGPU>,
    pub npus: Vec<MetricNPU>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloaderRequest {
    pub model_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloaderActionRequest {
    pub model_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloaderStatus {
    pub model_name: String,
    pub total_size: u64,
    pub downloaded_size: u64,
    pub status: String,
    pub progress: u32,
    pub download_speed: u32,
    pub path: String,
    pub started_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloaderListResponse {
    pub models: Vec<DownloaderStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloaderResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocalModelInfo {
    pub id: String,
    pub path: String,
    pub model_name: Option<String>,
    pub model_type: Option<String>,
    pub engine: Option<String>,
    #[serde(default)]
    pub has_config: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocalModelsResponse {
    pub models: Vec<LocalModelInfo>,
}
