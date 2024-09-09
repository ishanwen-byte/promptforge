use crate::placeholder::extract_variables;
use crate::template::Template;
use crate::template_format::{detect_template, validate_template, TemplateError, TemplateFormat};

#[derive(Debug)]
pub struct PromptTemplate {
    template: String,
    template_format: TemplateFormat,
    input_variables: Vec<String>,
}

impl PromptTemplate {
    pub fn new(tmpl: &str) -> Result<Self, TemplateError> {
        validate_template(tmpl)?;

        let template_format = detect_template(tmpl)?;
        let input_variables = extract_variables(tmpl);
        let template = tmpl.to_string();

        Ok(PromptTemplate {
            template,
            template_format,
            input_variables,
        })
    }

    pub fn from_template(tmpl: &str) -> Result<Self, TemplateError> {
        Self::new(tmpl)
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

    fn format_fmtstring(
        &self,
        variables: &std::collections::HashMap<&str, &str>,
    ) -> Result<String, TemplateError> {
        let mut result = self.template.clone();

        for var in &self.input_variables {
            if let Some(value) = variables.get(var.as_str()) {
                let placeholder = format!("{{{}}}", var);
                result = result.replace(&placeholder, value);
            } else {
                return Err(TemplateError::MissingVariable(var.clone()));
            }
        }

        Ok(result)
    }

    fn format_mustache(
        &self,
        variables: &std::collections::HashMap<&str, &str>,
    ) -> Result<String, TemplateError> {
        Ok("".to_string())
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
}
