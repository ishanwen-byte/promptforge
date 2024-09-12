pub struct Role(pub &str);

impl Role {
    pub const AI: Role = Role("ai");
    pub const HUMAN: Role = Role("human");
    pub const SYSTEM: Role = Role("system");
}
