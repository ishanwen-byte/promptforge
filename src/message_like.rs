use crate::prompt_template::PromptTemplate;
use messageforge::BaseMessage;
use std::sync::Arc;

#[derive(Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use crate::Template;

    use super::*;
    use messageforge::MessageType;
    use messageforge::{AiMessage, HumanMessage, SystemMessage};

    #[test]
    fn test_from_base_message_human() {
        let human_message = HumanMessage::new("Hello, how are you?");

        let message_like = MessageLike::from_base_message(human_message);

        if let MessageLike::BaseMessage(msg) = message_like {
            assert_eq!(msg.content(), "Hello, how are you?");
            assert_eq!(msg.message_type(), MessageType::Human);
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        }
    }

    #[test]
    fn test_from_base_message_ai() {
        let ai_message = AiMessage::new("I am an AI.");

        let message_like = MessageLike::from_base_message(ai_message);

        if let MessageLike::BaseMessage(msg) = message_like {
            assert_eq!(msg.content(), "I am an AI.");
            assert_eq!(msg.message_type(), MessageType::Ai);
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        }
    }

    #[test]
    fn test_from_base_message_system() {
        let system_message = SystemMessage::new("You are a helpful assistant.");

        let message_like = MessageLike::from_base_message(system_message);

        if let MessageLike::BaseMessage(msg) = message_like {
            assert_eq!(msg.content(), "You are a helpful assistant.");
            assert_eq!(msg.message_type(), MessageType::System);
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        }
    }

    #[test]
    fn test_from_prompt_template() {
        let template = PromptTemplate::new("Hello, {name}!").unwrap();
        let message_like = MessageLike::from_prompt_template(template);

        if let MessageLike::PromptTemplate(tmpl) = message_like {
            assert_eq!(tmpl.template(), "Hello, {name}!");
            assert_eq!(tmpl.input_variables(), vec!["name"]);
        } else {
            panic!("Expected MessageLike::PromptTemplate variant.");
        }
    }

    #[test]
    fn test_clone_message_like() {
        let human_message = HumanMessage::new("Hello!");
        let message_like = MessageLike::from_base_message(human_message);
        let cloned_message_like = message_like.clone();

        if let MessageLike::BaseMessage(msg) = cloned_message_like {
            assert_eq!(msg.content(), "Hello!");
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        }

        let template = PromptTemplate::new("Hello, {name}!").unwrap();
        let message_like = MessageLike::from_prompt_template(template);
        let cloned_message_like = message_like.clone();

        if let MessageLike::PromptTemplate(tmpl) = cloned_message_like {
            assert_eq!(tmpl.template(), "Hello, {name}!");
        } else {
            panic!("Expected MessageLike::PromptTemplate variant.");
        }
    }
}
