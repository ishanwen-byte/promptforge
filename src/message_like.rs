use crate::role::Role;
use crate::template::Template;
use crate::MessagesPlaceholder;
use messageforge::BaseMessage;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum MessageLike {
    BaseMessage(Arc<dyn BaseMessage>),
    RolePromptTemplate(Role, Arc<Template>),
    Placeholder(MessagesPlaceholder),
}

impl MessageLike {
    pub fn from_base_message(message: Arc<dyn BaseMessage>) -> Self {
        MessageLike::BaseMessage(message)
    }

    pub fn from_role_prompt_template(role: Role, template: Template) -> Self {
        MessageLike::RolePromptTemplate(role, Arc::new(template))
    }

    pub fn from_placeholder(placeholder: MessagesPlaceholder) -> Self {
        MessageLike::Placeholder(placeholder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Templatable;
    use messageforge::MessageType;
    use messageforge::{AiMessage, HumanMessage, SystemMessage};

    #[test]
    fn test_from_base_message_human() {
        let human_message = Arc::new(HumanMessage::new("Hello, how are you?"));

        let message_like = MessageLike::from_base_message(human_message);

        if let MessageLike::BaseMessage(msg) = message_like {
            assert_eq!(msg.content(), "Hello, how are you?");
            assert_eq!(msg.message_type(), &MessageType::Human);
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        }
    }

    #[test]
    fn test_from_base_message_ai() {
        let ai_message = Arc::new(AiMessage::new("I am an AI."));

        let message_like = MessageLike::from_base_message(ai_message);

        if let MessageLike::BaseMessage(msg) = message_like {
            assert_eq!(msg.content(), "I am an AI.");
            assert_eq!(msg.message_type(), &MessageType::Ai);
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        }
    }

    #[test]
    fn test_from_base_message_system() {
        let system_message = Arc::new(SystemMessage::new("You are a helpful assistant."));

        let message_like = MessageLike::from_base_message(system_message);

        if let MessageLike::BaseMessage(msg) = message_like {
            assert_eq!(msg.content(), "You are a helpful assistant.");
            assert_eq!(msg.message_type(), &MessageType::System);
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        }
    }

    #[test]
    fn test_from_role_prompt_template() {
        let template = Template::new("Hello, {name}!").unwrap();
        let message_like = MessageLike::from_role_prompt_template(Role::Human, template);

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
        let human_message = Arc::new(HumanMessage::new("Hello!"));
        let message_like = MessageLike::from_base_message(human_message);
        let cloned_message_like = message_like.clone();

        if let MessageLike::BaseMessage(msg) = cloned_message_like {
            assert_eq!(msg.content(), "Hello!");
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        }

        let template = Template::new("Hello, {name}!").unwrap();
        let message_like = MessageLike::from_role_prompt_template(Role::Ai, template);
        let cloned_message_like = message_like.clone();

        if let MessageLike::RolePromptTemplate(role, tmpl) = cloned_message_like {
            assert_eq!(role, Role::Ai);
            assert_eq!(tmpl.template(), "Hello, {name}!");
        } else {
            panic!("Expected MessageLike::RolePromptTemplate variant.");
        }
    }

    #[test]
    fn test_from_placeholder() {
        let placeholder = MessagesPlaceholder::new("history".to_string());
        let message_like = MessageLike::from_placeholder(placeholder.clone());

        if let MessageLike::Placeholder(placeholder_msg) = message_like {
            assert_eq!(placeholder_msg.variable_name(), "history");
            assert!(!placeholder_msg.optional());
            assert_eq!(
                placeholder_msg.n_messages(),
                MessagesPlaceholder::DEFAULT_LIMIT
            );
        } else {
            panic!("Expected MessageLike::Placeholder variant.");
        }
    }

    #[test]
    fn test_clone_message_like_placeholder() {
        let placeholder = MessagesPlaceholder::new("history".to_string());
        let message_like = MessageLike::from_placeholder(placeholder.clone());
        let cloned_message_like = message_like.clone();

        if let MessageLike::Placeholder(placeholder_msg) = cloned_message_like {
            assert_eq!(placeholder_msg.variable_name(), "history");
            assert!(!placeholder_msg.optional());
            assert_eq!(
                placeholder_msg.n_messages(),
                MessagesPlaceholder::DEFAULT_LIMIT
            );
        } else {
            panic!("Expected MessageLike::Placeholder variant.");
        }
    }

    #[test]
    fn test_placeholder_with_options() {
        let placeholder = MessagesPlaceholder::with_options("history".to_string(), true, 50);
        let message_like = MessageLike::from_placeholder(placeholder.clone());

        if let MessageLike::Placeholder(placeholder_msg) = message_like {
            assert_eq!(placeholder_msg.variable_name(), "history");
            assert!(placeholder_msg.optional());
            assert_eq!(placeholder_msg.n_messages(), 50);
        } else {
            panic!("Expected MessageLike::Placeholder variant.");
        }
    }
}
