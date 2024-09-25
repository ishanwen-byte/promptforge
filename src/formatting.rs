use crate::template_format::{TemplateError, TemplateFormat};
use std::collections::HashMap;

pub trait Formattable {
    fn format(&self, variables: &HashMap<&str, &str>) -> Result<String, TemplateError>;
}

pub trait Templatable: Formattable {
    fn template(&self) -> &str;
    fn template_format(&self) -> TemplateFormat;
    fn input_variables(&self) -> Vec<String>;
}
