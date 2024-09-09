use crate::template_format::TemplateFormat;
use std::collections::HashMap;

pub trait Template {
    fn format(&self, variables: HashMap<&str, &str>) -> Result<String, String>;
    fn template_format(&self) -> TemplateFormat;
    fn input_variables(&self) -> Vec<String>;
}
