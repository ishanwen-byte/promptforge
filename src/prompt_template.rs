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
}

impl Template for PromptTemplate {
    fn format(
        &self,
        variables: std::collections::HashMap<&str, &str>,
    ) -> Result<String, TemplateError> {
        Err(TemplateError::UnsupportedFormat(
            "Unsupported format".to_string(),
        ))
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
}
