use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::formatting::{Formattable, Templatable};
use crate::placeholder::extract_variables;
use crate::template_format::{
    detect_template, merge_vars, validate_template, TemplateError, TemplateFormat,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    template: String,
    template_format: TemplateFormat,
    input_variables: Vec<String>,
    #[serde(skip, default)]
    handlebars: Option<Handlebars<'static>>,
    #[serde(skip)]
    partials: HashMap<String, String>,
}

impl Template {
    pub const MUSTACHE_TEMPLATE: &'static str = "mustache_template";

    pub fn new(tmpl: &str) -> Result<Self, TemplateError> {
        validate_template(tmpl)?;

        let template_format = detect_template(tmpl)?;
        let input_variables = extract_variables(tmpl)
            .into_iter()
            .map(|var| var.to_string())
            .collect();

        let handlebars = if template_format == TemplateFormat::Mustache {
            let handle = Self::initialize_handlebars(tmpl)?;
            Some(handle)
        } else {
            None
        };

        Ok(Template {
            template: tmpl.to_string(),
            template_format,
            input_variables,
            handlebars,
            partials: HashMap::new(),
        })
    }

    pub fn from_template(tmpl: &str) -> Result<Self, TemplateError> {
        Self::new(tmpl)
    }

    pub fn partial(&mut self, var: &str, value: &str) -> &mut Self {
        self.partials.insert(var.to_string(), value.to_string());
        self
    }

    pub fn clear_partials(&mut self) -> &mut Self {
        self.partials.clear();
        self
    }

    pub fn partial_vars(&self) -> &HashMap<String, String> {
        &self.partials
    }

    fn initialize_handlebars(tmpl: &str) -> Result<Handlebars<'static>, TemplateError> {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string(Self::MUSTACHE_TEMPLATE, tmpl)
            .map_err(|e| {
                TemplateError::MalformedTemplate(format!("Failed to register template: {}", e))
            })?;
        Ok(handlebars)
    }

    fn validate_variables(
        &self,
        variables: &std::collections::HashMap<&str, &str>,
    ) -> Result<(), TemplateError> {
        for var in &self.input_variables {
            let has_key = variables.contains_key(var.as_str());
            if !has_key {
                return Err(TemplateError::MissingVariable(format!(
                    "Variable '{}' is missing. Expected: {:?}, but received: {:?}",
                    var,
                    self.input_variables,
                    variables.keys().collect::<Vec<_>>()
                )));
            }
        }
        Ok(())
    }

    fn format_fmtstring(&self, variables: &HashMap<&str, &str>) -> Result<String, TemplateError> {
        let mut result = self.template.clone();

        for var in &self.input_variables {
            let placeholder = format!("{{{}}}", var);

            if let Some(value) = variables.get(var.as_str()) {
                result = result.replace(&placeholder, value);
            } else {
                return Err(TemplateError::MissingVariable(var.clone()));
            }
        }

        Ok(result)
    }

    fn format_mustache(&self, variables: &HashMap<&str, &str>) -> Result<String, TemplateError> {
        match &self.handlebars {
            None => Err(TemplateError::UnsupportedFormat(
                "Handlebars not initialized".to_string(),
            )),
            Some(handlebars) => handlebars
                .render(Self::MUSTACHE_TEMPLATE, variables)
                .map_err(TemplateError::RuntimeError),
        }
    }
}

impl Formattable for Template {
    fn format(&self, variables: &HashMap<&str, &str>) -> Result<String, TemplateError> {
        let merged_variables = merge_vars(&self.partials, variables);
        self.validate_variables(&merged_variables)?;

        match self.template_format {
            TemplateFormat::FmtString => self.format_fmtstring(&merged_variables),
            TemplateFormat::Mustache => self.format_mustache(&merged_variables),
            TemplateFormat::PlainText => Ok(self.template.clone()),
        }
    }
}

impl Templatable for Template {
    fn template(&self) -> &str {
        &self.template
    }

    fn template_format(&self) -> TemplateFormat {
        self.template_format.clone()
    }

    fn input_variables(&self) -> Vec<String> {
        self.input_variables.clone()
    }
}

impl TryFrom<String> for Template {
    type Error = TemplateError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Template::new(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vars;

    #[test]
    fn test_prompt_template_new_success() {
        let valid_template = "Tell me a {adjective} joke about {content}.";
        let tmpl = Template::new(valid_template);
        assert!(tmpl.is_ok());
        let tmpl = tmpl.unwrap();
        assert_eq!(tmpl.template, valid_template);
        assert_eq!(tmpl.template_format, TemplateFormat::FmtString);
        assert_eq!(tmpl.input_variables, vec!["adjective", "content"]);

        let valid_mustache_template = "Tell me a {{adjective}} joke about {{content}}.";
        let tmpl = Template::new(valid_mustache_template);
        assert!(tmpl.is_ok());
        let tmpl = tmpl.unwrap();
        assert_eq!(tmpl.template, valid_mustache_template);
        assert_eq!(tmpl.template_format, TemplateFormat::Mustache);
        assert_eq!(tmpl.input_variables, vec!["adjective", "content"]);

        let no_placeholder_template = "Tell me a joke.";
        let tmpl = Template::new(no_placeholder_template);
        assert!(tmpl.is_ok());
        let tmpl = tmpl.unwrap();
        assert_eq!(tmpl.template, no_placeholder_template);
        assert_eq!(tmpl.template_format, TemplateFormat::PlainText);
        assert_eq!(tmpl.input_variables.len(), 0);
    }

    #[test]
    fn test_prompt_template_new_error() {
        let mixed_template = "Tell me a {adjective} joke about {{content}}.";
        let tmpl_err = Template::new(mixed_template).unwrap_err();
        assert!(matches!(tmpl_err, TemplateError::MalformedTemplate(_)));

        let malformed_fmtstring = "Tell me a {adjective joke about {content}.";
        let tmpl_err = Template::new(malformed_fmtstring).unwrap_err();
        assert!(matches!(tmpl_err, TemplateError::MalformedTemplate(_)));

        let malformed_mustache = "Tell me a {{adjective joke about {{content}}.";
        let tmpl_err = Template::new(malformed_mustache).unwrap_err();
        assert!(matches!(tmpl_err, TemplateError::MalformedTemplate(_)));
    }

    #[test]
    fn test_fmtstring_formatting() {
        let tmpl = Template::new("Hello, {name}!").unwrap();
        let variables = &vars!(name = "John");
        let formatted = tmpl.format(variables).unwrap();
        assert_eq!(formatted, "Hello, John!");

        let tmpl = Template::new("Hi {name}, you are {age} years old!").unwrap();
        let variables = &vars!(name = "Alice", age = "30");
        let formatted = tmpl.format(variables).unwrap();
        assert_eq!(formatted, "Hi Alice, you are 30 years old!");

        let tmpl = Template::new("Hello World!").unwrap();
        let variables = &vars!();
        let formatted = tmpl.format(variables).unwrap();
        assert_eq!(formatted, "Hello World!");

        let tmpl = Template::new("Goodbye, {name}!").unwrap();
        let variables = &vars!(name = "John", extra = "data");
        let formatted = tmpl.format(variables).unwrap();
        assert_eq!(formatted, "Goodbye, John!");

        let tmpl = Template::new("Goodbye, {name}!").unwrap();
        let variables = &vars!(wrong_name = "John");
        let result = tmpl.format(variables);
        assert!(result.is_err());

        let tmpl = Template::new("Hi {name}, you are {age} years old!").unwrap();
        let variables = &vars!(name = "Alice");
        let result = tmpl.format(variables).unwrap_err();
        assert!(matches!(result, TemplateError::MissingVariable(_)));
    }

    #[test]
    fn test_format_mustache_success() {
        let tmpl = Template::new("Hello, {{name}}!").unwrap();
        let variables = &vars!(name = "John");
        let result = tmpl.format(variables).unwrap();
        assert_eq!(result, "Hello, John!");

        let variables = &vars!(name = "John", extra = "data");
        let result = tmpl.format(variables).unwrap();
        assert_eq!(result, "Hello, John!");

        let tmpl_multiple_vars = Template::new("Hello, {{name}}! You are {{adjective}}.").unwrap();
        let variables = &vars!(name = "John", adjective = "awesome");
        let result = tmpl_multiple_vars.format(variables).unwrap();
        assert_eq!(result, "Hello, John! You are awesome.");

        let tmpl_multiple_instances =
            Template::new("{{greeting}}, {{name}}! {{greeting}}, again!").unwrap();
        let variables = &vars!(greeting = "Hello", name = "John");
        let result = tmpl_multiple_instances.format(variables).unwrap();
        assert_eq!(result, "Hello, John! Hello, again!");
    }

    #[test]
    fn test_format_mustache_error() {
        let tmpl_missing_var = Template::new("Hello, {{name}}!").unwrap();
        let variables = &vars!(adjective = "cool");
        let err = tmpl_missing_var.format(variables).unwrap_err();
        assert!(matches!(err, TemplateError::MissingVariable(_)));
    }

    #[test]
    fn test_format_plaintext() {
        let tmpl = Template::new("Hello, world!").unwrap();
        let variables = &vars!();
        let result = tmpl.format(variables).unwrap();
        assert_eq!(result, "Hello, world!");

        let tmpl = Template::new("Welcome to the Rust world!").unwrap();
        let variables = &vars!(name = "John", adjective = "awesome");
        let result = tmpl.format(variables).unwrap();
        assert_eq!(result, "Welcome to the Rust world!");

        let tmpl_no_placeholders = Template::new("No placeholders here").unwrap();
        let variables = &vars!(name = "ignored");
        let result = tmpl_no_placeholders.format(variables).unwrap();
        assert_eq!(result, "No placeholders here");

        let tmpl_extra_spaces = Template::new("  Just some text   ").unwrap();
        let variables = &vars!();
        let result = tmpl_extra_spaces.format(variables).unwrap();
        assert_eq!(result, "  Just some text   ");

        let tmpl_with_newlines = Template::new("Text with\nmultiple lines\n").unwrap();
        let result = tmpl_with_newlines.format(&vars!()).unwrap();
        assert_eq!(result, "Text with\nmultiple lines\n");
    }

    #[test]
    fn test_partial_adds_variables() {
        let mut template = Template::new("Hello, {name}").unwrap();

        template.partial("name", "Jill");

        let partial_vars = template.partial_vars();
        assert_eq!(partial_vars.get("name"), Some(&"Jill".to_string()));

        let variables = &vars!();
        let formatted = template.format(variables).unwrap();
        assert_eq!(formatted, "Hello, Jill");

        let variables = &vars!(name = "Alice");
        let formatted = template.format(variables).unwrap();
        assert_eq!(formatted, "Hello, Alice");
    }

    #[test]
    fn test_multiple_partials() {
        let mut template = Template::new("Hello, {name}. You are feeling {mood}.").unwrap();

        template.partial("name", "Jill").partial("mood", "happy");

        let partial_vars = template.partial_vars();
        assert_eq!(partial_vars.get("name"), Some(&"Jill".to_string()));
        assert_eq!(partial_vars.get("mood"), Some(&"happy".to_string()));

        let variables = &vars!();
        let formatted = template.format(variables).unwrap();
        assert_eq!(formatted, "Hello, Jill. You are feeling happy.");

        let variables = &vars!(mood = "excited");
        let formatted = template.format(variables).unwrap();
        assert_eq!(formatted, "Hello, Jill. You are feeling excited.");
    }

    #[test]
    fn test_clear_partials() {
        let mut template = Template::new("Hello, {name}.").unwrap();

        template.partial("name", "Jill").clear_partials();

        let partial_vars = template.partial_vars();
        assert!(partial_vars.is_empty());

        let variables = &vars!(name = "John");
        let formatted = template.format(variables).unwrap();
        assert_eq!(formatted, "Hello, John.");

        let variables = &vars!();
        let result = template.format(variables);
        assert!(result.is_err());
    }

    #[test]
    fn test_partial_vars() {
        let mut template = Template::new("Hello, {name}!").unwrap();
        template.partial("name", "Alice");

        assert_eq!(
            template.partial_vars().get("name"),
            Some(&"Alice".to_string())
        );

        template.partial("name", "Bob");
        assert_eq!(
            template.partial_vars().get("name"),
            Some(&"Bob".to_string())
        );

        template.clear_partials();
        assert!(template.partial_vars().is_empty());

        let variables = &vars!(name = "Charlie");
        let formatted = template.format(variables).unwrap();
        assert_eq!(formatted, "Hello, Charlie!");

        let variables = &vars!();
        let result = template.format(variables);
        assert!(result.is_err());
    }

    #[test]
    fn test_format_with_partials_and_runtime_vars() {
        let mut template = Template::new("Hello, {name}. You are feeling {mood}.").unwrap();

        template.partial("name", "Alice").partial("mood", "calm");

        let variables = &vars!();
        let formatted = template.format(variables).unwrap();
        assert_eq!(formatted, "Hello, Alice. You are feeling calm.");

        let variables = &vars!(mood = "excited");
        let formatted = template.format(variables).unwrap();
        assert_eq!(formatted, "Hello, Alice. You are feeling excited.");

        let variables = &vars!(name = "Bob");
        let formatted = template.format(variables).unwrap();
        assert_eq!(formatted, "Hello, Bob. You are feeling calm.");

        let variables = &vars!(name = "Charlie", mood = "joyful");
        let formatted = template.format(variables).unwrap();
        assert_eq!(formatted, "Hello, Charlie. You are feeling joyful.");
    }

    #[test]
    fn test_format_with_missing_variables_in_partials() {
        let mut template = Template::new("Hello, {name}. You are feeling {mood}.").unwrap();

        template.partial("name", "Alice");

        let variables = &vars!();
        let result = template.format(variables);
        assert!(result.is_err());

        let variables = &vars!(mood = "happy");
        let formatted = template.format(variables).unwrap();
        assert_eq!(formatted, "Hello, Alice. You are feeling happy.");
    }

    #[test]
    fn test_format_with_conflicting_partial_and_runtime_vars() {
        let mut template = Template::new("Hello, {name}. You are feeling {mood}.").unwrap();

        template.partial("name", "Alice").partial("mood", "calm");

        let variables = &vars!(name = "Bob", mood = "excited");
        let formatted = template.format(variables).unwrap();
        assert_eq!(formatted, "Hello, Bob. You are feeling excited.");
    }

    #[test]
    fn test_try_from_string_valid_template() {
        let valid_template = "Hello, {name}! Your order number is {order_id}.".to_string();

        let template = Template::try_from(valid_template.clone());
        assert!(template.is_ok());
        let template = template.unwrap();

        assert_eq!(template.template, valid_template);
        assert_eq!(template.template_format, TemplateFormat::FmtString);
        assert_eq!(
            template.input_variables,
            vec!["name".to_string(), "order_id".to_string()]
        );
    }

    #[test]
    fn test_try_from_string_valid_mustache_template() {
        let valid_mustache_template =
            "Hello, {{name}}! Your favorite color is {{color}}.".to_string();

        let template = Template::try_from(valid_mustache_template.clone());
        assert!(template.is_ok());
        let template = template.unwrap();

        assert_eq!(template.template, valid_mustache_template);
        assert_eq!(template.template_format, TemplateFormat::Mustache);
        assert_eq!(
            template.input_variables,
            vec!["name".to_string(), "color".to_string()]
        );
    }

    #[test]
    fn test_try_from_string_plaintext_template() {
        let plaintext_template = "Hello, world!".to_string();

        let template = Template::try_from(plaintext_template.clone());
        assert!(template.is_ok());
        let template = template.unwrap();

        assert_eq!(template.template, plaintext_template);
        assert_eq!(template.template_format, TemplateFormat::PlainText);
        assert!(template.input_variables.is_empty());
    }

    #[test]
    fn test_try_from_string_malformed_template() {
        let invalid_template = "Hello, {name!".to_string();

        let template = Template::try_from(invalid_template.clone());
        assert!(template.is_err());
        if let Err(TemplateError::MalformedTemplate(msg)) = template {
            println!("{}", msg);
        } else {
            panic!("Expected TemplateError::MalformedTemplate");
        }
    }

    #[test]
    fn test_try_from_string_mixed_format_template() {
        let mixed_format_template = "Hello, {name} and {{color}}.".to_string();

        let template = Template::try_from(mixed_format_template.clone());
        assert!(template.is_err());
        if let Err(TemplateError::MalformedTemplate(msg)) = template {
            println!("{}", msg);
        } else {
            panic!("Expected TemplateError::MalformedTemplate");
        }
    }
}
