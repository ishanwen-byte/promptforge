use promptforge::{FewShotChatTemplate, MessageLike, Role, Templatable};
use std::path::Path;

#[tokio::test]
async fn test_few_shot_chat_template_from_toml_file() {
    let path = Path::new("tests/data/few_shot_chat_template.toml");
    let template_result = FewShotChatTemplate::from_toml_file(path).await;

    assert!(
        template_result.is_ok(),
        "Failed to parse TOML: {:?}",
        template_result
    );
    let few_shot_chat_template = template_result.unwrap();

    assert_eq!(few_shot_chat_template.example_separator(), "\n---\n");
    let examples = few_shot_chat_template.examples();
    assert_eq!(examples.len(), 2);

    assert_eq!(
        examples[0].template(),
        "{question}: What is 5 + 5?\\n{answer}: 10"
    );
    assert_eq!(examples[0].template_format().as_str(), "FmtString");
    assert_eq!(examples[0].input_variables(), &["question", "answer"]);

    assert_eq!(
        examples[1].template(),
        "{question}: What is 6 + 6?\\n{answer}: 12"
    );
    assert_eq!(examples[1].template_format().as_str(), "FmtString");
    assert_eq!(examples[1].input_variables(), &["question", "answer"]);

    let formatted_examples = few_shot_chat_template.format_examples().unwrap();
    let expected_output = "\
        This is the prefix.\n---\nhuman: What is 5 + 5?\\nai: 10\n---\n\
        human: What is 6 + 6?\\nai: 12\n---\nThis is the suffix.\n\n";
    assert_eq!(formatted_examples, expected_output);

    let chat_prompt = few_shot_chat_template.example_prompt();
    assert_eq!(chat_prompt.messages.len(), 2);

    if let MessageLike::RolePromptTemplate(role, template) = &chat_prompt.messages[0] {
        assert_eq!(template.template(), "{question}");
        assert_eq!(role, &Role::Human);
    } else {
        panic!("Expected a PromptTemplate for the Human message.");
    }

    if let MessageLike::RolePromptTemplate(role, template) = &chat_prompt.messages[1] {
        assert_eq!(template.template(), "{answer}");
        assert_eq!(role, &Role::Ai);
    } else {
        panic!("Expected a BaseMessage for the AI message.");
    }
}
