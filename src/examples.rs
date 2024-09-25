#[macro_export]
macro_rules! examples {
    // Empty case
    () => {
        Vec::<$crate::ChatTemplate>::new()
    };

    // Handle the `examples!(Human="...", Ai="...")` pattern
    ($($role:ident = $tmpl:expr),+ $(,)?) => {
        vec![
            $crate::ChatTemplate::from_messages($crate::chats!($($role = $tmpl),+)).unwrap(),
        ]
    };

    // Handle the `examples!((Human="...", Ai="..."), ...)` pattern
    ($(($($role:ident = $tmpl:expr),+)),+ $(,)?) => {
        vec![
            $(
                $crate::ChatTemplate::from_messages($crate::chats!($($role = $tmpl),+)).unwrap(),
            )+
        ]
    };
}

#[cfg(test)]
mod tests {
    use messageforge::{BaseMessage, MessageEnum};

    use crate::Role::{Ai, Human};
    use crate::{examples, MessageLike};

    #[test]
    fn test_examples_empty() {
        // Testing empty examples case
        let result = examples!();
        assert!(
            result.is_empty(),
            "Expected an empty vector of ChatTemplate"
        );
    }

    #[test]
    fn test_examples_with_single_example() {
        // Testing examples with a single example (Human and Ai roles)
        let result = examples!(Human = "Hello", Ai = "Hi");

        assert_eq!(result.len(), 1, "Expected one ChatTemplate");

        let chat_template = &result[0];
        assert_eq!(
            chat_template.messages.len(),
            2,
            "Expected two messages in the template"
        );

        // Check the content of messages
        if let MessageLike::BaseMessage(base_message) = &chat_template.messages[0] {
            if let MessageEnum::Human(human_message) = base_message.as_ref() {
                assert_eq!(
                    human_message.content(),
                    "Hello",
                    "Expected Human message content to be 'Hello'"
                );
            } else {
                panic!("Expected HumanMessage, but got something else");
            }
        } else {
            panic!("Expected BaseMessage for HumanMessage, but got something else");
        }

        if let MessageLike::BaseMessage(base_message) = &chat_template.messages[1] {
            if let MessageEnum::Ai(ai_message) = base_message.as_ref() {
                assert_eq!(
                    ai_message.content(),
                    "Hi",
                    "Expected Ai message content to be 'Hi'"
                );
            } else {
                panic!("Expected AiMessage, but got something else");
            }
        } else {
            panic!("Expected BaseMessage for AiMessage, but got something else");
        }
    }

    #[test]
    fn test_examples_with_multiple_examples() {
        // Testing examples with multiple examples (Human and Ai roles)
        let result = examples!((Human = "2 + 2", Ai = "4"), (Human = "3 + 3", Ai = "6"));

        assert_eq!(result.len(), 2, "Expected two ChatTemplates");

        // First example
        let chat_template1 = &result[0];
        assert_eq!(
            chat_template1.messages.len(),
            2,
            "Expected two messages in the first template"
        );

        if let MessageLike::BaseMessage(base_message) = &chat_template1.messages[0] {
            if let MessageEnum::Human(human_message) = base_message.as_ref() {
                assert_eq!(
                    human_message.content(),
                    "2 + 2",
                    "Expected Human message content to be '2 + 2'"
                );
            } else {
                panic!("Expected HumanMessage, but got something else");
            }
        } else {
            panic!("Expected BaseMessage for HumanMessage, but got something else");
        }

        if let MessageLike::BaseMessage(base_message) = &chat_template1.messages[1] {
            if let MessageEnum::Ai(ai_message) = base_message.as_ref() {
                assert_eq!(
                    ai_message.content(),
                    "4",
                    "Expected Ai message content to be '4'"
                );
            } else {
                panic!("Expected AiMessage, but got something else");
            }
        } else {
            panic!("Expected BaseMessage for AiMessage, but got something else");
        }

        // Second example
        let chat_template2 = &result[1];
        assert_eq!(
            chat_template2.messages.len(),
            2,
            "Expected two messages in the second template"
        );

        if let MessageLike::BaseMessage(base_message) = &chat_template2.messages[0] {
            if let MessageEnum::Human(human_message) = base_message.as_ref() {
                assert_eq!(
                    human_message.content(),
                    "3 + 3",
                    "Expected Human message content to be '3 + 3'"
                );
            } else {
                panic!("Expected HumanMessage, but got something else");
            }
        } else {
            panic!("Expected BaseMessage for HumanMessage, but got something else");
        }

        if let MessageLike::BaseMessage(base_message) = &chat_template2.messages[1] {
            if let MessageEnum::Ai(ai_message) = base_message.as_ref() {
                assert_eq!(
                    ai_message.content(),
                    "6",
                    "Expected Ai message content to be '6'"
                );
            } else {
                panic!("Expected AiMessage, but got something else");
            }
        } else {
            panic!("Expected BaseMessage for AiMessage, but got something else");
        }
    }
}
