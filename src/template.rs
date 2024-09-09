use crate::template_format::{TemplateError, TemplateFormat};
use std::collections::HashMap;

pub trait Template {
    fn format(&self, variables: HashMap<&str, &str>) -> Result<String, TemplateError>;
    fn template_format(&self) -> TemplateFormat;
    fn input_variables(&self) -> Vec<String>;
}
