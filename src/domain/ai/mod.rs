use async_trait::async_trait;
use shaku::Interface;

pub mod use_cases;

#[async_trait]
pub trait AIRepository: Interface {
    async fn generate_first_message_help(
        &self,
        user_message: String,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
