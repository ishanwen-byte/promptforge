use crate::prompt_template::PromptTemplate;
use messageforge::BaseMessage;
use std::sync::Arc;

pub enum MessageLike {
    BaseMessage(Arc<dyn BaseMessage>),
    PromptTemplate(Arc<PromptTemplate>),
}

impl MessageLike {
    pub fn from_base_message<T: BaseMessage + 'static>(message: T) -> Self {
        MessageLike::BaseMessage(Arc::new(message))
    }

    pub fn from_prompt_template(template: PromptTemplate) -> Self {
        MessageLike::PromptTemplate(Arc::new(template))
    }
}
