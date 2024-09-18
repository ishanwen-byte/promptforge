use std::{convert::TryFrom, fmt, sync::Arc};

use messageforge::{AiMessage, HumanMessage, MessageEnum, SystemMessage};

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

impl fmt::Display for InvalidRoleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid role provided")
    }
}

impl std::error::Error for InvalidRoleError {}

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

    pub fn to_message(self, content: &str) -> Result<Arc<MessageEnum>, InvalidRoleError> {
        let message_enum = match self {
            Role::System => MessageEnum::System(SystemMessage::new(content)),
            Role::Human => MessageEnum::Human(HumanMessage::new(content)),
            Role::Ai => MessageEnum::Ai(AiMessage::new(content)),
            _ => return Err(InvalidRoleError),
        };

        Ok(Arc::new(message_enum))
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
    use messageforge::BaseMessage;

    fn test_message_creation(role: Role, content: &str) {
        let result = role.to_message(content).unwrap();
        assert_eq!(result.content(), content);
    }

    fn test_invalid_message_creation(role: Role, content: &str) {
        let result = role.to_message(content);
        assert_eq!(result.unwrap_err(), InvalidRoleError);
    }

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
        test_message_creation(Role::System, "This is a system message.");
    }

    #[test]
    fn test_human_message_creation() {
        test_message_creation(Role::Human, "This is a human message.");
    }

    #[test]
    fn test_ai_message_creation() {
        test_message_creation(Role::Ai, "This is an AI message.");
    }

    #[test]
    fn test_tool_message_creation() {
        test_invalid_message_creation(Role::Tool, "This is a tool message.");
    }

    #[test]
    fn test_placeholder_message_creation() {
        test_invalid_message_creation(Role::Placeholder, "This is a placeholder message.");
    }

    #[test]
    fn test_invalid_role_message() {
        let result = Role::try_from("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_message_creation() {
        test_message_creation(Role::Ai, "");
    }

    #[test]
    fn test_case_insensitivity() {
        assert_eq!(Role::try_from("HUMAN").unwrap(), Role::Human);
        assert_eq!(Role::try_from("AI").unwrap(), Role::Ai);
    }
}
