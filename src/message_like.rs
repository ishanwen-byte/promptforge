use crate::template::Template;
use crate::MessagesPlaceholder;
use crate::{role::Role, FewShotChatTemplate};
use messageforge::{AiMessage, HumanMessage, MessageEnum, SystemMessage, ToolMessage};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageLike {
    BaseMessage(Arc<MessageEnum>),
    RolePromptTemplate(Role, Arc<Template>),
    Placeholder(MessagesPlaceholder),
    FewShotPrompt(Box<FewShotChatTemplate>), // Boxed to avoid recursive type
}

impl MessageLike {
    pub fn base_message(message: MessageEnum) -> Self {
        MessageLike::BaseMessage(Arc::new(message))
    }

    pub fn role_prompt_template(role: Role, template: Template) -> Self {
        MessageLike::RolePromptTemplate(role, Arc::new(template))
    }

    pub fn placeholder(placeholder: MessagesPlaceholder) -> Self {
        MessageLike::Placeholder(placeholder)
    }

    pub fn few_shot_prompt(few_shot_prompt: FewShotChatTemplate) -> Self {
        MessageLike::FewShotPrompt(Box::new(few_shot_prompt))
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
    use crate::Role::{Ai, Human};
    use crate::{chats, examples, ChatTemplate, FewShotTemplate, Templatable};
    use messageforge::{AiMessage, HumanMessage, SystemMessage};
    use messageforge::{BaseMessage as _, MessageType};

    #[test]
    fn test_from_base_message_human() {
        let human_message = HumanMessage::new("Hello, how are you?");

        let message_like = MessageLike::base_message(human_message.into());

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

        let message_like = MessageLike::base_message(ai_message);

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

        let message_like = MessageLike::base_message(system_message.into());

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
        let message_like = MessageLike::role_prompt_template(Role::Human, template);

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
        let message_like = MessageLike::base_message(human_message.into());
        let cloned_message_like = message_like.clone();

        if let MessageLike::BaseMessage(msg) = cloned_message_like {
            assert_eq!(msg.content(), "Hello!");
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        }

        let template = Template::new("Hello, {name}!").unwrap();
        let message_like = MessageLike::role_prompt_template(Role::Ai, template);
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
        let message_like = MessageLike::placeholder(placeholder.clone());

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
        let message_like = MessageLike::placeholder(placeholder.clone());
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
        let message_like = MessageLike::placeholder(placeholder.clone());

        if let MessageLike::Placeholder(placeholder_msg) = message_like {
            assert_eq!(placeholder_msg.variable_name(), "history");
            assert!(placeholder_msg.optional());
            assert_eq!(placeholder_msg.n_messages(), 50);
        } else {
            panic!("Expected MessageLike::Placeholder variant.");
        }
    }

    #[test]
    fn test_from_few_shot_prompt() {
        let examples = examples!(
            ("{input}: What is 2 + 2?", "{output}: 4"),
            ("{input}: What is 2 + 3?", "{output}: 5"),
            ("{input}: What is 3 + 3?", "{output}: 6"),
        );

        let incorrect_example_prompt =
            ChatTemplate::from_messages(chats!(Human = "{input}", Ai = "{output}")).unwrap();

        let few_shot_template = FewShotTemplate::new(examples);

        let few_shot_chat_template =
            FewShotChatTemplate::new(few_shot_template, incorrect_example_prompt.clone());

        let message_like = MessageLike::few_shot_prompt(few_shot_chat_template.clone());

        if let MessageLike::FewShotPrompt(few_shot_prompt_msg) = message_like {
            assert_eq!(
                few_shot_prompt_msg.format_examples().unwrap(),
                few_shot_chat_template.format_examples().unwrap()
            );
        } else {
            panic!("Expected MessageLike::FewShotPrompt variant.");
        }
    }

    #[test]
    fn test_unwrap_enum_success() {
        let ai_message = AiMessage::new("I am an AI.").into();
        let message_like = MessageLike::base_message(ai_message);

        if let MessageLike::BaseMessage(arc_message_enum) = message_like {
            let unwrapped = arc_message_enum.unwrap_enum();

            assert!(matches!(unwrapped, MessageEnum::Ai(_)));

            if let MessageEnum::Ai(ai_message) = unwrapped {
                assert_eq!(ai_message.content(), "I am an AI.");
            } else {
                panic!("Expected AiMessage, got something else.");
            }
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        }
    }

    #[test]
    fn test_unwrap_enum_with_clone() {
        let human_message = HumanMessage::new("Hello from Human.").into();
        let message_like = MessageLike::base_message(human_message);

        let arc_message_enum = if let MessageLike::BaseMessage(arc_message_enum) = &message_like {
            Arc::clone(arc_message_enum)
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        };

        let unwrapped = arc_message_enum.unwrap_enum();

        assert!(matches!(unwrapped, MessageEnum::Human(_)));

        if let MessageEnum::Human(human_message) = unwrapped {
            assert_eq!(human_message.content(), "Hello from Human.");
        } else {
            panic!("Expected HumanMessage, got something else.");
        }
    }

    #[test]
    fn test_unwrap_enum_with_multiple_references() {
        let ai_message = AiMessage::new("Another AI message").into();
        let message_like = MessageLike::base_message(ai_message);

        let arc_message_enum1 = if let MessageLike::BaseMessage(arc_message_enum) = &message_like {
            Arc::clone(arc_message_enum)
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        };

        let unwrapped = arc_message_enum1.unwrap_enum();

        assert!(matches!(unwrapped, MessageEnum::Ai(_)));

        if let MessageEnum::Ai(ai_message) = unwrapped {
            assert_eq!(ai_message.content(), "Another AI message");
        } else {
            panic!("Expected AiMessage, got something else.");
        }
    }
}
