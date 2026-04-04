use reqwest::{Client, header};
use crate::error::OpenArcError;
use crate::management::{
    VersionResponse, MetricsResponse, DownloaderRequest, DownloaderActionRequest, 
    DownloaderListResponse, DownloaderResponse, ServerStatusResponse, 
    LoadModelRequest, UnloadModelRequest, BenchmarkRequest, BenchmarkResponse, 
    LoadModelResponse, UnloadModelResponse, LocalModelsResponse
};

pub struct OpenArcClient {
    base_url: String,
    api_key: String,
    http_client: Client,
}

impl OpenArcClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        let mut auth_value = header::HeaderValue::from_str(&format!("Bearer {}", api_key))
            .expect("Invalid API key format");
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);

        let http_client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
            http_client,
        }
    }

    pub async fn get_version(&self) -> Result<VersionResponse, OpenArcError> {
        let url = format!("{}/openarc/version", self.base_url);
        let res = self.http_client.get(&url).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError { status_code: res.status().as_u16(), message: res.text().await.unwrap_or_default() });
        }
        let text = res.text().await?;
        serde_json::from_str(&text).map_err(|e| OpenArcError::ApiError { status_code: 200, message: format!("JSON Parse Error: {} for body: {}", e, text) })
    }

    pub async fn get_metrics(&self) -> Result<MetricsResponse, OpenArcError> {
        let url = format!("{}/openarc/metrics", self.base_url);
        let res = self.http_client.get(&url).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError { status_code: res.status().as_u16(), message: res.text().await.unwrap_or_default() });
        }
        let text = res.text().await?;
        serde_json::from_str(&text).map_err(|e| OpenArcError::ApiError { status_code: 200, message: format!("JSON Parse Error: {} for body: {}", e, text) })
    }

    pub async fn start_download(&self, req: DownloaderRequest) -> Result<DownloaderResponse, OpenArcError> {
        let url = format!("{}/openarc/downloader", self.base_url);
        let res = self.http_client.post(&url).json(&req).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError { status_code: res.status().as_u16(), message: res.text().await.unwrap_or_default() });
        }
        Ok(res.json().await?)
    }

    pub async fn list_downloads(&self) -> Result<DownloaderListResponse, OpenArcError> {
        let url = format!("{}/openarc/downloader", self.base_url);
        let res = self.http_client.get(&url).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError { status_code: res.status().as_u16(), message: res.text().await.unwrap_or_default() });
        }
        Ok(res.json().await?)
    }

    pub async fn cancel_download(&self, req: DownloaderActionRequest) -> Result<DownloaderResponse, OpenArcError> {
        let url = format!("{}/openarc/downloader", self.base_url);
        // Reqwest delete doesn't have an explicit body method typically? It does via `Client::request(Method::DELETE...)` but `Client::delete(...).json(...)` also works in recent versions
        let res = self.http_client.request(reqwest::Method::DELETE, &url).json(&req).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError { status_code: res.status().as_u16(), message: res.text().await.unwrap_or_default() });
        }
        Ok(res.json().await?)
    }

    pub async fn pause_download(&self, req: DownloaderActionRequest) -> Result<DownloaderResponse, OpenArcError> {
        let url = format!("{}/openarc/downloader/pause", self.base_url);
        let res = self.http_client.post(&url).json(&req).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError { status_code: res.status().as_u16(), message: res.text().await.unwrap_or_default() });
        }
        Ok(res.json().await?)
    }

    pub async fn resume_download(&self, req: DownloaderActionRequest) -> Result<DownloaderResponse, OpenArcError> {
        let url = format!("{}/openarc/downloader/resume", self.base_url);
        let res = self.http_client.post(&url).json(&req).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError { status_code: res.status().as_u16(), message: res.text().await.unwrap_or_default() });
        }
        Ok(res.json().await?)
    }

    pub async fn get_local_models(&self, path: Option<&str>) -> Result<LocalModelsResponse, OpenArcError> {
        let mut url = format!("{}/openarc/models", self.base_url);
        if let Some(p) = path {
            url.push_str(&format!("?path={}", p));
        }
        let res = self.http_client.get(&url).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError { status_code: res.status().as_u16(), message: res.text().await.unwrap_or_default() });
        }
        let text = res.text().await?;
        serde_json::from_str(&text).map_err(|e| OpenArcError::ApiError { status_code: 200, message: format!("JSON Parse Error: {} for body: {}", e, text) })
    }

    pub async fn update_local_model_config(&self, model_path: &str, config: serde_json::Value) -> Result<(), OpenArcError> {
        let url = format!("{}/openarc/models/update", self.base_url);
        let req_body = serde_json::json!({
            "model_path": model_path,
            "config": config
        });
        
        let res = self.http_client.post(&url).json(&req_body).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError { status_code: res.status().as_u16(), message: res.text().await.unwrap_or_default() });
        }
        Ok(())
    }

    pub async fn get_status(&self) -> Result<ServerStatusResponse, OpenArcError> {
        let url = format!("{}/openarc/status", self.base_url);
        let res = self.http_client.get(&url).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError { status_code: res.status().as_u16(), message: res.text().await.unwrap_or_default() });
        }
        let text = res.text().await?;
        serde_json::from_str(&text).map_err(|e| OpenArcError::ApiError { status_code: 200, message: format!("JSON Parse Error: {} for body: {}", e, text) })
    }

    pub async fn load_model(&self, req: LoadModelRequest) -> Result<LoadModelResponse, OpenArcError> {
        let url = format!("{}/openarc/load", self.base_url);
        let res = self.http_client.post(&url).json(&req).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError { status_code: res.status().as_u16(), message: res.text().await.unwrap_or_default() });
        }
        let text = res.text().await?;
        serde_json::from_str(&text).map_err(|e| OpenArcError::ApiError { status_code: 200, message: format!("JSON Parse Error: {} for body: {}", e, text) })
    }

    pub async fn unload_model(&self, req: UnloadModelRequest) -> Result<UnloadModelResponse, OpenArcError> {
        let url = format!("{}/openarc/unload", self.base_url);
        let res = self.http_client.post(&url).json(&req).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError { status_code: res.status().as_u16(), message: res.text().await.unwrap_or_default() });
        }
        let text = res.text().await?;
        serde_json::from_str(&text).map_err(|e| OpenArcError::ApiError { status_code: 200, message: format!("JSON Parse Error: {} for body: {}", e, text) })
    }

    pub async fn benchmark(&self, req: BenchmarkRequest) -> Result<BenchmarkResponse, OpenArcError> {
        let url = format!("{}/openarc/bench", self.base_url);
        let res = self.http_client.post(&url).json(&req).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError { status_code: res.status().as_u16(), message: res.text().await.unwrap_or_default() });
        }
        let text = res.text().await?;
        serde_json::from_str(&text).map_err(|e| OpenArcError::ApiError { status_code: 200, message: format!("JSON Parse Error: {} for body: {}", e, text) })
    }
}
