use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Profile {
    pub appearance: ApperanceConfig,
    pub llm_backend: LLMBackendConfig,
    pub tts_backend: TTSBackendConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApperanceConfig {
    pub model_type: ApperanceModelType,
    pub model_path: String,
    pub scale: f32,
    pub position_x: f32,
    pub position_y: f32,
    pub default_motion: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LLMBackendConfig {
    pub provider: LLMProvider,
    pub model_name: String,
    pub temperature: f32,
    pub prompt_template: String,
    pub summary_threshold: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TTSBackendConfig {
    pub engine: TTSEngine,
    pub speaker: String,
    pub speed: f32,
    pub pitch: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ApperanceModelType {
    Live2D,
    VRM,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LLMProvider {
    Claude,
    Deepseek,
    Gemini,
    Groq,
    Mistral,
    Ollama,
    OpenAI,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TTSEngine {
    Azure,
    ElevenLabs,
    Google,
    OpenAI,
    System,
    VITS,
    VoiceVox,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            appearance: ApperanceConfig {
                model_type: ApperanceModelType::Live2D,
                model_path: String::new(),
                scale: 1.0,
                position_x: 0.0,
                position_y: 0.0,
                default_motion: String::from("idle"),
            },
            llm_backend: LLMBackendConfig {
                provider: LLMProvider::OpenAI,
                model_name: String::from("gpt-4o"),
                temperature: 0.7,
                prompt_template: String::new(),
                summary_threshold: 10,
            },
            tts_backend: TTSBackendConfig {
                engine: TTSEngine::System,
                speaker: String::new(),
                speed: 1.0,
                pitch: 1.0,
            },
        }
    }
}

use crate::core::renderer::frontend::{
    Frontend,
    Live2dFrontend,
    VrmFrontend
};

impl ApperanceModelType {
    pub fn create_frontend(&self, model_path: PathBuf) -> Box<dyn Frontend> {
        match self {
            ApperanceModelType::Live2D => {
                Box::new(Live2dFrontend::new(model_path))
            },
            ApperanceModelType::VRM => {
                Box::new(VrmFrontend::new(model_path))
            },
        }
    }
}

impl LLMProvider {
    pub fn api_url(&self) -> &'static str {
        match self {
            LLMProvider::Claude   => "https://api.anthropic.com/v1/",
            LLMProvider::Deepseek => "https://api.deepseek.com/",
            LLMProvider::Gemini   => "https://generativelanguage.googleapis.com/v1beta/openai/",
            LLMProvider::Groq     => "https://api.groq.com/openai/v1/",
            LLMProvider::Mistral  => "https://api.mistral.ai/v1/",
            LLMProvider::Ollama   => "http://localhost:11434/v1/",
            LLMProvider::OpenAI   => "https://api.openai.com/v1/",
        }
    }
}

impl TTSEngine {
    pub fn create_provider(&self) {
        match self {
            TTSEngine::Azure => {
                // Create Azure TTS provider
            },
            TTSEngine::ElevenLabs => {
                // Create ElevenLabs TTS provider
            },
            TTSEngine::Google => {
                // Create Google TTS provider
            },
            TTSEngine::OpenAI => {
                // Create OpenAI TTS provider
            },
            TTSEngine::System => {
                // Create System TTS provider
            },
            TTSEngine::VITS => {
                // Create VITS TTS provider
            },
            TTSEngine::VoiceVox => {
                // Create VoiceVox TTS provider
            }
        }
    }
}