use std::{collections::HashMap, sync::Arc};

use messageforge::{BaseMessage, MessageEnum};

use crate::{
    extract_placeholder_variable, message_like::MessageLike, PromptTemplate, Role, Template,
    TemplateError, TemplateFormat,
};

#[derive(Debug, Clone)]
pub struct ChatPromptTemplate {
    pub messages: Vec<MessageLike>,
}

impl ChatPromptTemplate {
    pub fn from_messages(messages: &[(Role, &str)]) -> Result<Self, TemplateError> {
        let mut result = Vec::new();

        for (role, tmpl) in messages {
            let prompt_template = PromptTemplate::from_template(tmpl)?;

            match prompt_template.template_format() {
                TemplateFormat::PlainText => match role.to_message(tmpl) {
                    Ok(base_message) => result.push(MessageLike::from_base_message(base_message)),
                    Err(_) => return Err(TemplateError::InvalidRoleError),
                },
                _ => {
                    result.push(MessageLike::from_role_prompt_template(
                        role,
                        prompt_template,
                    ));
                }
            }
        }

        Ok(ChatPromptTemplate { messages: result })
    }

    pub fn invoke(
        &self,
        variables: &HashMap<&str, &str>,
    ) -> Result<Vec<Arc<dyn BaseMessage>>, TemplateError> {
        let mut result = Vec::new();

        for message_like in &self.messages {
            match message_like {
                MessageLike::BaseMessage(base_message) => {
                    result.push(base_message.clone());
                }

                MessageLike::RolePromptTemplate(role, template) => {
                    if *role == Role::Placeholder {
                        let placeholder_var = extract_placeholder_variable(template.template())?;

                        if let Some(history) = variables.get(placeholder_var.as_str()) {
                            let deserialized_messages: Vec<MessageEnum> =
                                serde_json::from_str(history).map_err(|e| {
                                    TemplateError::MalformedTemplate(format!(
                                        "Failed to deserialize placeholder: {}",
                                        e
                                    ))
                                })?;

                            for message_enum in deserialized_messages {
                                result.push(Arc::new(message_enum) as Arc<dyn BaseMessage>);
                            }
                        } else {
                            continue;
                        }
                    } else {
                        let formatted_message = template.format(variables.clone())?;
                        let base_message = role.to_message(&formatted_message)?;
                        result.push(Arc::from(base_message));
                    }
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat_templates;
    use crate::message_like::MessageLike;
    use crate::Role::{Ai, Human, Placeholder, System};

    #[test]
    fn test_from_messages_plaintext() {
        let templates = chat_templates!(
            System = "This is a system message.",
            Human = "Hello, human!",
        );

        let chat_prompt = ChatPromptTemplate::from_messages(templates);
        let chat_prompt = chat_prompt.unwrap();
        assert_eq!(chat_prompt.messages.len(), 2);

        if let MessageLike::BaseMessage(message) = &chat_prompt.messages[0] {
            assert_eq!(message.content(), "This is a system message.");
        } else {
            panic!("Expected a BaseMessage for the system message.");
        }

        if let MessageLike::BaseMessage(message) = &chat_prompt.messages[1] {
            assert_eq!(message.content(), "Hello, human!");
        } else {
            panic!("Expected a BaseMessage for the human message.");
        }
    }

    #[test]
    fn test_from_messages_formatted_template() {
        let templates = chat_templates!(
            System = "You are a helpful AI bot. Your name is {name}.",
            Ai = "I'm doing well, thank you.",
        );

        let chat_prompt = ChatPromptTemplate::from_messages(templates);
        let chat_prompt = chat_prompt.unwrap();
        assert_eq!(chat_prompt.messages.len(), 2);

        if let MessageLike::RolePromptTemplate(role, template) = &chat_prompt.messages[0] {
            assert_eq!(
                template.template(),
                "You are a helpful AI bot. Your name is {name}."
            );
            assert_eq!(role, &System);
        } else {
            panic!("Expected a PromptTemplate for the system message.");
        }

        if let MessageLike::BaseMessage(message) = &chat_prompt.messages[1] {
            assert_eq!(message.content(), "I'm doing well, thank you.");
        } else {
            panic!("Expected a BaseMessage for the AI message.");
        }
    }

    #[test]
    fn test_from_messages_placeholder() {
        let templates = chat_templates!(
            System = "This is a valid system message.",
            Placeholder = "{history}",
        );

        let chat_prompt = ChatPromptTemplate::from_messages(templates).unwrap();
        assert_eq!(chat_prompt.messages.len(), 2);

        if let MessageLike::BaseMessage(system_message) = &chat_prompt.messages[0] {
            assert_eq!(system_message.content(), "This is a valid system message.");
        } else {
            panic!("Expected BaseMessage for the system role.");
        }

        if let MessageLike::RolePromptTemplate(role, tmpl) = &chat_prompt.messages[1] {
            assert_eq!(*role, Role::Placeholder);
            assert_eq!(tmpl.template(), "{history}");
        } else {
            panic!("Expected RolePromptTemplate for the placeholder role.");
        }
    }
}
