use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
pub enum Role {
    System,
    Human,
    Ai,
    Tool,
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
        }
    }
}
