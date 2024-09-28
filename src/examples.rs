#[macro_export]
macro_rules! examples {
    () => {
        Vec::<$crate::Template>::new()
    };
    ($(($input:expr, $output:expr)),+ $(,)?) => {
        vec![
            $(
                $crate::Template::new(&format!("{}\n{}", $input, $output))
                    .expect("Failed to create Template"),
            )+
        ]
    };
}

#[cfg(test)]
mod tests {
    use crate::Templatable;

    #[test]
    fn test_examples_macro_with_multiple_entries() {
        let examples = examples![
            ("{input} What is 2 + 2?", "{output} 4"),
            ("{input} What is 3 + 3?", "{output} 6"),
        ];

        assert_eq!(examples.len(), 2);
        assert_eq!(examples[0].template(), "{input} What is 2 + 2?\n{output} 4");
        assert_eq!(examples[1].template(), "{input} What is 3 + 3?\n{output} 6");
    }

    #[test]
    fn test_examples_macro_with_single_entry() {
        let examples = examples![("{input} What is the capital of France?", "{output} Paris"),];

        assert_eq!(examples.len(), 1);
        assert_eq!(
            examples[0].template(),
            "{input} What is the capital of France?\n{output} Paris"
        );
    }

    #[test]
    fn test_examples_macro_with_empty_input() {
        let examples = examples![];

        assert!(examples.is_empty());
    }

    #[test]
    fn test_examples_macro_with_trailing_comma() {
        let examples = examples![
            ("{input} What is 5 + 5?", "{output} 10"),
            ("{input} What is 6 + 4?", "{output} 10"),
        ];

        assert_eq!(examples.len(), 2);
    }

    #[test]
    fn test_examples_macro_with_complex_strings() {
        let examples = examples![
            ("{input} Solve ∫x^2 dx", "{output} (1/3)x^3 + C"),
            ("{input} Translate 'Hello' to Spanish", "{output} 'Hola'"),
        ];

        assert_eq!(examples.len(), 2);
        assert_eq!(
            examples[0].template(),
            "{input} Solve ∫x^2 dx\n{output} (1/3)x^3 + C"
        );
        assert_eq!(
            examples[1].template(),
            "{input} Translate 'Hello' to Spanish\n{output} 'Hola'"
        );
    }

    #[test]
    fn test_examples_macro_with_variables() {
        let input_question = "{input} What's the weather today?";
        let output_answer = "{output} It's sunny and warm.";

        let examples = examples![(input_question, output_answer),];

        assert_eq!(examples.len(), 1);
        assert_eq!(
            examples[0].template(),
            "{input} What's the weather today?\n{output} It's sunny and warm."
        );
    }

    #[test]
    fn test_examples_macro_with_special_characters() {
        let examples = examples![
            ("{input} What's 7 * 8?", "{output} 56"),
            ("{input} List symbols: @#$%^&*()", "{output} Symbols listed"),
        ];

        assert_eq!(examples.len(), 2);
        assert_eq!(
            examples[1].template(),
            "{input} List symbols: @#$%^&*()\n{output} Symbols listed"
        );
    }

    #[test]
    fn test_examples_macro_with_newlines_in_strings() {
        let examples = examples![("{input} First line\nSecond line", "{output} Response line"),];

        assert_eq!(examples.len(), 1);
        assert_eq!(
            examples[0].template(),
            "{input} First line\nSecond line\n{output} Response line"
        );
    }
}
