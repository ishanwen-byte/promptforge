use std::{collections::HashMap, fmt, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{ChatTemplate, FewShotTemplate, Formattable, Template, TemplateError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FewShotChatTemplate {
    examples: FewShotTemplate<Template>,
    example_prompt: Arc<ChatTemplate>,
}

impl FewShotChatTemplate {
    pub fn new(examples: FewShotTemplate<Template>, example_prompt: ChatTemplate) -> Self {
        FewShotChatTemplate {
            examples,
            example_prompt: Arc::new(example_prompt),
        }
    }

    pub fn format_examples(&self) -> Result<String, TemplateError> {
        let variables = self.example_prompt.to_variables_map();
        self.format(&variables)
    }

    pub fn examples(&self) -> &[Template] {
        self.examples.examples()
    }

    pub fn example_prompt(&self) -> &ChatTemplate {
        &self.example_prompt
    }

    pub fn example_separator(&self) -> &str {
        self.examples.example_separator()
    }

    pub fn prefix(&self) -> Option<&Template> {
        self.examples.prefix()
    }

    pub fn suffix(&self) -> Option<&Template> {
        self.examples.suffix()
    }
}

impl Formattable for FewShotChatTemplate {
    fn format(&self, variables: &HashMap<&str, &str>) -> Result<String, TemplateError> {
        let examples = self.examples.format(variables)?;
        if examples.is_empty() {
            Ok(String::new())
        } else {
            let formatted_examples = format!("{}\n\n", examples);
            Ok(formatted_examples)
        }
    }
}

impl TryFrom<String> for FewShotChatTemplate {
    type Error = TemplateError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Ok(template) = serde_json::from_str::<FewShotChatTemplate>(&value) {
            return Ok(template);
        }

        let deserialized: HashMap<String, String> = serde_json::from_str(&value).map_err(|e| {
            TemplateError::MalformedTemplate(format!("Failed to parse JSON: {}", e))
        })?;

        let examples_str = deserialized
            .get("examples")
            .ok_or(TemplateError::MalformedTemplate(
                "Missing 'examples' field".to_string(),
            ))?;
        let examples = FewShotTemplate::try_from(examples_str.clone())?;

        let example_prompt_str =
            deserialized
                .get("example_prompt")
                .ok_or(TemplateError::MalformedTemplate(
                    "Missing 'example_prompt' field".to_string(),
                ))?;
        let example_prompt = ChatTemplate::try_from(example_prompt_str.clone())?;

        Ok(FewShotChatTemplate::new(examples, example_prompt))
    }
}

impl fmt::Display for FewShotChatTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json_rep = serde_json::to_string(&self).map_err(|_| fmt::Error)?;
        write!(f, "{}", json_rep)
    }
}

#[cfg(test)]
mod tests {
    use messageforge::BaseMessage;

    use super::*;
    use crate::{
        chats, examples, ChatTemplate, MessageLike,
        Role::{Ai, Human},
    };

    #[test]
    fn test_few_shot_chat_template_format_examples() {
        let examples = examples!(
            ("{input}: What is 2 + 2?", "{output}: 4"),
            ("{input}: What is 2 + 3?", "{output}: 5"),
            ("{input}: What is 3 + 3?", "{output}: 6"),
        );

        let prefix = Template::new("### Examples:").unwrap();
        let suffix = Template::new("---").unwrap();

        let few_shot_builder = FewShotTemplate::<Template>::builder();
        let few_shot_template = few_shot_builder
            .examples(examples)
            .prefix(prefix)
            .suffix(suffix)
            .build();

        let example_prompt =
            ChatTemplate::from_messages(chats!(Human = "{input}", Ai = "{output}")).unwrap();
        let few_shot_chat_template =
            FewShotChatTemplate::new(few_shot_template, example_prompt.clone());

        let formatted_examples = few_shot_chat_template.format_examples().unwrap();
        let expected_output = "### Examples:\n\nhuman: What is 2 + 2?\nai: 4\n\nhuman: What is 2 + 3?\nai: 5\n\nhuman: What is 3 + 3?\nai: 6\n\n---\n\n";
        assert_eq!(formatted_examples, expected_output);

        let examples = examples!(
            ("{question}: What is 5 + 5?", "{answer}: 10"),
            ("{question}: What is 6 + 6?", "{answer}: 12"),
        );

        let few_shot_template = FewShotTemplate::new(examples);
        let example_prompt =
            ChatTemplate::from_messages(chats!(Human = "{question}", Ai = "{answer}")).unwrap();
        let few_shot_chat_template =
            FewShotChatTemplate::new(few_shot_template, example_prompt.clone());

        let formatted_examples = few_shot_chat_template.format_examples().unwrap();
        let expected_output = "human: What is 5 + 5?\nai: 10\n\nhuman: What is 6 + 6?\nai: 12\n\n";
        assert_eq!(formatted_examples, expected_output);
    }

    #[test]
    fn test_few_shot_chat_template_empty_examples() {
        let few_shot_template = FewShotTemplate::new(vec![]);
        let example_prompt =
            ChatTemplate::from_messages(chats!(Human = "{input}", Ai = "{output}")).unwrap();

        let few_shot_chat_template =
            FewShotChatTemplate::new(few_shot_template, example_prompt.clone());

        let formatted_examples = few_shot_chat_template.format_examples().unwrap();
        let expected_output = "";
        assert_eq!(formatted_examples, expected_output);
    }

    #[test]
    fn test_few_shot_chat_template_incorrect_variables() {
        let examples = examples!(
            ("{input}: What is 2 + 2?", "{output}: 4"),
            ("{input}: What is 2 + 3?", "{output}: 5"),
            ("{input}: What is 3 + 3?", "{output}: 6"),
        );

        let incorrect_example_prompt =
            ChatTemplate::from_messages(chats!(Human = "{question}", Ai = "{answer}")).unwrap();

        let few_shot_template = FewShotTemplate::new(examples);
        let few_shot_chat_template =
            FewShotChatTemplate::new(few_shot_template, incorrect_example_prompt.clone());

        let format_result = few_shot_chat_template.format_examples();
        assert!(matches!(
            format_result,
            Err(TemplateError::MissingVariable(_))
        ));
    }

    #[test]
    fn test_try_from_valid_string_few_shot_chat_template() {
        let json_data = r#"
    {
        "examples": "{\"examples\":[{\"template\":\"{question}: What is 5 + 5?\\n{answer}: 10\",\"template_format\":\"FmtString\",\"input_variables\":[\"question\",\"answer\"]},{\"template\":\"{question}: What is 6 + 6?\\n{answer}: 12\",\"template_format\":\"FmtString\",\"input_variables\":[\"question\",\"answer\"]}],\"example_separator\":\"\\n\\n\"}",
        "example_prompt": "{\"messages\":[{\"type\":\"BaseMessage\",\"value\":{\"role\":\"human\",\"content\":\"{question}\"}},{\"type\":\"BaseMessage\",\"value\":{\"role\":\"ai\",\"content\":\"{answer}\"}}]}"
    }
    "#;

        let result = FewShotChatTemplate::try_from(json_data.to_string());
        assert!(result.is_ok());
        let few_shot_chat_template = result.unwrap();
        let formatted_examples = few_shot_chat_template.format_examples().unwrap();
        let expected_output = "human: What is 5 + 5?\nai: 10\n\nhuman: What is 6 + 6?\nai: 12\n\n";
        assert_eq!(formatted_examples, expected_output);
    }

    #[test]
    fn test_try_from_invalid_string_few_shot_chat_template() {
        let invalid_json_data = r#"{
            "examples": "{\"examples\":[{\"template\":\"{input}: What is 2 + 2?\",\"template_format\":\"FmtString\",\"input_variables\":[\"input\"]}]}",
            "example_prompt": "invalid_json"
        }"#;

        let result = FewShotChatTemplate::try_from(invalid_json_data.to_string());
        assert!(result.is_err());

        if let Err(TemplateError::MalformedTemplate(msg)) = result {
            println!("{}", msg);
        } else {
            panic!("Expected TemplateError::MalformedTemplate");
        }
    }

    #[test]
    fn test_try_from_missing_fields() {
        let missing_fields_json = r#"
        {
            "example_prompt": "{\"messages\":[{\"BaseMessage\":{\"role\":\"human\",\"content\":\"{input}\"}},{\"BaseMessage\":{\"role\":\"ai\",\"content\":\"{output}\"}}]}"
        }
        "#;

        let result = FewShotChatTemplate::try_from(missing_fields_json.to_string());
        assert!(result.is_err());

        if let Err(TemplateError::MalformedTemplate(msg)) = result {
            assert!(msg.contains("Missing 'examples' field"));
        } else {
            panic!("Expected TemplateError::MalformedTemplate");
        }
    }

    #[test]
    fn test_try_from_empty_string_few_shot_chat_template() {
        let empty_json_data = "{}";

        let result = FewShotChatTemplate::try_from(empty_json_data.to_string());
        assert!(result.is_err());

        if let Err(TemplateError::MalformedTemplate(msg)) = result {
            assert!(msg.contains("Missing 'examples' field"));
        } else {
            panic!("Expected TemplateError::MalformedTemplate");
        }
    }

    #[test]
    fn test_try_from_string_invalid_json() {
        let invalid_json_data = r#"{
            "messages": [
                {"role": "human", "content": "What is 2 + 2?"}
            "#; // Invalid JSON (unclosed array and object)

        let result = ChatTemplate::try_from(invalid_json_data.to_string());
        assert!(result.is_err());

        if let Err(TemplateError::MalformedTemplate(msg)) = result {
            assert!(msg.contains("Failed to parse JSON"));
        } else {
            panic!("Expected TemplateError::MalformedTemplate");
        }
    }

    #[test]
    fn test_try_from_string_missing_field() {
        let json_data_missing_field = r#"
        {
            "some_other_field": "value"
        }
        "#;

        let result = ChatTemplate::try_from(json_data_missing_field.to_string());
        assert!(result.is_err());

        if let Err(TemplateError::MalformedTemplate(msg)) = result {
            assert!(msg.contains("missing field"));
        } else {
            panic!("Expected TemplateError::MalformedTemplate");
        }
    }

    #[test]
    fn test_try_from_string_empty_json() {
        let empty_json_data = "{}";

        let result = ChatTemplate::try_from(empty_json_data.to_string());
        assert!(result.is_err());

        if let Err(TemplateError::MalformedTemplate(msg)) = result {
            assert!(msg.contains("missing field"));
        } else {
            panic!("Expected TemplateError::MalformedTemplate");
        }
    }

    #[test]
    fn test_try_from_string_valid_chat_template() {
        let json_data = r#"
        {
            "messages": [
                {
                    "type": "BaseMessage",
                    "value": {
                        "role": "human",
                        "content": "What is 2 + 2?"
                    }
                },
                {
                    "type": "BaseMessage",
                    "value": {
                        "role": "ai",
                        "content": "4"
                    }
                }
            ]
        }
        "#;

        let result = ChatTemplate::try_from(json_data.to_string());
        assert!(result.is_ok());
        let chat_template = result.unwrap();

        assert_eq!(chat_template.messages.len(), 2);
        if let MessageLike::BaseMessage(human_message) = &chat_template.messages[0] {
            assert_eq!(human_message.content(), "What is 2 + 2?");
        } else {
            panic!("Expected a BaseMessage for the human message.");
        }

        if let MessageLike::BaseMessage(ai_message) = &chat_template.messages[1] {
            assert_eq!(ai_message.content(), "4");
        } else {
            panic!("Expected a BaseMessage for the AI message.");
        }
    }

    #[tokio::test]
    async fn test_format_few_shot_chat_template() {
        let examples = examples!(
            ("{input}: What is 2+2?", "{output}: 4"),
            ("{input}: What is 2+3?", "{output}: 5")
        );

        let few_shot_template = FewShotTemplate::new(examples);
        let example_prompt =
            ChatTemplate::from_messages(chats!(Human = "{input}", Ai = "{output}")).unwrap();

        let few_shot_chat_template = FewShotChatTemplate::new(few_shot_template, example_prompt);
        let formatted_output = few_shot_chat_template.format_examples().unwrap();
        let expected_output = "\
human: What is 2+2?
ai: 4

human: What is 2+3?
ai: 5

";

        assert_eq!(formatted_output, expected_output);
    }
}
