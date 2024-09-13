use crate::{
    message_like::MessageLike, PromptTemplate, Role, Template, TemplateError, TemplateFormat,
};
use std::sync::Arc;

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
                    Ok(base_message) => result.push(MessageLike::BaseMessage(base_message)),
                    Err(_) => return Err(TemplateError::InvalidRoleError),
                },
                _ => {
                    result.push(MessageLike::PromptTemplate(Arc::new(prompt_template)));
                }
            }
        }

        Ok(ChatPromptTemplate { messages: result })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat_templates;
    use crate::Role::{Ai, Human, Placeholder, System};
    use crate::{message_like::MessageLike, TemplateError};

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

        if let MessageLike::PromptTemplate(template) = &chat_prompt.messages[0] {
            assert_eq!(
                template.template(),
                "You are a helpful AI bot. Your name is {name}."
            );
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
    fn test_from_messages_invalid_role() {
        let templates = chat_templates!(
            System = "This is a valid system message.",
            Placeholder = "This is an invalid role message.",
        );

        let chat_prompt = ChatPromptTemplate::from_messages(templates);
        assert!(chat_prompt
            .unwrap_err()
            .matches(&TemplateError::InvalidRoleError));
    }
}
