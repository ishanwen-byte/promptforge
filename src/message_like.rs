use crate::role::Role;
use crate::template::Template;
use crate::MessagesPlaceholder;
use messageforge::{AiMessage, HumanMessage, MessageEnum, SystemMessage, ToolMessage};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum MessageLike {
    BaseMessage(Arc<MessageEnum>),
    RolePromptTemplate(Role, Arc<Template>),
    Placeholder(MessagesPlaceholder),
}

impl MessageLike {
    pub fn from_base_message(message: MessageEnum) -> Self {
        MessageLike::BaseMessage(Arc::new(message))
    }

    pub fn from_role_prompt_template(role: Role, template: Template) -> Self {
        MessageLike::RolePromptTemplate(role, Arc::new(template))
    }

    pub fn from_placeholder(placeholder: MessagesPlaceholder) -> Self {
        MessageLike::Placeholder(placeholder)
    }

    fn match_message_enum<T>(
        &self,
        extract_message: impl Fn(&MessageEnum) -> Option<&T>,
    ) -> Option<&T> {
        if let MessageLike::BaseMessage(ref message_enum) = self {
            extract_message(message_enum)
        } else {
            None
        }
    }

    pub fn as_human(&self) -> Option<&HumanMessage> {
        self.match_message_enum(MessageEnum::as_human)
    }

    pub fn as_ai(&self) -> Option<&AiMessage> {
        self.match_message_enum(MessageEnum::as_ai)
    }

    pub fn as_system(&self) -> Option<&SystemMessage> {
        self.match_message_enum(MessageEnum::as_system)
    }

    pub fn as_tool(&self) -> Option<&ToolMessage> {
        self.match_message_enum(MessageEnum::as_tool)
    }
}

pub trait ArcMessageEnumExt {
    fn unwrap_enum(self) -> MessageEnum;
}

impl ArcMessageEnumExt for Arc<MessageEnum> {
    fn unwrap_enum(self) -> MessageEnum {
        Arc::try_unwrap(self).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Templatable;
    use messageforge::{AiMessage, HumanMessage, SystemMessage};
    use messageforge::{BaseMessage as _, MessageType};

    #[test]
    fn test_from_base_message_human() {
        let human_message = HumanMessage::new("Hello, how are you?");

        let message_like = MessageLike::from_base_message(human_message.into());

        if let MessageLike::BaseMessage(msg_enum) = message_like {
            let msg = msg_enum.unwrap_enum();
            assert_eq!(msg.content(), "Hello, how are you?");
            assert_eq!(msg.message_type(), &MessageType::Human);
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        }
    }

    #[test]
    fn test_from_base_message_ai() {
        let ai_message = AiMessage::new("I am an AI.").into();

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
        let system_message = SystemMessage::new("You are a helpful assistant.");

        let message_like = MessageLike::from_base_message(system_message.into());

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
        let human_message = HumanMessage::new("Hello!");
        let message_like = MessageLike::from_base_message(human_message.into());
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
