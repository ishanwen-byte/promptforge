use crate::template_format::{TemplateError, TemplateFormat};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

pub trait Formattable<K, V>
where
    K: Into<String> + Hash + Eq,
    V: Into<String> + Display,
{
    fn format(&self, variables: &HashMap<K, V>) -> Result<String, TemplateError>;
}

pub trait Templatable<K, V>: Formattable<K, V>
where
    K: Into<String> + Hash + Eq,
    V: Into<String> + Display,
{
    fn template(&self) -> &str;
    fn template_format(&self) -> TemplateFormat;
    fn input_variables(&self) -> Vec<String>;
}
