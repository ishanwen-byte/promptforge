use crate::{extract_variables, Template, TemplateError, TemplateFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FewShotChatTemplateConfig {
    pub example_separator: String,
    pub prefix: TemplateConfig,
    pub suffix: TemplateConfig,
    pub examples: Vec<TemplateConfig>,
    pub messages: Vec<MessageConfig>,
}

#[derive(Debug, Deserialize)]
pub struct TemplateConfig {
    pub template: String,
    pub template_format: String,
    pub input_variables: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct MessageConfig {
    #[serde(rename = "type")]
    pub message_type: String,
    pub value: MessageValue,
}

#[derive(Debug, Deserialize)]
pub struct MessageValue {
    pub role: String,
    pub content: String,
}

impl TryInto<Template> for TemplateConfig {
    type Error = TemplateError;

    fn try_into(self) -> Result<Template, Self::Error> {
        let template_format = TemplateFormat::try_from(self.template_format.as_str())?;

        let input_variables = Some(
            extract_variables(&self.template)
                .into_iter()
                .map(|var| var.to_string())
                .collect::<Vec<String>>(),
        );

        Template::new_with_config(&self.template, Some(template_format), input_variables)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Templatable, TemplateFormat};

    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_few_shot_chat_template_config_deserialization() {
        let toml_str = r#"
        example_separator = "\n---\n"

        [prefix]
        template = "This is the prefix. Topic: {topic}"
        template_format = "FmtString"
        input_variables = ["topic"]

        [suffix]
        template = "This is the suffix. Remember to think about {topic}."
        template_format = "FmtString"
        input_variables = ["topic"]

        [[examples]]
        template = "Q: {question1}\nA: {answer1}"
        template_format = "FmtString"
        input_variables = ["question1", "answer1"]

        [[examples]]
        template = "Q: {question2}\nA: {answer2}"
        template_format = "FmtString"
        input_variables = ["question2", "answer2"]

        [[messages]]
        type = "BaseMessage"
        [messages.value]
        role = "system"
        content = "System initialized."

        [[messages]]
        type = "BaseMessage"
        [messages.value]
        role = "human"
        content = "Hello, AI!"

        [[messages]]
        type = "BaseMessage"
        [messages.value]
        role = "ai"
        content = "Hello, human! How can I assist you today?"
        "#;

        let config: FewShotChatTemplateConfig =
            toml::from_str(toml_str).expect("Failed to parse TOML");

        assert_eq!(config.example_separator, "\n---\n");
        assert_eq!(config.prefix.template, "This is the prefix. Topic: {topic}");
        assert_eq!(config.prefix.template_format, "FmtString");
        assert_eq!(config.prefix.input_variables, vec!["topic"]);

        assert_eq!(
            config.suffix.template,
            "This is the suffix. Remember to think about {topic}."
        );
        assert_eq!(config.suffix.template_format, "FmtString");
        assert_eq!(config.suffix.input_variables, vec!["topic"]);

        assert_eq!(config.examples.len(), 2);
        assert_eq!(config.examples[0].template, "Q: {question1}\nA: {answer1}");
        assert_eq!(config.examples[0].template_format, "FmtString");
        assert_eq!(
            config.examples[0].input_variables,
            vec!["question1", "answer1"]
        );

        assert_eq!(config.examples[1].template, "Q: {question2}\nA: {answer2}");
        assert_eq!(config.examples[1].template_format, "FmtString");
        assert_eq!(
            config.examples[1].input_variables,
            vec!["question2", "answer2"]
        );

        assert_eq!(config.messages.len(), 3);

        assert_eq!(config.messages[0].message_type, "BaseMessage");
        assert_eq!(config.messages[0].value.role, "system");
        assert_eq!(config.messages[0].value.content, "System initialized.");

        assert_eq!(config.messages[1].message_type, "BaseMessage");
        assert_eq!(config.messages[1].value.role, "human");
        assert_eq!(config.messages[1].value.content, "Hello, AI!");

        assert_eq!(config.messages[2].message_type, "BaseMessage");
        assert_eq!(config.messages[2].value.role, "ai");
        assert_eq!(
            config.messages[2].value.content,
            "Hello, human! How can I assist you today?"
        );
    }

    #[test]
    fn test_invalid_toml_deserialization() {
        let invalid_toml_str = r#"
        example_separator = 123  # Invalid type, should be a string
        "#;

        let result: Result<FewShotChatTemplateConfig, toml::de::Error> =
            toml::from_str(invalid_toml_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_into_template_success() {
        let config = TemplateConfig {
            template: "{name} is learning Rust!".to_string(),
            template_format: "FmtString".to_string(),
            input_variables: vec!["name".to_string()],
        };

        let template: Result<Template, TemplateError> = config.try_into();

        assert!(template.is_ok());
        let template = template.unwrap();

        assert_eq!(template.template(), "{name} is learning Rust!");
        assert_eq!(template.template_format(), TemplateFormat::FmtString);
        assert_eq!(template.input_variables(), vec!["name".to_string()]);
    }

    #[test]
    fn test_try_into_template_mustache_format() {
        let config = TemplateConfig {
            template: "Hello, {{name}}!".to_string(),
            template_format: "Mustache".to_string(),
            input_variables: vec!["name".to_string()],
        };

        let template: Result<Template, TemplateError> = config.try_into();

        assert!(template.is_ok());
        let template = template.unwrap();

        assert_eq!(template.template(), "Hello, {{name}}!");
        assert_eq!(template.template_format(), TemplateFormat::Mustache);
        assert_eq!(template.input_variables(), vec!["name".to_string()]);
    }

    #[test]
    fn test_try_into_template_invalid_format() {
        let config = TemplateConfig {
            template: "This format is unsupported: <<<var>>>".to_string(),
            template_format: "UnknownFormat".to_string(),
            input_variables: vec!["var".to_string()],
        };

        let result: Result<Template, TemplateError> = config.try_into();

        assert!(result.is_err());
        if let Err(TemplateError::UnsupportedFormat(msg)) = result {
            assert_eq!(msg, "Unsupported template format");
        } else {
            panic!("Expected UnsupportedFormat error");
        }
    }

    #[test]
    fn test_try_into_template_missing_variable() {
        let config = TemplateConfig {
            template: "This is a test without variables.".to_string(),
            template_format: "PlainText".to_string(),
            input_variables: vec![],
        };

        let template: Result<Template, TemplateError> = config.try_into();

        assert!(template.is_ok());
        let template = template.unwrap();

        assert_eq!(template.template(), "This is a test without variables.");
        assert_eq!(template.template_format(), TemplateFormat::PlainText);
        assert!(template.input_variables().is_empty());
    }

    #[test]
    fn test_try_into_template_detect_format() {
        let config = TemplateConfig {
            template: "Hello, {user}!".to_string(),
            template_format: "FmtString".to_string(),
            input_variables: vec!["user".to_string()],
        };

        let template: Result<Template, TemplateError> = config.try_into();

        assert!(template.is_ok());
        let template = template.unwrap();

        assert_eq!(template.template(), "Hello, {user}!");
        assert_eq!(template.template_format(), TemplateFormat::FmtString);
        assert_eq!(template.input_variables(), vec!["user".to_string()]);
    }
}
