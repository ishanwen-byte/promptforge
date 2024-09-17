use crate::template_format::{TemplateError, TemplateFormat};
use std::collections::HashMap;

pub trait Templatable {
    fn format(
        &self,
        variables: HashMap<&str, &str>,
    ) -> impl std::future::Future<Output = Result<String, TemplateError>> + Send;
    fn template(&self) -> &str;
    fn template_format(&self) -> TemplateFormat;
    fn input_variables(&self) -> Vec<String>;
}
