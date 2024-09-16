#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessagesPlaceholder {
    variable_name: String,
    optional: bool,
    n_messages: usize,
}

impl MessagesPlaceholder {
    pub const DEFAULT_LIMIT: usize = 100;

    pub fn new(variable_name: String) -> Self {
        Self::with_options(variable_name, false, Self::DEFAULT_LIMIT)
    }

    pub fn with_options(variable_name: String, optional: bool, n_messages: usize) -> Self {
        Self {
            variable_name,
            optional,
            n_messages: if n_messages == 0 {
                Self::DEFAULT_LIMIT
            } else {
                n_messages
            },
        }
    }

    pub fn variable_name(&self) -> &str {
        &self.variable_name
    }

    pub fn optional(&self) -> bool {
        self.optional
    }

    pub fn n_messages(&self) -> usize {
        self.n_messages
    }

    pub fn set_n_messages(&mut self, n_messages: usize) {
        self.n_messages = n_messages;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messages_placeholder_new() {
        let placeholder = MessagesPlaceholder::new("history".to_string());

        assert_eq!(placeholder.variable_name, "history");
        assert!(!placeholder.optional);
        assert_eq!(placeholder.n_messages, MessagesPlaceholder::DEFAULT_LIMIT);
    }

    #[test]
    fn test_messages_placeholder_with_options() {
        let placeholder = MessagesPlaceholder::with_options("history".to_string(), true, 50);

        assert_eq!(placeholder.variable_name, "history");
        assert!(placeholder.optional);
        assert_eq!(placeholder.n_messages, 50);
    }

    #[test]
    fn test_messages_placeholder_with_zero_limit() {
        let placeholder = MessagesPlaceholder::with_options("history".to_string(), false, 0);

        assert_eq!(placeholder.variable_name, "history");
        assert!(!placeholder.optional);
        assert_eq!(placeholder.n_messages, MessagesPlaceholder::DEFAULT_LIMIT);
    }

    #[test]
    fn test_messages_placeholder_default_limit_on_zero() {
        let placeholder = MessagesPlaceholder::with_options("history".to_string(), true, 0);

        assert_eq!(placeholder.variable_name, "history");
        assert!(placeholder.optional);
        assert_eq!(placeholder.n_messages, MessagesPlaceholder::DEFAULT_LIMIT);
    }
}
