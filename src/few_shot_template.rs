use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::template_format::TemplateError;
use crate::{Formattable, Templatable, Template};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FewShotTemplate<T: Templatable + Formattable> {
    examples: Vec<T>,
    example_separator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<T>,
}

impl<T> Default for FewShotTemplate<T>
where
    T: Templatable + Formattable + DeserializeOwned + TryFrom<String, Error = TemplateError>,
{
    fn default() -> Self {
        Self {
            examples: Vec::new(),
            example_separator: Self::DEFAULT_EXAMPLE_SEPARATOR.to_string(),
            prefix: None,
            suffix: None,
        }
    }
}

impl<T> FewShotTemplate<T>
where
    T: Templatable + Formattable + DeserializeOwned + TryFrom<String, Error = TemplateError>,
{
    pub const DEFAULT_EXAMPLE_SEPARATOR: &'static str = "\n\n";

    pub fn new(examples: Vec<T>) -> Self {
        Self {
            examples,
            ..Default::default()
        }
    }

    pub fn with_options(
        examples: Vec<T>,
        prefix: Option<T>,
        suffix: Option<T>,
        example_separator: impl Into<String>,
    ) -> Self {
        FewShotTemplate {
            examples,
            example_separator: example_separator.into(),
            prefix,
            suffix,
        }
    }

    pub fn builder() -> FewShotTemplateBuilder<T> {
        FewShotTemplateBuilder::new()
    }

    pub fn examples(&self) -> &[T] {
        &self.examples
    }

    pub fn example_separator(&self) -> &str {
        &self.example_separator
    }

    pub fn prefix(&self) -> Option<&T> {
        self.prefix.as_ref()
    }

    pub fn suffix(&self) -> Option<&T> {
        self.suffix.as_ref()
    }

    pub async fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, TemplateError> {
        let toml_content = fs::read_to_string(path).await.map_err(|e| {
            TemplateError::TomlDeserializationError(format!("Failed to read TOML file: {}", e))
        })?;

        FewShotTemplate::try_from(toml_content)
    }
}

impl Formattable for FewShotTemplate<Template> {
    fn format(&self, variables: &HashMap<&str, &str>) -> Result<String, TemplateError> {
        let prefix_str = if let Some(ref prefix_template) = self.prefix {
            prefix_template.format(variables)?
        } else {
            String::new()
        };

        let mut formatted_examples = Vec::new();

        for example in &self.examples {
            let formatted_example = example.format(variables)?;
            formatted_examples.push(formatted_example);
        }

        let examples_str = formatted_examples.join(&self.example_separator);

        let suffix_str = if let Some(ref suffix_template) = self.suffix {
            suffix_template.format(variables)?
        } else {
            String::new()
        };

        let mut result_parts = Vec::new();

        if !prefix_str.is_empty() {
            result_parts.push(prefix_str);
        }
        if !examples_str.is_empty() {
            result_parts.push(examples_str);
        }
        if !suffix_str.is_empty() {
            result_parts.push(suffix_str);
        }

        let result = result_parts.join(&self.example_separator);

        Ok(result)
    }
}

#[derive(Debug)]
pub struct FewShotTemplateBuilder<T>
where
    T: Templatable + Formattable,
{
    examples: Vec<T>,
    example_separator: String,
    prefix: Option<T>,
    suffix: Option<T>,
}

impl<T> Default for FewShotTemplateBuilder<T>
where
    T: Templatable + Formattable + DeserializeOwned + TryFrom<String, Error = TemplateError>,
{
    fn default() -> Self {
        Self {
            prefix: None,
            suffix: None,
            example_separator: FewShotTemplate::<T>::DEFAULT_EXAMPLE_SEPARATOR.to_string(),
            examples: Vec::new(),
        }
    }
}

impl<T> FewShotTemplateBuilder<T>
where
    T: Templatable + Formattable + DeserializeOwned + TryFrom<String, Error = TemplateError>,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prefix(mut self, prefix: T) -> Self {
        self.prefix = Some(prefix);
        self
    }

    pub fn suffix(mut self, suffix: T) -> Self {
        self.suffix = Some(suffix);
        self
    }

    pub fn example_separator(mut self, example_separator: impl Into<String>) -> Self {
        self.example_separator = example_separator.into();
        self
    }

    pub fn example(mut self, example: T) -> Self {
        self.examples.push(example);
        self
    }

    pub fn examples<I>(mut self, examples: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        self.examples.extend(examples);
        self
    }

    pub fn build(self) -> FewShotTemplate<T> {
        FewShotTemplate {
            examples: self.examples,
            example_separator: self.example_separator,
            prefix: self.prefix,
            suffix: self.suffix,
        }
    }
}

impl<T> TryFrom<String> for FewShotTemplate<T>
where
    T: Templatable + Formattable + DeserializeOwned,
{
    type Error = TemplateError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().starts_with('{') {
            serde_json::from_str(&value).map_err(|e| {
                TemplateError::MalformedTemplate(format!("JSON deserialization error: {}", e))
            })
        } else {
            toml::from_str(&value).map_err(|e| {
                TemplateError::MalformedTemplate(format!("TOML deserialization error: {}", e))
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template_format::TemplateError;
    use crate::vars;
    use crate::Template;

    #[test]
    fn test_few_shot_template_with_prefix_suffix_and_examples() {
        let prefix_template = Template::new("This is the prefix. Topic: {topic}").unwrap();
        let example_template1 = Template::new("Q: {question1}\nA: {answer1}").unwrap();
        let example_template2 = Template::new("Q: {question2}\nA: {answer2}").unwrap();
        let suffix_template =
            Template::new("This is the suffix. Remember to think about {topic}.").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .example(example_template1)
            .example(example_template2)
            .suffix(suffix_template)
            .example_separator("\n---\n")
            .build();

        let variables = &vars!(
            topic = "Science",
            question1 = "What is the speed of light?",
            answer1 = "Approximately 299,792 kilometers per second.",
            question2 = "What is the gravitational constant?",
            answer2 = "Approximately 6.674×10^-11 N·(m/kg)^2.",
        );

        let formatted_output = few_shot_template.format(variables).unwrap();
        let expected_output = "\
This is the prefix. Topic: Science
---
Q: What is the speed of light?
A: Approximately 299,792 kilometers per second.
---
Q: What is the gravitational constant?
A: Approximately 6.674×10^-11 N·(m/kg)^2.
---
This is the suffix. Remember to think about Science.";

        assert_eq!(formatted_output, expected_output);
    }

    #[test]
    fn test_few_shot_template_without_prefix_and_suffix() {
        let example_template1 = Template::new("First example with {variable}.").unwrap();
        let example_template2 = Template::new("Second example with {variable}.").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .example(example_template1)
            .example(example_template2)
            .example_separator("\n***\n")
            .build();

        let variables = &vars!(variable = "test data",);
        let formatted_output = few_shot_template.format(variables).unwrap();
        let expected_output = "\
First example with test data.
***
Second example with test data.";

        assert_eq!(formatted_output, expected_output);
    }

    #[test]
    fn test_few_shot_template_with_empty_examples() {
        let prefix_template = Template::new("This is the prefix.").unwrap();
        let suffix_template = Template::new("This is the suffix.").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .suffix(suffix_template)
            .build();

        let variables = &vars!();
        let formatted_output = few_shot_template.format(variables).unwrap();

        let expected_output = "\
This is the prefix.

This is the suffix.";

        assert_eq!(formatted_output, expected_output);
    }

    #[test]
    fn test_few_shot_template_with_missing_variables() {
        let prefix_template = Template::new("Prefix with {var1}").unwrap();
        let example_template = Template::new("Example with {var2}").unwrap();
        let suffix_template = Template::new("Suffix with {var3}").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .example(example_template)
            .suffix(suffix_template)
            .build();

        let variables = &vars!(
            var1 = "value1",
            // var2 is missing
            var3 = "value3",
        );

        let result = few_shot_template.format(variables);

        // Expect an error due to missing 'var2'
        assert!(result.is_err());
        if let Err(TemplateError::MissingVariable(msg)) = result {
            assert!(msg.contains("var2"));
        } else {
            panic!("Expected MissingVariable error");
        }
    }

    #[test]
    fn test_few_shot_template_with_partial_variables() {
        let prefix_template = Template::new("Welcome, {user}!").unwrap();
        let example_template = Template::new("Your role is {role}.").unwrap();
        let suffix_template = Template::new("Goodbye, {user}.").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .example(example_template)
            .suffix(suffix_template)
            .build();

        let variables = &vars!(user = "Alice",);
        let result = few_shot_template.format(variables);

        assert!(result.is_err());
        if let Err(TemplateError::MissingVariable(msg)) = result {
            assert!(msg.contains("role"));
        } else {
            panic!("Expected MissingVariable error");
        }
    }

    #[test]
    fn test_few_shot_template_with_custom_example_separator() {
        let prefix_template = Template::new("Start").unwrap();
        let example_template1 = Template::new("Example One").unwrap();
        let example_template2 = Template::new("Example Two").unwrap();
        let suffix_template = Template::new("End").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .example(example_template1)
            .example(example_template2)
            .suffix(suffix_template)
            .example_separator("\n===\n")
            .build();

        let variables = &vars!();
        let formatted_output = few_shot_template.format(variables).unwrap();

        let expected_output = "\
Start
===
Example One
===
Example Two
===
End";

        assert_eq!(formatted_output, expected_output);
    }

    #[test]
    fn test_few_shot_template_with_plain_text_templates() {
        let prefix_template = Template::new("Plain prefix").unwrap();
        let example_template = Template::new("Plain example").unwrap();
        let suffix_template = Template::new("Plain suffix").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .example(example_template)
            .suffix(suffix_template)
            .build();

        let variables = &vars!();
        let formatted_output = few_shot_template.format(variables).unwrap();

        let expected_output = "\
Plain prefix

Plain example

Plain suffix";

        assert_eq!(formatted_output, expected_output);
    }

    #[test]
    fn test_few_shot_template_with_multiple_examples() {
        let prefix_template = Template::new("Examples Start").unwrap();
        let mut examples = Vec::new();

        for i in 1..=5 {
            let tmpl_str = format!("Example number {}", i);
            let tmpl = Template::new(&tmpl_str).unwrap();
            examples.push(tmpl);
        }

        let suffix_template = Template::new("Examples End").unwrap();
        let mut builder = FewShotTemplate::builder();
        builder = builder.prefix(prefix_template);
        for example in examples {
            builder = builder.example(example);
        }
        builder = builder.suffix(suffix_template);
        let few_shot_template = builder.build();
        let variables = &vars!();
        let formatted_output = few_shot_template.format(variables).unwrap();

        let expected_output = "\
Examples Start

Example number 1

Example number 2

Example number 3

Example number 4

Example number 5

Examples End";

        assert_eq!(formatted_output, expected_output);
    }

    #[test]
    fn test_few_shot_template_with_repeated_variables() {
        let prefix_template = Template::new("Start {var}").unwrap();
        let example_template = Template::new("Example {var}").unwrap();
        let suffix_template = Template::new("End {var}").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .example(example_template.clone())
            .example(example_template)
            .suffix(suffix_template)
            .build();

        let variables = &vars!(var = "Value",);
        let formatted_output = few_shot_template.format(variables).unwrap();

        let expected_output = "\
Start Value

Example Value

Example Value

End Value";

        assert_eq!(formatted_output, expected_output);
    }

    #[test]
    fn test_few_shot_template_with_no_examples() {
        let prefix_template = Template::new("Only Prefix").unwrap();
        let suffix_template = Template::new("Only Suffix").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .suffix(suffix_template)
            .build();

        let variables = &vars!();
        let formatted_output = few_shot_template.format(variables).unwrap();

        let expected_output = "\
Only Prefix

Only Suffix";

        assert_eq!(formatted_output, expected_output);
    }

    #[test]
    fn test_few_shot_template_langchain_example() {
        use crate::vars;
        use crate::Template;

        let examples = vec![
            vars!(
                question = "Who lived longer, Muhammad Ali or Alan Turing?",
                answer = r#"Are follow up questions needed here: Yes.
Follow up: How old was Muhammad Ali when he died?
Intermediate answer: Muhammad Ali was 74 years old when he died.
Follow up: How old was Alan Turing when he died?
Intermediate answer: Alan Turing was 41 years old when he died.
So the final answer is: Muhammad Ali
"#
            ),
            vars!(
                question = "When was the founder of craigslist born?",
                answer = r#"Are follow up questions needed here: Yes.
Follow up: Who was the founder of craigslist?
Intermediate answer: Craigslist was founded by Craig Newmark.
Follow up: When was Craig Newmark born?
Intermediate answer: Craig Newmark was born on December 6, 1952.
So the final answer is: December 6, 1952
"#
            ),
            vars!(
                question = "Who was the maternal grandfather of George Washington?",
                answer = r#"Are follow up questions needed here: Yes.
Follow up: Who was the mother of George Washington?
Intermediate answer: The mother of George Washington was Mary Ball Washington.
Follow up: Who was the father of Mary Ball Washington?
Intermediate answer: The father of Mary Ball Washington was Joseph Ball.
So the final answer is: Joseph Ball
"#
            ),
            vars!(
                question =
                    "Are both the directors of Jaws and Casino Royale from the same country?",
                answer = r#"Are follow up questions needed here: Yes.
Follow up: Who is the director of Jaws?
Intermediate Answer: The director of Jaws is Steven Spielberg.
Follow up: Where is Steven Spielberg from?
Intermediate Answer: The United States.
Follow up: Who is the director of Casino Royale?
Intermediate Answer: The director of Casino Royale is Martin Campbell.
Follow up: Where is Martin Campbell from?
Intermediate Answer: New Zealand.
So the final answer is: No
"#
            ),
        ];

        let example_template_str = r#"Question: {question}

{answer}"#;

        let example_template = Template::new(example_template_str).unwrap();

        let mut formatted_examples = Vec::new();

        for example_vars in &examples {
            let formatted_example = example_template.format(example_vars).unwrap();
            let example = Template::new(&formatted_example).unwrap();
            formatted_examples.push(example);
        }

        let suffix_template_str = r#"Question: {input}"#;

        let suffix_template = Template::new(suffix_template_str).unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .examples(formatted_examples)
            .suffix(suffix_template)
            .example_separator("\n\n")
            .build();

        let variables = &vars!(input = "Who was the father of Mary Ball Washington?");

        let formatted_output = few_shot_template.format(variables).unwrap();

        let expected_output = r#"
Question: Who lived longer, Muhammad Ali or Alan Turing?

Are follow up questions needed here: Yes.
Follow up: How old was Muhammad Ali when he died?
Intermediate answer: Muhammad Ali was 74 years old when he died.
Follow up: How old was Alan Turing when he died?
Intermediate answer: Alan Turing was 41 years old when he died.
So the final answer is: Muhammad Ali


Question: When was the founder of craigslist born?

Are follow up questions needed here: Yes.
Follow up: Who was the founder of craigslist?
Intermediate answer: Craigslist was founded by Craig Newmark.
Follow up: When was Craig Newmark born?
Intermediate answer: Craig Newmark was born on December 6, 1952.
So the final answer is: December 6, 1952


Question: Who was the maternal grandfather of George Washington?

Are follow up questions needed here: Yes.
Follow up: Who was the mother of George Washington?
Intermediate answer: The mother of George Washington was Mary Ball Washington.
Follow up: Who was the father of Mary Ball Washington?
Intermediate answer: The father of Mary Ball Washington was Joseph Ball.
So the final answer is: Joseph Ball


Question: Are both the directors of Jaws and Casino Royale from the same country?

Are follow up questions needed here: Yes.
Follow up: Who is the director of Jaws?
Intermediate Answer: The director of Jaws is Steven Spielberg.
Follow up: Where is Steven Spielberg from?
Intermediate Answer: The United States.
Follow up: Who is the director of Casino Royale?
Intermediate Answer: The director of Casino Royale is Martin Campbell.
Follow up: Where is Martin Campbell from?
Intermediate Answer: New Zealand.
So the final answer is: No


Question: Who was the father of Mary Ball Washington?
"#;

        let formatted_output_trimmed = formatted_output.trim();
        let expected_output_trimmed = expected_output.trim();

        assert_eq!(formatted_output_trimmed, expected_output_trimmed);
    }

    #[test]
    fn test_serialize_few_shot_template() {
        let prefix_template = Template::new("This is the prefix. Topic: {topic}").unwrap();
        let example_template1 = Template::new("Q: {question1}\nA: {answer1}").unwrap();
        let example_template2 = Template::new("Q: {question2}\nA: {answer2}").unwrap();
        let suffix_template =
            Template::new("This is the suffix. Remember to think about {topic}.").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .example(example_template1)
            .example(example_template2)
            .suffix(suffix_template)
            .example_separator("\n---\n")
            .build();

        let serialized = serde_json::to_string(&few_shot_template).expect("Serialization failed");

        assert!(serialized.contains("This is the prefix"));
        assert!(serialized.contains("Q: {question1}"));
        assert!(serialized.contains("This is the suffix"));

        println!("Serialized FewShotTemplate: {}", serialized);
    }

    #[test]
    fn test_deserialize_few_shot_template() {
        let json_data = r#"
    {
        "examples": [
            { 
                "template": "Q: {question1}\nA: {answer1}",
                "template_format": "FmtString",
                "input_variables": ["question1", "answer1"]
            },
            { 
                "template": "Q: {question2}\nA: {answer2}",
                "template_format": "FmtString",
                "input_variables": ["question2", "answer2"]
            }
        ],
        "example_separator": "\n---\n",
        "prefix": { 
            "template": "This is the prefix. Topic: {topic}",
            "template_format": "FmtString",
            "input_variables": ["topic"]
        },
        "suffix": { 
            "template": "This is the suffix. Remember to think about {topic}.",
            "template_format": "FmtString",
            "input_variables": ["topic"]
        }
    }
    "#;

        let deserialized: FewShotTemplate<Template> =
            serde_json::from_str(json_data).expect("Deserialization failed");

        assert_eq!(deserialized.examples.len(), 2);
        assert_eq!(deserialized.example_separator, "\n---\n");

        assert!(deserialized.prefix.is_some());
        assert!(deserialized.suffix.is_some());

        if let Some(prefix) = &deserialized.prefix {
            assert_eq!(prefix.template(), "This is the prefix. Topic: {topic}");
        }

        if let Some(suffix) = &deserialized.suffix {
            assert_eq!(
                suffix.template(),
                "This is the suffix. Remember to think about {topic}."
            );
        }
    }

    #[test]
    fn test_serialize_deserialize_few_shot_template() {
        let prefix_template = Template::new("Prefix {var}").unwrap();
        let example_template = Template::new("Example {var}").unwrap();
        let suffix_template = Template::new("Suffix {var}").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template.clone())
            .example(example_template.clone())
            .suffix(suffix_template.clone())
            .build();

        let serialized = serde_json::to_string(&few_shot_template).expect("Serialization failed");

        let deserialized: FewShotTemplate<Template> =
            serde_json::from_str(&serialized).expect("Deserialization failed");

        assert_eq!(deserialized.examples.len(), 1);
        assert_eq!(deserialized.example_separator, "\n\n");

        assert_eq!(
            deserialized.prefix.as_ref().unwrap().template(),
            prefix_template.template()
        );
        assert_eq!(
            deserialized.suffix.as_ref().unwrap().template(),
            suffix_template.template()
        );

        assert_eq!(
            deserialized.examples[0].template(),
            example_template.template()
        );
    }

    #[test]
    fn test_try_from_string_valid() {
        let json_data = r#"
    {
        "examples": [
            { 
                "template": "Q: {question1}\nA: {answer1}",
                "template_format": "FmtString",
                "input_variables": ["question1", "answer1"]
            },
            { 
                "template": "Q: {question2}\nA: {answer2}",
                "template_format": "FmtString",
                "input_variables": ["question2", "answer2"]
            }
        ],
        "example_separator": "\n---\n",
        "prefix": { 
            "template": "This is the prefix. Topic: {topic}",
            "template_format": "FmtString",
            "input_variables": ["topic"]
        },
        "suffix": { 
            "template": "This is the suffix. Remember to think about {topic}.",
            "template_format": "FmtString",
            "input_variables": ["topic"]
        }
    }
    "#;

        let template = FewShotTemplate::<Template>::try_from(json_data.to_string());

        assert!(template.is_ok());

        let template = template.unwrap();
        assert_eq!(template.examples.len(), 2);
        assert!(template.prefix.is_some());
        assert!(template.suffix.is_some());
        assert_eq!(template.example_separator, "\n---\n");
    }

    #[test]
    fn test_try_from_string_invalid() {
        let invalid_json_data = "Invalid JSON string";

        let error =
            FewShotTemplate::<Template>::try_from(invalid_json_data.to_string()).unwrap_err();

        match error {
            TemplateError::MalformedTemplate(msg) => {
                println!("Error message: {}", msg);
            }
            _ => {
                panic!(
                    "Expected TemplateError::MalformedTemplate, but got {:?}",
                    error
                );
            }
        }
    }
}
