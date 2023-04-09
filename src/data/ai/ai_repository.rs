use crate::domain::ai::AIRepository;
use async_trait::async_trait;
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};
use shaku::{Component};


#[derive(Component)]
#[shaku(interface = AIRepository)]
pub struct AIRepositoryImpl;


#[async_trait]
impl AIRepository for AIRepositoryImpl {
    async fn generate_first_message_help(&self, user_message: String) -> Result<String, Box<dyn std::error::Error>> {
        let messages = vec![
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::System,
                content: include_str!("prompts/first_message_prompt.txt").to_string(),
                name: None,
            },
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: user_message,
                name: None,
            },
        ];

        let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages)
            .create()
            .await??;
        let first_choice = chat_completion.choices.first().ok_or("No chat completions returned")?;

        Ok(first_choice.message.content.to_string())
    }
}