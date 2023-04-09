use std::sync::Arc;
use shaku::{Component, Interface};
use crate::domain::ai::AIRepository;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateFirstMessageHelpUseCase: Interface {
    async fn call(&self, user_message: String) -> Result<String ,Box<dyn std::error::Error>>;
}

#[derive(Component)]
#[shaku(interface=GenerateFirstMessageHelpUseCase)]
pub struct GenerateFirstMessageHelpUseCaseImpl {
    #[shaku(inject)]
    ai_repository: Arc<dyn AIRepository>
}

#[async_trait]
impl GenerateFirstMessageHelpUseCase for GenerateFirstMessageHelpUseCaseImpl {
    async fn call(&self, user_message: String) -> Result<String ,Box<dyn std::error::Error>> {
       self.ai_repository.generate_first_message_help(user_message).await
    }
}