use crate::client::OpenArcClient;
use crate::error::OpenArcError;
use bytes::Bytes;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeechRequest {
    pub model: String,
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openarc_tts: Option<serde_json::Value>,
}

impl OpenArcClient {
    pub async fn audio_speech(&self, req: &SpeechRequest) -> Result<Bytes, OpenArcError> {
        let url = format!("{}/v1/audio/speech", self.base_url());
        let res = self.http().post(&url).json(req).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError {
                status_code: res.status().as_u16(),
                message: res.text().await.unwrap_or_default(),
            });
        }
        Ok(res.bytes().await?)
    }

    pub async fn audio_speech_stream(
        &self,
        req: &SpeechRequest,
    ) -> Result<reqwest::Response, OpenArcError> {
        let url = format!("{}/v1/audio/speech", self.base_url());
        let res = self.http().post(&url).json(req).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError {
                status_code: res.status().as_u16(),
                message: res.text().await.unwrap_or_default(),
            });
        }
        Ok(res)
    }

    pub async fn audio_transcribe(
        &self,
        file_path: &Path,
        model: &str,
        response_format: &str,
        openarc_asr: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, OpenArcError> {
        let file_bytes = tokio::fs::read(file_path).await.map_err(|e| {
            OpenArcError::ApiError {
                status_code: 0,
                message: format!("Failed to read audio file {}: {}", file_path.display(), e),
            }
        })?;

        let file_name = file_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("audio.bin")
            .to_string();

        let mime = mime_from_ext(file_path);

        let part = Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str(mime)
            .map_err(OpenArcError::RequestError)?;

        let mut form = Form::new()
            .text("model", model.to_string())
            .text("response_format", response_format.to_string())
            .part("file", part);

        if let Some(cfg) = openarc_asr {
            form = form.text("openarc_asr", serde_json::to_string(cfg)?);
        }

        let url = format!("{}/v1/audio/transcriptions", self.base_url());
        let res = self.http().post(&url).multipart(form).send().await?;
        if !res.status().is_success() {
            return Err(OpenArcError::ApiError {
                status_code: res.status().as_u16(),
                message: res.text().await.unwrap_or_default(),
            });
        }

        let text = res.text().await?;

        match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(v) => Ok(v),
            Err(_) => Ok(serde_json::json!({ "text": text })),
        }
    }
}

fn mime_from_ext(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .as_deref()
    {
        Some("wav") => "audio/wav",
        Some("mp3") => "audio/mpeg",
        Some("flac") => "audio/flac",
        Some("ogg") => "audio/ogg",
        Some("m4a") | Some("mp4") => "audio/mp4",
        Some("webm") => "audio/webm",
        _ => "application/octet-stream",
    }
}
