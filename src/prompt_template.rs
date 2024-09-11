//! # PromptTemplate
//!
//! `PromptTemplate` is a struct designed to simplify the creation and formatting of dynamic, reusable prompts for AI-driven applications. It supports multiple template formats, including `FmtString` (similar to Python's f-strings) and `Mustache` (a logic-less templating system).
//!
//! This struct provides an easy way to define templates with placeholders for variables and then substitute values for those placeholders at runtime.
//!
//! ## Example Usage
//!
//! ### FmtString Template
//!
//! ```rust
//! use promptforge::{PromptTemplate, TemplateError, prompt_vars};
//! use promptforge::Template;
//!
//! fn main() -> Result<(), TemplateError> {
//!     let tmpl = PromptTemplate::new("Hello, {name}! Your order number is {order_id}.")?;
//!     let variables = prompt_vars!(name = "Alice", order_id = "12345");
//!     let result = tmpl.format(variables)?;
//!     
//!     println!("{}", result);  // Outputs: Hello, Alice! Your order number is 12345.
//!     Ok(())
//! }
//! ```
//!
//! ### Mustache Template
//!
//! ```rust
//! use promptforge::Template;
//! use promptforge::{PromptTemplate, TemplateError, prompt_vars};
//!
//! fn main() -> Result<(), TemplateError> {
//!     let tmpl = PromptTemplate::new("Hello, {{name}}! Your favorite color is {{color}}.")?;
//!     let variables = prompt_vars!(name = "Bob", color = "blue");
//!     let result = tmpl.format(variables)?;
//!     
//!     println!("{}", result);  // Outputs: Hello, Bob! Your favorite color is blue.
//!     Ok(())
//! }
//! ```
//!
//! ### Handling Missing Variables
//!
//! ```rust
//! use promptforge::Template;
//! use promptforge::{PromptTemplate, TemplateError, prompt_vars};
//!
//! fn main() -> Result<(), TemplateError> {
//!     let tmpl = PromptTemplate::new("Hi, {name}! Please confirm your email: {email}.")?;
//!     let variables = prompt_vars!(name = "Charlie");
//!     let result = tmpl.format(variables);
//!     
//!     assert!(result.is_err());
//!     println!("Error: {:?}", result.unwrap_err());  // Outputs: Error: MissingVariable("email")
//!     Ok(())
//! }
//! ```
//!
//! ## Fields
//!
//! - `template`: The raw string template that contains placeholders for variables. This template can be either in `FmtString` or `Mustache` format.
//! - `template_format`: Specifies whether the template is in `FmtString`, `Mustache`, or `PlainText` format. This is automatically detected based on the template passed in.
//! - `input_variables`: A `Vec<String>` that lists the variable names expected to be provided when formatting the template. These are automatically extracted from the template.
//! - `handlebars`: Optional `Handlebars<'static>` instance. This is only initialized when using Mustache templates. It is used for rendering Mustache-style templates.
//!
//! ## Methods
//!
//! ### `new`
//!
//! ```rust
//! // pub fn new(tmpl: &str) -> Result<Self, TemplateError>
//! ```
//!
//! Creates a new `PromptTemplate` instance from a template string. The function validates the template, detects the format (FmtString or Mustache), and extracts the expected variables.
//!
//! - **Arguments**:
//!   - `tmpl`: The template string, which contains placeholders (e.g., `"{name}"` or `"{{name}}"`).
//! - **Returns**:
//!   - `Result<Self, TemplateError>`: A `PromptTemplate` instance or a `TemplateError` if the template is malformed or contains unsupported formats.
//!
//! ### `from_template`
//!
//! ```rust
//! // pub fn from_template(tmpl: &str) -> Result<Self, TemplateError>
//! ```
//!
//! Alias for `new`. This method is provided to keep consistency with the API, mimicking similar libraries like LangChain.
//!
//! - **Arguments**:
//!   - `tmpl`: The template string.
//! - **Returns**:
//!   - Same as `new`.
//!
//! ### `validate_variables`
//!
//! ```rust
//! // fn validate_variables(&self, variables: &std::collections::HashMap<&str, &str>) -> Result<(), TemplateError>
//! ```
//!
//! Ensures that all required variables for the template are provided in the `variables` map. If a required variable is missing, it returns a `TemplateError::MissingVariable`.
//!
//! - **Arguments**:
//!   - `variables`: A `HashMap` containing the variable names and values to be substituted in the template.
//! - **Returns**:
//!   - `Ok(())` if all variables are valid, otherwise returns a `TemplateError`.
//!
//! ### `format`
//!
//! ```rust
//! // pub fn format(&self, variables: std::collections::HashMap<&str, &str>) -> Result<String, TemplateError>
//! ```
//!
//! Formats the template by substituting the provided variables into the placeholders in the template. The function supports both `FmtString` and `Mustache` templates, performing the appropriate rendering based on the detected format.
//!
//! - **Arguments**:
//!   - `variables`: A `HashMap` containing the variable names and values to be substituted in the template.
//! - **Returns**:
//!   - `Result<String, TemplateError>`: The formatted string or an error if any variables are missing or the template is malformed.
//!
//! ### `template_format`
//!
//! ```rust
//! // pub fn template_format(&self) -> TemplateFormat
//! ```
//!
//! Returns the format of the template, which can be `FmtString`, `Mustache`, or `PlainText`.
//!
//! - **Returns**:
//!   - The `TemplateFormat` for the template.
//!
//! ### `input_variables`
//!
//! ```rust
//! // pub fn input_variables(&self) -> Vec<String>
//! ```
//!
//! Returns a list of the variable names expected by the template.
//!
//! - **Returns**:
//!   - A `Vec<String>` of variable names.
//!
//! ### Internal Helper Methods
//!
//! #### `initialize_handlebars`
//!
//! ```rust
//! // fn initialize_handlebars(tmpl: &str) -> Result<Handlebars<'static>, TemplateError>
//! ```
//!
//! Initializes the `Handlebars` instance and registers the Mustache template. This is used internally when a Mustache template is detected.
//!
//! - **Arguments**:
//!   - `tmpl`: The template string.
//! - **Returns**:
//!   - A `Handlebars` instance or a `TemplateError` if the template registration fails.
//!
//! ## Error Handling
//!
//! `PromptTemplate` provides comprehensive error handling through the `TemplateError` enum. It ensures that:
//!
//! - Templates are validated upon creation (invalid placeholders, mixed formats, etc.).
//! - Missing variables are detected and reported with detailed error messages.
//! - Unsupported template formats are caught early.
//!
//! ## Design Decisions
//!
//! - **Thread Safety**: `PromptTemplate` can be used in asynchronous contexts without issues, as it doesn’t require modification after creation. For multi-threaded environments, you can safely share instances of `PromptTemplate` across threads or tasks by wrapping it in an `Arc`. No additional synchronization (like `Mutex` or `RwLock`) is necessary unless you plan to modify the instance after its creation.
//!
//! - **Handlebars**: The `Handlebars` instance is only created when a Mustache template is detected. This avoids the overhead of initializing it for templates that don’t require Mustache-style rendering.
//!
//! ## Planned Enhancements
//!
//! - **Asynchronous Support**: Adding asynchronous methods to align with async Rust patterns.
//! - **Advanced Templating**: Support for conditionals and loops in Mustache templates.
//! - **Customizable Format**: Allow users to define and plug in custom template formats.
//!
//! ## Conclusion
//!
//! `PromptTemplate` is a powerful and flexible tool for managing dynamic prompts in AI-driven systems. By supporting both FmtString and Mustache formats, it provides developers with the ability to create reusable, dynamic prompts that can be adapted to a wide range of use cases.

use std::collections::HashMap;

use handlebars::Handlebars;

use crate::placeholder::extract_variables;
use crate::template::Template;
use crate::template_format::{detect_template, validate_template, TemplateError, TemplateFormat};

#[derive(Debug)]
pub struct PromptTemplate {
    template: String,
    template_format: TemplateFormat,
    input_variables: Vec<String>,
    handlebars: Option<Handlebars<'static>>,
}

impl PromptTemplate {
    pub const MUSTACHE_TEMPLATE: &'static str = "mustache_template";

    pub fn new(tmpl: &str) -> Result<Self, TemplateError> {
        validate_template(tmpl)?;

        let template_format = detect_template(tmpl)?;
        let input_variables = extract_variables(tmpl);

        let handlebars = if template_format == TemplateFormat::Mustache {
            let handle = Self::initialize_handlebars(tmpl)?;
            Some(handle)
        } else {
            None
        };

        Ok(PromptTemplate {
            template: tmpl.to_string(),
            template_format,
            input_variables,
            handlebars,
        })
    }

    pub fn from_template(tmpl: &str) -> Result<Self, TemplateError> {
        Self::new(tmpl)
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
            if !variables.contains_key(var.as_str()) {
                return Err(TemplateError::MissingVariable(var.clone()));
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
                .map_err(TemplateError::RenderError),
        }
    }
}

impl Template for PromptTemplate {
    fn format(
        &self,
        variables: std::collections::HashMap<&str, &str>,
    ) -> Result<String, TemplateError> {
        self.validate_variables(&variables)?;

        match self.template_format {
            TemplateFormat::FmtString => self.format_fmtstring(&variables),
            TemplateFormat::Mustache => self.format_mustache(&variables),
            TemplateFormat::PlainText => Ok(self.template.clone()),
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt_vars;

    #[test]
    fn test_prompt_template_new_success() {
        let valid_template = "Tell me a {adjective} joke about {content}.";
        let tmpl = PromptTemplate::new(valid_template);
        assert!(tmpl.is_ok());
        let tmpl = tmpl.unwrap();
        assert_eq!(tmpl.template, valid_template);
        assert_eq!(tmpl.template_format, TemplateFormat::FmtString);
        assert_eq!(tmpl.input_variables, vec!["adjective", "content"]);

        let valid_mustache_template = "Tell me a {{adjective}} joke about {{content}}.";
        let tmpl = PromptTemplate::new(valid_mustache_template);
        assert!(tmpl.is_ok());
        let tmpl = tmpl.unwrap();
        assert_eq!(tmpl.template, valid_mustache_template);
        assert_eq!(tmpl.template_format, TemplateFormat::Mustache);
        assert_eq!(tmpl.input_variables, vec!["adjective", "content"]);

        let no_placeholder_template = "Tell me a joke.";
        let tmpl = PromptTemplate::new(no_placeholder_template);
        assert!(tmpl.is_ok());
        let tmpl = tmpl.unwrap();
        assert_eq!(tmpl.template, no_placeholder_template);
        assert_eq!(tmpl.template_format, TemplateFormat::PlainText);
        assert_eq!(tmpl.input_variables.len(), 0);
    }

    #[test]
    fn test_prompt_template_new_error() {
        let mixed_template = "Tell me a {adjective} joke about {{content}}.";
        let tmpl_err = PromptTemplate::new(mixed_template).unwrap_err();
        assert!(matches!(tmpl_err, TemplateError::MalformedTemplate(_)));

        let malformed_fmtstring = "Tell me a {adjective joke about {content}.";
        let tmpl_err = PromptTemplate::new(malformed_fmtstring).unwrap_err();
        assert!(matches!(tmpl_err, TemplateError::MalformedTemplate(_)));

        let malformed_mustache = "Tell me a {{adjective joke about {{content}}.";
        let tmpl_err = PromptTemplate::new(malformed_mustache).unwrap_err();
        assert!(matches!(tmpl_err, TemplateError::MalformedTemplate(_)));
    }

    #[test]
    fn test_fmtstring_formatting() {
        let tmpl = PromptTemplate::new("Hello, {name}!").unwrap();
        let variables = prompt_vars!(name = "John");
        let formatted = tmpl.format(variables).unwrap();
        assert_eq!(formatted, "Hello, John!");

        let tmpl = PromptTemplate::new("Hi {name}, you are {age} years old!").unwrap();
        let variables = prompt_vars!(name = "Alice", age = "30");
        let formatted = tmpl.format(variables).unwrap();
        assert_eq!(formatted, "Hi Alice, you are 30 years old!");

        let tmpl = PromptTemplate::new("Hello World!").unwrap();
        let variables = prompt_vars!();
        let formatted = tmpl.format(variables).unwrap();
        assert_eq!(formatted, "Hello World!");

        let tmpl = PromptTemplate::new("Goodbye, {name}!").unwrap();
        let variables = prompt_vars!(name = "John", extra = "data");
        let formatted = tmpl.format(variables).unwrap();
        assert_eq!(formatted, "Goodbye, John!");

        let tmpl = PromptTemplate::new("Goodbye, {name}!").unwrap();
        let variables = prompt_vars!(wrong_name = "John");
        let result = tmpl.format(variables);
        assert!(result.is_err());

        let tmpl = PromptTemplate::new("Hi {name}, you are {age} years old!").unwrap();
        let variables = prompt_vars!(name = "Alice");
        let result = tmpl.format(variables).unwrap_err();
        assert!(matches!(result, TemplateError::MissingVariable(_)));
    }

    #[test]
    fn test_format_mustache_success() {
        let tmpl = PromptTemplate::new("Hello, {{name}}!").unwrap();
        let variables = prompt_vars!(name = "John");
        let result = tmpl.format(variables).unwrap();
        assert_eq!(result, "Hello, John!");

        let variables = prompt_vars!(name = "John", extra = "data");
        let result = tmpl.format(variables).unwrap();
        assert_eq!(result, "Hello, John!");

        let tmpl_multiple_vars =
            PromptTemplate::new("Hello, {{name}}! You are {{adjective}}.").unwrap();
        let variables = prompt_vars!(name = "John", adjective = "awesome");
        let result = tmpl_multiple_vars.format(variables).unwrap();
        assert_eq!(result, "Hello, John! You are awesome.");

        let tmpl_multiple_instances =
            PromptTemplate::new("{{greeting}}, {{name}}! {{greeting}}, again!").unwrap();
        let variables = prompt_vars!(greeting = "Hello", name = "John");
        let result = tmpl_multiple_instances.format(variables).unwrap();
        assert_eq!(result, "Hello, John! Hello, again!");
    }

    #[test]
    fn test_format_mustache_error() {
        let tmpl_missing_var = PromptTemplate::new("Hello, {{name}}!").unwrap();
        let variables = prompt_vars!(adjective = "cool");
        let err = tmpl_missing_var.format(variables).unwrap_err();
        assert!(matches!(err, TemplateError::MissingVariable(_)));
    }

    #[test]
    fn test_format_plaintext() {
        let tmpl = PromptTemplate::new("Hello, world!").unwrap();
        let variables = prompt_vars!();
        let result = tmpl.format(variables).unwrap();
        assert_eq!(result, "Hello, world!");

        let tmpl = PromptTemplate::new("Welcome to the Rust world!").unwrap();
        let variables = prompt_vars!(name = "John", adjective = "awesome");
        let result = tmpl.format(variables).unwrap();
        assert_eq!(result, "Welcome to the Rust world!");

        let tmpl_no_placeholders = PromptTemplate::new("No placeholders here").unwrap();
        let variables = prompt_vars!(name = "ignored");
        let result = tmpl_no_placeholders.format(variables).unwrap();
        assert_eq!(result, "No placeholders here");

        let tmpl_extra_spaces = PromptTemplate::new("  Just some text   ").unwrap();
        let variables = prompt_vars!();
        let result = tmpl_extra_spaces.format(variables).unwrap();
        assert_eq!(result, "  Just some text   ");

        let tmpl_with_newlines = PromptTemplate::new("Text with\nmultiple lines\n").unwrap();
        let result = tmpl_with_newlines.format(prompt_vars!()).unwrap();
        assert_eq!(result, "Text with\nmultiple lines\n");
    }
}
