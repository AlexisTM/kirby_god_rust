use ollama_rs::{
    generation::{
        chat::{request::ChatMessageRequest, ChatMessage},
        options::GenerationOptions,
    },
    Ollama,
};

#[derive(Debug)]
pub struct OllamaAI {
    pub ollama: Ollama,
    options: GenerationOptions,
    pub model: String,
}

impl OllamaAI {
    pub fn new(model: &str, options: GenerationOptions) -> Self {
        Self {
            ollama: Ollama::new_default_with_history(100),
            options,
            model: model.to_owned(),
        }
    }

    pub async fn request(
        &mut self,
        messages: &[ChatMessage],
        history_id: &str,
    ) -> Option<ChatMessage> {
        let request = ChatMessageRequest::new(self.model.clone(), messages.to_owned());
        let response = self
            .ollama
            .send_chat_messages_with_history(request.options(self.options.clone()), history_id)
            .await;
        if let Ok(response) = response {
            return response.message;
        }
        None
    }

    pub fn clear(&mut self, history_id: &str) {
        self.ollama.clear_messages_for_id(history_id);
    }
}
