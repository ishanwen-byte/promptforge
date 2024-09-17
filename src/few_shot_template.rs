use crate::Templatable;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FewShotTemplate<T: Templatable + Send + Sync> {
    prefix: String,
    suffix: String,
    example_separator: String,
    examples: Vec<Arc<T>>,
}

impl<T: Templatable + Send + Sync> Default for FewShotTemplate<T> {
    fn default() -> Self {
        Self {
            prefix: Self::DEFAULT_PREFIX.to_string(),
            suffix: Self::DEFAULT_SUFFIX.to_string(),
            example_separator: Self::DEFAULT_EXAMPLE_SEPARATOR.to_string(),
            examples: Vec::new(),
        }
    }
}

impl<T: Templatable + Send + Sync> FewShotTemplate<T> {
    pub const DEFAULT_PREFIX: &'static str = "";
    pub const DEFAULT_SUFFIX: &'static str = "";
    pub const DEFAULT_EXAMPLE_SEPARATOR: &'static str = "\n\n";

    pub fn new(examples: Vec<Arc<T>>) -> Self {
        Self {
            examples,
            ..Default::default()
        }
    }

    pub fn with_options(
        examples: Vec<Arc<T>>,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
        example_separator: impl Into<String>,
    ) -> Self {
        FewShotTemplate {
            prefix: prefix.into(),
            suffix: suffix.into(),
            example_separator: example_separator.into(),
            examples,
        }
    }

    pub fn builder() -> FewShotTemplateBuilder<T> {
        FewShotTemplateBuilder::new()
    }
}

#[derive(Debug)]
pub struct FewShotTemplateBuilder<T: Templatable + Send + Sync> {
    prefix: String,
    suffix: String,
    example_separator: String,
    examples: Vec<Arc<T>>,
}

impl<T: Templatable + Send + Sync> Default for FewShotTemplateBuilder<T> {
    fn default() -> Self {
        Self {
            prefix: FewShotTemplate::<T>::DEFAULT_PREFIX.to_string(),
            suffix: FewShotTemplate::<T>::DEFAULT_SUFFIX.to_string(),
            example_separator: FewShotTemplate::<T>::DEFAULT_EXAMPLE_SEPARATOR.to_string(),
            examples: Vec::new(),
        }
    }
}

impl<T: Templatable + Send + Sync> FewShotTemplateBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = suffix.into();
        self
    }

    pub fn example_separator(mut self, example_separator: impl Into<String>) -> Self {
        self.example_separator = example_separator.into();
        self
    }

    pub fn add_example(mut self, example: T) -> Self {
        self.examples.push(Arc::new(example));
        self
    }

    pub fn build(self) -> FewShotTemplate<T> {
        FewShotTemplate::with_options(
            self.examples,
            self.prefix,
            self.suffix,
            self.example_separator,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Template;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_few_shot_template_default() {
        let few_shot_template = FewShotTemplate::<Template>::default();

        assert_eq!(few_shot_template.prefix, "");
        assert_eq!(few_shot_template.suffix, "");
        assert_eq!(few_shot_template.example_separator, "\n\n");
        assert!(few_shot_template.examples.is_empty());
    }

    #[tokio::test]
    async fn test_few_shot_template_new() {
        let example_template = Template::new("Hello, {name}!").unwrap();
        let example_arc = Arc::new(example_template);

        let few_shot_template = FewShotTemplate::new(vec![example_arc.clone()]);

        assert_eq!(few_shot_template.prefix, "");
        assert_eq!(few_shot_template.suffix, "");
        assert_eq!(few_shot_template.example_separator, "\n\n");
        assert_eq!(few_shot_template.examples.len(), 1);
        assert!(Arc::ptr_eq(&few_shot_template.examples[0], &example_arc));
    }

    #[tokio::test]
    async fn test_few_shot_template_with_options() {
        let example_template1 = Template::new("Hi, {name}!").unwrap();
        let example_template2 = Template::new("Welcome, {name}!").unwrap();

        let examples = vec![
            Arc::new(example_template1.clone()),
            Arc::new(example_template2.clone()),
        ];

        let few_shot_template =
            FewShotTemplate::with_options(examples.clone(), "Prefix", "Suffix", "---");

        assert_eq!(few_shot_template.prefix, "Prefix");
        assert_eq!(few_shot_template.suffix, "Suffix");
        assert_eq!(few_shot_template.example_separator, "---");
        assert_eq!(few_shot_template.examples.len(), 2);
        assert!(Arc::ptr_eq(&few_shot_template.examples[0], &examples[0]));
        assert!(Arc::ptr_eq(&few_shot_template.examples[1], &examples[1]));
    }

    #[tokio::test]
    async fn test_few_shot_template_builder() {
        let example_template1 = Template::new("Hi, {name}!").unwrap();
        let example_template2 = Template::new("Welcome, {name}!").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix("Builder Prefix")
            .suffix("Builder Suffix")
            .example_separator("===")
            .add_example(example_template1.clone())
            .add_example(example_template2.clone())
            .build();

        assert_eq!(few_shot_template.prefix, "Builder Prefix");
        assert_eq!(few_shot_template.suffix, "Builder Suffix");
        assert_eq!(few_shot_template.example_separator, "===");
        assert_eq!(few_shot_template.examples.len(), 2);

        // Since we're storing Arc<T>, we can check pointer equality
        assert_eq!(
            few_shot_template.examples[0].template(),
            example_template1.template()
        );
        assert_eq!(
            few_shot_template.examples[1].template(),
            example_template2.template()
        );
    }

    #[tokio::test]
    async fn test_few_shot_template_empty_examples() {
        let few_shot_template = FewShotTemplate::<Template>::new(vec![]);

        assert_eq!(few_shot_template.examples.len(), 0);
    }

    #[tokio::test]
    async fn test_few_shot_template_builder_defaults() {
        let few_shot_template = FewShotTemplate::<Template>::builder().build();

        assert_eq!(few_shot_template.prefix, "");
        assert_eq!(few_shot_template.suffix, "");
        assert_eq!(few_shot_template.example_separator, "\n\n");
        assert_eq!(few_shot_template.examples.len(), 0);
    }

    #[tokio::test]
    async fn test_few_shot_template_builder_partial() {
        let example_template = Template::new("Hi, {name}!").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix("Only Prefix")
            .add_example(example_template.clone())
            .build();

        assert_eq!(few_shot_template.prefix, "Only Prefix");
        assert_eq!(few_shot_template.suffix, "");
        assert_eq!(few_shot_template.example_separator, "\n\n");
        assert_eq!(few_shot_template.examples.len(), 1);
    }

    #[tokio::test]
    async fn test_few_shot_template_examples_content() {
        let example_template1 = Template::new("Hi, {name}!").unwrap();
        let example_template2 = Template::new("Welcome, {name}!").unwrap();

        let examples = vec![
            Arc::new(example_template1.clone()),
            Arc::new(example_template2.clone()),
        ];

        let few_shot_template = FewShotTemplate::with_options(examples.clone(), "", "", "\n");

        assert_eq!(few_shot_template.examples.len(), 2);

        assert_eq!(few_shot_template.examples[0].template(), "Hi, {name}!");
        assert_eq!(few_shot_template.examples[1].template(), "Welcome, {name}!");
    }

    #[tokio::test]
    async fn test_builder_chaining() {
        let example_template = Template::new("Example {number}").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .prefix("Prefix")
            .suffix("Suffix")
            .example_separator("---")
            .add_example(example_template.clone())
            .build();

        assert_eq!(few_shot_template.prefix, "Prefix");
        assert_eq!(few_shot_template.suffix, "Suffix");
        assert_eq!(few_shot_template.example_separator, "---");
        assert_eq!(few_shot_template.examples.len(), 1);
    }

    #[tokio::test]
    async fn test_multiple_examples_in_builder() {
        let example_template1 = Template::new("Example 1").unwrap();
        let example_template2 = Template::new("Example 2").unwrap();
        let example_template3 = Template::new("Example 3").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .add_example(example_template1.clone())
            .add_example(example_template2.clone())
            .add_example(example_template3.clone())
            .build();

        assert_eq!(few_shot_template.examples.len(), 3);
        assert_eq!(few_shot_template.examples[0].template(), "Example 1");
        assert_eq!(few_shot_template.examples[1].template(), "Example 2");
        assert_eq!(few_shot_template.examples[2].template(), "Example 3");
    }

    #[tokio::test]
    async fn test_custom_example_separator() {
        let example_template1 = Template::new("Example A").unwrap();
        let example_template2 = Template::new("Example B").unwrap();

        let few_shot_template = FewShotTemplate::builder()
            .example_separator("||")
            .add_example(example_template1.clone())
            .add_example(example_template2.clone())
            .build();

        assert_eq!(few_shot_template.example_separator, "||");
    }
}
