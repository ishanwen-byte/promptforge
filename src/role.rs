use std::{convert::TryFrom, fmt};

use messageforge::{AiMessage, BaseMessage, HumanMessage, SystemMessage};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Role {
    System,
    Human,
    Ai,
    Tool,
    Placeholder,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidRoleError;

impl TryFrom<&str> for Role {
    type Error = InvalidRoleError;

    fn try_from(role: &str) -> Result<Self, Self::Error> {
        match role.to_lowercase().as_str() {
            "system" => Ok(Role::System),
            "human" => Ok(Role::Human),
            "ai" => Ok(Role::Ai),
            "tool" => Ok(Role::Tool),
            "placeholder" => Ok(Role::Placeholder),
            _ => Err(InvalidRoleError),
        }
    }
}

impl Role {
    pub fn as_str(&self) -> &str {
        match self {
            Role::System => "system",
            Role::Human => "human",
            Role::Ai => "ai",
            Role::Tool => "tool",
            Role::Placeholder => "placeholder",
        }
    }

    pub fn to_message(self, content: &str) -> Result<Box<dyn BaseMessage>, InvalidRoleError> {
        match self {
            Role::System => Ok(Box::new(SystemMessage::new(content))),
            Role::Human => Ok(Box::new(HumanMessage::new(content))),
            Role::Ai => Ok(Box::new(AiMessage::new(content))),
            // Role::Tool => Ok(Box::new(ToolMessage::new(content))),
            _ => Err(InvalidRoleError),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_to_string() {
        assert_eq!(Role::System.to_string(), "system");
        assert_eq!(Role::Human.to_string(), "human");
        assert_eq!(Role::Ai.to_string(), "ai");
        assert_eq!(Role::Tool.to_string(), "tool");
        assert_eq!(Role::Placeholder.to_string(), "placeholder");
    }

    #[test]
    fn test_try_from_str() {
        assert_eq!(Role::try_from("system").unwrap(), Role::System);
        assert_eq!(Role::try_from("human").unwrap(), Role::Human);
        assert_eq!(Role::try_from("ai").unwrap(), Role::Ai);
        assert_eq!(Role::try_from("tool").unwrap(), Role::Tool);
        assert_eq!(Role::try_from("placeholder").unwrap(), Role::Placeholder);
        assert!(Role::try_from("invalid").is_err());
    }

    #[test]
    fn test_system_message_creation() {
        let role = Role::System;
        let content = "This is a system message.";
        let result = role.to_message(content);
        let message = result.unwrap();
        assert_eq!(message.content(), content);
    }

    #[test]
    fn test_human_message_creation() {
        let role = Role::Human;
        let content = "This is a human message.";
        let result = role.to_message(content);
        let message = result.unwrap();
        assert_eq!(message.content(), content);
    }

    #[test]
    fn test_ai_message_creation() {
        let role = Role::Ai;
        let content = "This is an AI message.";
        let result = role.to_message(content);
        let message = result.unwrap();
        assert_eq!(message.content(), content);
    }

    // #[test]
    // fn test_tool_message_creation() {
    //     let role = Role::Tool;
    //     let content = "This is a tool message.";
    //     let result = role.to_message(content);
    //     let message = result.unwrap();
    //     assert_eq!(message.content(), content);
    // }

    #[test]
    fn test_invalid_role() {
        let role = Role::Placeholder;
        let content = "This is a placeholder message.";
        let result = role.to_message(content);

        assert_eq!(result.unwrap_err(), InvalidRoleError);
    }
}
