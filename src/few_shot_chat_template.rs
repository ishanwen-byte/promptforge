use std::collections::HashMap;

use crate::{ChatTemplate, FewShotTemplate, Formattable, Template, TemplateError};

#[derive(Debug, Clone)]
pub struct FewShotChatTemplate {
    examples: FewShotTemplate<Template>,
    example_prompt: ChatTemplate,
}

impl FewShotChatTemplate {
    pub fn new(examples: FewShotTemplate<Template>, example_prompt: ChatTemplate) -> Self {
        FewShotChatTemplate {
            examples,
            example_prompt,
        }
    }

    pub fn format_examples(&self) -> Result<String, TemplateError> {
        let variables = self.example_prompt.to_variables_map();
        self.format(&variables)
    }
}

impl Formattable for FewShotChatTemplate {
    fn format(&self, variables: &HashMap<&str, &str>) -> Result<String, TemplateError> {
        let examples = self.examples.format(variables)?;
        Ok(examples)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        chats, examples, ChatTemplate,
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
        let expected_output = "### Examples:\n\nhuman: What is 2 + 2?\nai: 4\n\nhuman: What is 2 + 3?\nai: 5\n\nhuman: What is 3 + 3?\nai: 6\n\n---";
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
        let expected_output = "human: What is 5 + 5?\nai: 10\n\nhuman: What is 6 + 6?\nai: 12";
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
}
