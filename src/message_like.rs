use crate::prompt_template::PromptTemplate;
use crate::role::Role;
use messageforge::BaseMessage;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum MessageLike {
    BaseMessage(Arc<dyn BaseMessage>),
    RolePromptTemplate(Role, Arc<PromptTemplate>),
}

impl MessageLike {
    pub fn from_base_message(message: Box<dyn BaseMessage>) -> Self {
        MessageLike::BaseMessage(Arc::from(message))
    }

    pub fn from_role_prompt_template(role: &Role, template: PromptTemplate) -> Self {
        MessageLike::RolePromptTemplate(*role, Arc::new(template))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Template;
    use messageforge::MessageType;
    use messageforge::{AiMessage, HumanMessage, SystemMessage};

    #[test]
    fn test_from_base_message_human() {
        let human_message = Box::new(HumanMessage::new("Hello, how are you?"));

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
        let ai_message = Box::new(AiMessage::new("I am an AI."));

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
        let system_message = Box::new(SystemMessage::new("You are a helpful assistant."));

        let message_like = MessageLike::from_base_message(system_message);

        if let MessageLike::BaseMessage(msg) = message_like {
            assert_eq!(msg.content(), "You are a helpful assistant.");
            assert_eq!(msg.message_type(), MessageType::System);
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        }
    }

    #[test]
    fn test_from_role_prompt_template() {
        let template = PromptTemplate::new("Hello, {name}!").unwrap();
        let message_like = MessageLike::from_role_prompt_template(&Role::Human, template);

        if let MessageLike::RolePromptTemplate(role, tmpl) = message_like {
            assert_eq!(role, Role::Human);
            assert_eq!(tmpl.template(), "Hello, {name}!");
            assert_eq!(tmpl.input_variables(), vec!["name"]);
        } else {
            panic!("Expected MessageLike::RolePromptTemplate variant.");
        }
    }

    #[test]
    fn test_clone_message_like() {
        let human_message = Box::new(HumanMessage::new("Hello!"));
        let message_like = MessageLike::from_base_message(human_message);
        let cloned_message_like = message_like.clone();

        if let MessageLike::BaseMessage(msg) = cloned_message_like {
            assert_eq!(msg.content(), "Hello!");
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        }

        let template = PromptTemplate::new("Hello, {name}!").unwrap();
        let message_like = MessageLike::from_role_prompt_template(&Role::Ai, template);
        let cloned_message_like = message_like.clone();

        if let MessageLike::RolePromptTemplate(role, tmpl) = cloned_message_like {
            assert_eq!(role, Role::Ai);
            assert_eq!(tmpl.template(), "Hello, {name}!");
        } else {
            panic!("Expected MessageLike::RolePromptTemplate variant.");
        }
    }
}
