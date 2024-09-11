use crate::message_like::MessageLike;
use crate::prompt_template::PromptTemplate;

pub struct ChatPromptTemplate {
    pub prompt: PromptTemplate,
    pub message: MessageLike,
}
