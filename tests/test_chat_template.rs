use messageforge::BaseMessage;
use std::collections::HashMap;
use std::path::Path;

use promptforge::{ChatTemplate, Formattable, MessageLike};

#[tokio::test]
async fn test_chat_template_from_toml_file() {
    let toml_file_path = Path::new("tests/data/chat_template.toml");

    let chat_template = ChatTemplate::from_toml_file(toml_file_path).await;

    assert!(chat_template.is_ok());
    let chat_template = chat_template.unwrap();

    assert_eq!(chat_template.messages.len(), 3);

    if let Some(system_message) = chat_template.messages.first() {
        match system_message {
            MessageLike::BaseMessage(msg) => {
                assert_eq!(msg.content(), "System initialized.");
                assert_eq!(msg.message_type().as_str(), "system");
            }
            _ => panic!("Expected BaseMessage for system role"),
        }
    } else {
        panic!("Expected system message to be present");
    }

    if let Some(human_message) = chat_template.messages.get(1) {
        match human_message {
            MessageLike::BaseMessage(msg) => {
                assert_eq!(msg.content(), "Hello, AI!");
                assert_eq!(msg.message_type().as_str(), "human");
            }
            _ => panic!("Expected BaseMessage for human role"),
        }
    } else {
        panic!("Expected human message to be present");
    }

    if let Some(ai_message) = chat_template.messages.get(2) {
        match ai_message {
            MessageLike::BaseMessage(msg) => {
                assert_eq!(msg.content(), "Hello, human! How can I assist you today?");
                assert_eq!(msg.message_type().as_str(), "ai");
            }
            _ => panic!("Expected BaseMessage for ai role"),
        }
    } else {
        panic!("Expected ai message to be present");
    }
}

#[tokio::test]
async fn test_chat_template_format_with_variables() {
    let toml_file_path = Path::new("tests/data/chat_template.toml");
    let chat_template = ChatTemplate::from_toml_file(toml_file_path).await.unwrap();

    let variables = HashMap::new();

    let formatted_output = chat_template.format(&variables).unwrap();

    let expected_output = "\
system: System initialized.
human: Hello, AI!
ai: Hello, human! How can I assist you today?";

    assert_eq!(formatted_output, expected_output);
}
