use crate::template_format::TemplateError;
use crate::Templatable;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FewShotTemplate<T: Templatable + Send + Sync> {
    examples: Vec<T>,
    example_separator: String,
    prefix: Option<T>,
    suffix: Option<T>,
}

impl<T: Templatable + Send + Sync> Default for FewShotTemplate<T> {
    fn default() -> Self {
        Self {
            examples: Vec::new(),
            example_separator: Self::DEFAULT_EXAMPLE_SEPARATOR.to_string(),
            prefix: None,
            suffix: None,
        }
    }
}

impl<T: Templatable + Send + Sync> FewShotTemplate<T> {
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

    pub async fn format(&self, variables: HashMap<&str, &str>) -> Result<String, TemplateError> {
        let prefix_str = if let Some(ref prefix_template) = self.prefix {
            prefix_template.format(variables.clone()).await?
        } else {
            String::new()
        };

        let mut formatted_examples = Vec::new();

        for example in &self.examples {
            let formatted_example = example.format(variables.clone()).await?;
            formatted_examples.push(formatted_example);
        }

        let examples_str = formatted_examples.join(&self.example_separator);

        let suffix_str = if let Some(ref suffix_template) = self.suffix {
            suffix_template.format(variables.clone()).await?
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
pub struct FewShotTemplateBuilder<T: Templatable + Send + Sync> {
    examples: Vec<T>,
    example_separator: String,
    prefix: Option<T>,
    suffix: Option<T>,
}

impl<T: Templatable + Send + Sync> Default for FewShotTemplateBuilder<T> {
    fn default() -> Self {
        Self {
            prefix: None,
            suffix: None,
            example_separator: FewShotTemplate::<T>::DEFAULT_EXAMPLE_SEPARATOR.to_string(),
            examples: Vec::new(),
        }
    }
}

impl<T: Templatable + Send + Sync> FewShotTemplateBuilder<T> {
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

    pub fn add_examples<I>(mut self, examples: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        for example in examples {
            self.examples.push(example);
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template_format::TemplateError;
    use crate::vars;
    use crate::Templatable;
    use crate::Template;

    #[tokio::test]
    async fn test_few_shot_template_with_prefix_suffix_and_examples() {
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

        let variables = vars!(
            topic = "Science",
            question1 = "What is the speed of light?",
            answer1 = "Approximately 299,792 kilometers per second.",
            question2 = "What is the gravitational constant?",
            answer2 = "Approximately 6.674×10^-11 N·(m/kg)^2.",
        );

        let formatted_output = few_shot_template.format(variables).await.unwrap();
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

    #[tokio::test]
    async fn test_few_shot_template_without_prefix_and_suffix() {
        let example_template1 = Template::new("First example with {variable}.").unwrap();
        let example_template2 = Template::new("Second example with {variable}.").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .example(example_template1)
            .example(example_template2)
            .example_separator("\n***\n")
            .build();

        let variables = vars!(variable = "test data",);
        let formatted_output = few_shot_template.format(variables).await.unwrap();
        let expected_output = "\
First example with test data.
***
Second example with test data.";

        assert_eq!(formatted_output, expected_output);
    }

    #[tokio::test]
    async fn test_few_shot_template_with_empty_examples() {
        let prefix_template = Template::new("This is the prefix.").unwrap();
        let suffix_template = Template::new("This is the suffix.").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .suffix(suffix_template)
            .build();

        let variables = vars!();
        let formatted_output = few_shot_template.format(variables).await.unwrap();

        let expected_output = "\
This is the prefix.

This is the suffix.";

        assert_eq!(formatted_output, expected_output);
    }

    #[tokio::test]
    async fn test_few_shot_template_with_missing_variables() {
        let prefix_template = Template::new("Prefix with {var1}").unwrap();
        let example_template = Template::new("Example with {var2}").unwrap();
        let suffix_template = Template::new("Suffix with {var3}").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .example(example_template)
            .suffix(suffix_template)
            .build();

        let variables = vars!(
            var1 = "value1",
            // var2 is missing
            var3 = "value3",
        );

        let result = few_shot_template.format(variables).await;

        // Expect an error due to missing 'var2'
        assert!(result.is_err());
        if let Err(TemplateError::MissingVariable(msg)) = result {
            assert!(msg.contains("var2"));
        } else {
            panic!("Expected MissingVariable error");
        }
    }

    #[tokio::test]
    async fn test_few_shot_template_with_partial_variables() {
        let prefix_template = Template::new("Welcome, {user}!").unwrap();
        let example_template = Template::new("Your role is {role}.").unwrap();
        let suffix_template = Template::new("Goodbye, {user}.").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .example(example_template)
            .suffix(suffix_template)
            .build();

        let variables = vars!(user = "Alice",);
        let result = few_shot_template.format(variables).await;

        assert!(result.is_err());
        if let Err(TemplateError::MissingVariable(msg)) = result {
            assert!(msg.contains("role"));
        } else {
            panic!("Expected MissingVariable error");
        }
    }

    #[tokio::test]
    async fn test_few_shot_template_with_custom_example_separator() {
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

        let variables = vars!();
        let formatted_output = few_shot_template.format(variables).await.unwrap();

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

    #[tokio::test]
    async fn test_few_shot_template_with_plain_text_templates() {
        let prefix_template = Template::new("Plain prefix").unwrap();
        let example_template = Template::new("Plain example").unwrap();
        let suffix_template = Template::new("Plain suffix").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .example(example_template)
            .suffix(suffix_template)
            .build();

        let variables = vars!();
        let formatted_output = few_shot_template.format(variables).await.unwrap();

        let expected_output = "\
Plain prefix

Plain example

Plain suffix";

        assert_eq!(formatted_output, expected_output);
    }

    #[tokio::test]
    async fn test_few_shot_template_with_multiple_examples() {
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
        let variables = vars!();
        let formatted_output = few_shot_template.format(variables).await.unwrap();

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

    #[tokio::test]
    async fn test_few_shot_template_with_repeated_variables() {
        let prefix_template = Template::new("Start {var}").unwrap();
        let example_template = Template::new("Example {var}").unwrap();
        let suffix_template = Template::new("End {var}").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .example(example_template.clone())
            .example(example_template)
            .suffix(suffix_template)
            .build();

        let variables = vars!(var = "Value",);
        let formatted_output = few_shot_template.format(variables).await.unwrap();

        let expected_output = "\
Start Value

Example Value

Example Value

End Value";

        assert_eq!(formatted_output, expected_output);
    }

    #[tokio::test]
    async fn test_few_shot_template_with_no_examples() {
        let prefix_template = Template::new("Only Prefix").unwrap();
        let suffix_template = Template::new("Only Suffix").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix(prefix_template)
            .suffix(suffix_template)
            .build();

        let variables = vars!();
        let formatted_output = few_shot_template.format(variables).await.unwrap();

        let expected_output = "\
Only Prefix

Only Suffix";

        assert_eq!(formatted_output, expected_output);
    }

    #[tokio::test]
    async fn test_few_shot_template_langchain_example() {
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
            let formatted_example = example_template.format(example_vars.clone()).await.unwrap();
            let example = Template::new(&formatted_example).unwrap();
            formatted_examples.push(example);
        }

        let suffix_template_str = r#"Question: {input}"#;

        let suffix_template = Template::new(suffix_template_str).unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .add_examples(formatted_examples)
            .suffix(suffix_template)
            .example_separator("\n\n")
            .build();

        let variables = vars!(input = "Who was the father of Mary Ball Washington?");

        let formatted_output = few_shot_template.format(variables).await.unwrap();

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
}
