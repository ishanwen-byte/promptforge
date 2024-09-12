#[macro_export]
macro_rules! chat_templates {
    ($($role:ident = $tmpl:expr),+ $(,)?) => {
        &[
            $(
                ($role, $tmpl),
            )+
        ]
    };
}

#[cfg(test)]
mod tests {
    use crate::chat_templates;
    use crate::role::Role::{Ai, Human, System};

    #[test]
    fn test_single_message() {
        let templates = chat_templates!(System = "You are a helpful AI bot.");

        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0].0, System);
        assert_eq!(templates[0].1, "You are a helpful AI bot.");
    }

    #[test]
    fn test_multiple_messages() {
        let templates = chat_templates!(
            System = "You are a helpful AI bot.",
            Human = "Hello, how are you doing?",
            Ai = "I'm doing well, thanks!",
        );

        assert_eq!(templates.len(), 3);
        assert_eq!(templates[0].0, System);
        assert_eq!(templates[0].1, "You are a helpful AI bot.");

        assert_eq!(templates[1].0, Human);
        assert_eq!(templates[1].1, "Hello, how are you doing?");

        assert_eq!(templates[2].0, Ai);
        assert_eq!(templates[2].1, "I'm doing well, thanks!");
    }

    #[test]
    fn test_variable_placeholders() {
        let templates = chat_templates!(
            System = "You are a {adjective} AI bot. Your name is {name}.",
            Human = "What is your name?",
        );

        assert_eq!(templates.len(), 2);
        assert_eq!(templates[0].0, System);
        assert_eq!(
            templates[0].1,
            "You are a {adjective} AI bot. Your name is {name}."
        );

        assert_eq!(templates[1].0, Human);
        assert_eq!(templates[1].1, "What is your name?");
    }

    #[test]
    fn test_no_trailing_comma() {
        let templates = chat_templates!(
            System = "You are a helpful AI bot.",
            Human = "Hello, how are you doing?"
        );

        assert_eq!(templates.len(), 2);
        assert_eq!(templates[0].0, System);
        assert_eq!(templates[0].1, "You are a helpful AI bot.");

        assert_eq!(templates[1].0, Human);
        assert_eq!(templates[1].1, "Hello, how are you doing?");
    }

    #[test]
    fn test_trailing_comma() {
        let templates = chat_templates!(
            System = "You are a helpful AI bot.",
            Human = "Hello, how are you doing?",
        );

        assert_eq!(templates.len(), 2);
        assert_eq!(templates[0].0, System);
        assert_eq!(templates[0].1, "You are a helpful AI bot.");

        assert_eq!(templates[1].0, Human);
        assert_eq!(templates[1].1, "Hello, how are you doing?");
    }

    #[test]
    fn test_empty_template() {
        let templates = chat_templates!(System = "", Human = "Hello!",);
        assert_eq!(templates.len(), 2);
        assert_eq!(templates[0].0, System);
        assert_eq!(templates[0].1, "");

        assert_eq!(templates[1].0, Human);
        assert_eq!(templates[1].1, "Hello!");
    }
}
