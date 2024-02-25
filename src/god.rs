use ollama_rs::generation::chat::MessageRole;
use ollama_rs::generation::options::GenerationOptions;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serenity::prelude::{RwLock, TypeMapKey};

use crate::ollama::OllamaAI;
use ollama_rs::generation::chat::ChatMessage;
use std::clone::Clone;
use std::collections::HashMap;
use std::sync::Arc;

const MAX_RECOLLECTIONS: usize = 10;

// The nursery allows to find the god we are interested in, in all those servers
pub struct GodNursery;
impl TypeMapKey for GodNursery {
    type Value = RwLock<HashMap<u64, Arc<RwLock<God>>>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GodConfig {
    pub model: String,
    pub botname: String,
    pub options: GenerationOptions,
}

impl TypeMapKey for GodConfig {
    type Value = GodConfig;
}

impl Default for GodConfig {
    fn default() -> Self {
        let options = GenerationOptions::default()
            .num_ctx(4096)
            .num_predict(256)
            .temperature(0.8)
            .top_k(40)
            .top_p(0.9)
            .num_gpu(100)
            .num_thread(4);

        Self {
            model: "marvin".to_owned(),
            botname: "Marvin".to_owned(),
            options,
        }
    }
}

// trait Bot, for God
#[derive(Debug)]
pub struct God {
    pub brain: OllamaAI,
    pub config: GodConfig,
    // The actual live memory of the bot.
    recollections: Vec<ChatMessage>,
}

impl Default for God {
    fn default() -> Self {
        let config = GodConfig::default();
        Self::from_config(config)
    }
}

impl God {
    pub fn get_prompt(&self, author: &str, prompt: &str) -> Vec<ChatMessage> {
        let mut prompts = self.recollections.clone();
        prompts.push(ChatMessage::user(format!("{author}: {prompt}").to_owned()));
        prompts
    }

    pub fn set_prompt_response(&mut self, author: &str, prompt: &str, response: &str) {
        self.recollections.push(ChatMessage::user(
            format!("{author}: {}", prompt).to_owned(),
        ));
        self.recollections
            .push(ChatMessage::assistant(response.to_owned()));

        if self.recollections.len() > (MAX_RECOLLECTIONS * 2) {
            self.recollections.remove(0);
            self.recollections.remove(0);
        }
    }

    pub fn set_botname(&mut self, name: &str) {
        self.config.botname = name.to_string();
    }

    pub fn get_botname(&self) -> String {
        self.config.botname.clone()
    }

    // Remove recollections
    pub fn clear(&mut self) {
        self.recollections.clear();
    }

    pub fn from_config(config: GodConfig) -> God {
        God {
            brain: OllamaAI::new(&config.model, config.options.clone()),
            recollections: Vec::new(),
            config,
        }
    }

    pub fn update_from_config(&mut self, config: GodConfig) {
        self.brain = OllamaAI::new(&config.model, config.options.clone());
        self.recollections = Vec::new();
        self.config = config;
    }

    pub fn export_json(&self) -> serde_json::Value {
        json!(self.config)
    }

    pub fn import_json(val: &str) -> Option<Self> {
        if let Ok(config) = serde_json::from_str::<GodConfig>(val) {
            Some(Self::from_config(config))
        } else {
            None
        }
    }
    pub fn get_config(&self) -> String {
        let recollections: String = self
            .recollections
            .iter()
            .map(|x| match x.role {
                MessageRole::System => format!("System: {}\\nn", x.content),
                MessageRole::Assistant => format!("bot: {}\n", x.content),
                MessageRole::User => format!("{}\n", x.content),
            })
            .collect();
        format!(
            "{botname} config.
===========
Recollections
---------------
{recollections}\n",
            botname = self.config.botname,
            recollections = recollections,
        )
    }
}
