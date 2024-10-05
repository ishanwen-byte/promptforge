use crate::template::Template;
use crate::{role::Role, FewShotChatTemplate};
use crate::{MessagesPlaceholder, TemplateError};
use messageforge::{AiMessage, HumanMessage, MessageEnum, SystemMessage, ToolMessage};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
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

impl TryFrom<String> for MessageLike {
    type Error = TemplateError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let json_value: serde_json::Value = serde_json::from_str(&value).map_err(|e| {
            TemplateError::MalformedTemplate(format!("Failed to parse JSON: {}", e))
        })?;

        let message_like: MessageLike = match json_value.get("type").and_then(|t| t.as_str()) {
            Some("BaseMessage") => {
                let base_message = serde_json::from_value::<MessageEnum>(
                    json_value["value"].clone(),
                )
                .map_err(|e| {
                    TemplateError::MalformedTemplate(format!(
                        "Failed to deserialize BaseMessage: {}",
                        e
                    ))
                })?;
                MessageLike::BaseMessage(Arc::new(base_message))
            }
            Some("RolePromptTemplate") => {
                let role = serde_json::from_value::<Role>(json_value["value"][0].clone()).map_err(
                    |e| {
                        TemplateError::MalformedTemplate(format!(
                            "Failed to deserialize Role: {}",
                            e
                        ))
                    },
                )?;
                let template = serde_json::from_value::<Template>(json_value["value"][1].clone())
                    .map_err(|e| {
                    TemplateError::MalformedTemplate(format!(
                        "Failed to deserialize Template: {}",
                        e
                    ))
                })?;
                MessageLike::RolePromptTemplate(role, Arc::new(template))
            }
            Some("Placeholder") => {
                let placeholder =
                    serde_json::from_value::<MessagesPlaceholder>(json_value["value"].clone())
                        .map_err(|e| {
                            TemplateError::MalformedTemplate(format!(
                                "Failed to deserialize Placeholder: {}",
                                e
                            ))
                        })?;
                MessageLike::Placeholder(placeholder)
            }
            Some("FewShotPrompt") => {
                let few_shot_prompt =
                    serde_json::from_value::<FewShotChatTemplate>(json_value["value"].clone())
                        .map_err(|e| {
                            TemplateError::MalformedTemplate(format!(
                                "Failed to deserialize FewShotPrompt: {}",
                                e
                            ))
                        })?;
                MessageLike::FewShotPrompt(Box::new(few_shot_prompt))
            }
            _ => {
                return Err(TemplateError::MalformedTemplate(
                    "Unknown MessageLike type".to_string(),
                ));
            }
        };

        Ok(message_like)
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

    #[test]
    fn test_serialize_base_message() {
        let human_message = HumanMessage::new("Hello, human.");
        let message_like = MessageLike::base_message(human_message.into());

        let serialized = serde_json::to_string(&message_like).expect("Failed to serialize");
        let expected = r#"{"type":"BaseMessage","value":{"role":"human","content":"Hello, human.","example":false,"message_type":"Human"}}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_deserialize_base_message() {
        let json_data = r#"
        {
            "type": "BaseMessage",
            "value": {
                "role": "human",
                "content": "Hello, human.",
                "example": false,
                "message_type": "Human"
            }
        }
        "#;

        let deserialized: MessageLike =
            serde_json::from_str(json_data).expect("Failed to deserialize");

        if let MessageLike::BaseMessage(msg_enum) = deserialized {
            let msg = msg_enum.unwrap_enum();
            assert_eq!(msg.content(), "Hello, human.");
            assert!(matches!(msg, MessageEnum::Human(_)));
        } else {
            panic!("Expected MessageLike::BaseMessage variant.");
        }
    }

    #[test]
    fn test_serialize_role_prompt_template() {
        let template = Template::new("Hello, {name}!").unwrap();
        let message_like = MessageLike::role_prompt_template(Role::Human, template.clone());

        let serialized = serde_json::to_string(&message_like).expect("Failed to serialize");
        let expected = r#"{"type":"RolePromptTemplate","value":["Human",{"template":"Hello, {name}!","template_format":"FmtString","input_variables":["name"]}]}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_deserialize_role_prompt_template() {
        let json_data = r#"
        {
            "type": "RolePromptTemplate",
            "value": [
                "Human",
                {
                    "template": "Hello, {name}!",
                    "template_format": "FmtString",
                    "input_variables": ["name"]
                }
            ]
        }
        "#;

        let deserialized: MessageLike =
            serde_json::from_str(json_data).expect("Failed to deserialize");
        if let MessageLike::RolePromptTemplate(role, tmpl) = deserialized {
            assert_eq!(role.to_string(), "human");
            assert_eq!(tmpl.template(), "Hello, {name}!");
            assert_eq!(tmpl.input_variables(), vec!["name"]);
        } else {
            panic!("Expected MessageLike::RolePromptTemplate variant.");
        }
    }

    #[test]
    fn test_serialize_placeholder() {
        let placeholder = MessagesPlaceholder::new("history".to_string());
        let message_like = MessageLike::placeholder(placeholder.clone());

        let serialized = serde_json::to_string(&message_like).expect("Failed to serialize");
        let expected = r#"{"type":"Placeholder","value":{"variable_name":"history","optional":false,"n_messages":100}}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_deserialize_placeholder() {
        let json_data = r#"
        {
            "type": "Placeholder",
            "value": {
                "variable_name": "history",
                "optional": false,
                "n_messages": 5
            }
        }
        "#;

        let deserialized: MessageLike =
            serde_json::from_str(json_data).expect("Failed to deserialize");
        if let MessageLike::Placeholder(placeholder_msg) = deserialized {
            assert_eq!(placeholder_msg.variable_name(), "history");
            assert_eq!(placeholder_msg.n_messages(), 5);
        } else {
            panic!("Expected MessageLike::Placeholder variant.");
        }
    }

    #[test]
    fn test_serialize_few_shot_prompt() {
        let examples = examples!(
            ("{input}: What is 2 + 2?", "{output}: 4"),
            ("{input}: What is 2 + 3?", "{output}: 5")
        );

        let example_prompt =
            ChatTemplate::from_messages(chats!(Human = "{input}", Ai = "{output}")).unwrap();

        let few_shot_template = FewShotTemplate::new(examples);
        let few_shot_chat_template = FewShotChatTemplate::new(few_shot_template, example_prompt);

        let message_like = MessageLike::few_shot_prompt(few_shot_chat_template);

        let serialized = serde_json::to_string_pretty(&message_like).expect("Failed to serialize");

        let expected_json = serde_json::json!({
            "type": "FewShotPrompt",
            "value": {
                "examples": {
                    "examples": [
                        {
                            "template": "{input}: What is 2 + 2?\n{output}: 4",
                            "template_format": "FmtString",
                            "input_variables": ["input", "output"]
                        },
                        {
                            "template": "{input}: What is 2 + 3?\n{output}: 5",
                            "template_format": "FmtString",
                            "input_variables": ["input", "output"]
                        }
                    ],
                    "example_separator": "\n\n"
                },
                "example_prompt": {
                    "messages": [
                        {
                            "type": "RolePromptTemplate",
                            "value": [
                                "Human",
                                {
                                    "template": "{input}",
                                    "template_format": "FmtString",
                                    "input_variables": ["input"]
                                }
                            ]
                        },
                        {
                            "type": "RolePromptTemplate",
                            "value": [
                                "Ai",
                                {
                                    "template": "{output}",
                                    "template_format": "FmtString",
                                    "input_variables": ["output"]
                                }
                            ]
                        }
                    ]
                }
            }
        });

        let actual_json: serde_json::Value =
            serde_json::from_str(&serialized).expect("Failed to parse actual JSON");

        assert_eq!(actual_json, expected_json);
    }

    #[test]
    fn test_deserialize_few_shot_prompt() {
        let json_data = serde_json::json!({
            "type": "FewShotPrompt",
            "value": {
                "examples": {
                    "examples": [
                        {
                            "template": "{input}: What is 2 + 2?\n{output}: 4",
                            "template_format": "FmtString",
                            "input_variables": ["input", "output"]
                        },
                        {
                            "template": "{input}: What is 2 + 3?\n{output}: 5",
                            "template_format": "FmtString",
                            "input_variables": ["input", "output"]
                        }
                    ],
                    "example_separator": "\n\n"
                },
                "example_prompt": {
                    "messages": [
                        {
                            "type": "RolePromptTemplate",
                            "value": [
                                "Human",
                                {
                                    "template": "{input}",
                                    "template_format": "FmtString",
                                    "input_variables": ["input"]
                                }
                            ]
                        },
                        {
                            "type": "RolePromptTemplate",
                            "value": [
                                "Ai",
                                {
                                    "template": "{output}",
                                    "template_format": "FmtString",
                                    "input_variables": ["output"]
                                }
                            ]
                        }
                    ]
                }
            }
        });

        let json_str =
            serde_json::to_string_pretty(&json_data).expect("Failed to serialize test JSON");

        let deserialized: MessageLike =
            serde_json::from_str(&json_str).expect("Failed to deserialize MessageLike");

        if let MessageLike::FewShotPrompt(few_shot_chat_template) = deserialized {
            assert_eq!(few_shot_chat_template.examples().len(), 2);
            assert_eq!(
                few_shot_chat_template
                    .examples()
                    .first()
                    .unwrap()
                    .template(),
                "{input}: What is 2 + 2?\n{output}: 4"
            );
            assert_eq!(
                few_shot_chat_template.examples().get(1).unwrap().template(),
                "{input}: What is 2 + 3?\n{output}: 5"
            );
            assert_eq!(few_shot_chat_template.example_separator(), "\n\n");

            let example_prompt = few_shot_chat_template.example_prompt();
            assert_eq!(example_prompt.messages.len(), 2);

            if let MessageLike::RolePromptTemplate(role, template) = &example_prompt.messages[0] {
                assert_eq!(*role, Role::Human);
                assert_eq!(template.template(), "{input}");
                assert_eq!(template.input_variables(), vec!["input".to_string()]);
            } else {
                panic!("Expected RolePromptTemplate for Human");
            }

            if let MessageLike::RolePromptTemplate(role, template) = &example_prompt.messages[1] {
                assert_eq!(*role, Role::Ai);
                assert_eq!(template.template(), "{output}");
                assert_eq!(template.input_variables(), vec!["output".to_string()]);
            } else {
                panic!("Expected RolePromptTemplate for Ai");
            }
        } else {
            panic!("Expected MessageLike::FewShotPrompt variant");
        }
    }

    #[test]
    fn test_try_from_base_message() {
        let base_message = AiMessage::new("I am an AI.").into();
        let message_like = MessageLike::base_message(base_message);
        let serialized = serde_json::to_string(&message_like).unwrap();

        let deserialized: MessageLike = MessageLike::try_from(serialized).unwrap();
        if let MessageLike::BaseMessage(msg_enum) = deserialized {
            assert_eq!(msg_enum.content(), "I am an AI.");
            assert_eq!(msg_enum.message_type(), &MessageType::Ai);
        } else {
            panic!("Expected BaseMessage");
        }
    }

    #[test]
    fn test_try_from_role_prompt_template() {
        let template = Template::new("Hello, {name}!").unwrap();
        let message_like = MessageLike::role_prompt_template(Role::Human, template.clone());
        let serialized = serde_json::to_string(&message_like).unwrap();

        let deserialized: MessageLike = MessageLike::try_from(serialized).unwrap();
        if let MessageLike::RolePromptTemplate(role, tmpl) = deserialized {
            assert_eq!(role, Role::Human);
            assert_eq!(tmpl.template(), template.template());
        } else {
            panic!("Expected RolePromptTemplate");
        }
    }

    #[test]
    fn test_try_from_placeholder() {
        let placeholder = MessagesPlaceholder::new("history".to_string());
        let message_like = MessageLike::placeholder(placeholder.clone());
        let serialized = serde_json::to_string(&message_like).unwrap();

        let deserialized: MessageLike = MessageLike::try_from(serialized).unwrap();
        if let MessageLike::Placeholder(placeholder_msg) = deserialized {
            assert_eq!(placeholder_msg.variable_name(), "history");
        } else {
            panic!("Expected Placeholder");
        }
    }

    #[test]
    fn test_try_from_few_shot_prompt() {
        let examples = examples!(
            ("{input}: What is 2+2?", "{output}: 4"),
            ("{input}: What is 2+3?", "{output}: 5")
        );

        let example_prompt =
            ChatTemplate::from_messages(chats!(Human = "{input}", Ai = "{output}")).unwrap();
        let few_shot_template = FewShotTemplate::new(examples);
        let few_shot_chat_template = FewShotChatTemplate::new(few_shot_template, example_prompt);

        let message_like = MessageLike::few_shot_prompt(few_shot_chat_template.clone());
        let serialized = serde_json::to_string(&message_like).unwrap();

        let deserialized: MessageLike = MessageLike::try_from(serialized).unwrap();
        if let MessageLike::FewShotPrompt(few_shot_prompt_msg) = deserialized {
            assert_eq!(
                few_shot_prompt_msg.format_examples().unwrap(),
                few_shot_chat_template.format_examples().unwrap()
            );
        } else {
            panic!("Expected FewShotPrompt");
        }
    }
}
