// SPDX-FileCopyrightText: © 2024 Christopher Woods <Christopher.Woods@bristol.ac.uk>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::error::Error;

/// Grammar for all of the commands that can be sent to agents

///
/// A user identifier - this is a triple of username.project.portal
///
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct UserIdentifier {
    username: String,
    project: String,
    portal: String,
}

impl UserIdentifier {
    pub fn parse(identifier: &str) -> Result<Self, Error> {
        let parts: Vec<&str> = identifier.split('.').collect();

        if parts.len() != 3 {
            return Err(Error::Parse(format!(
                "Invalid UserIdentifier: {}",
                identifier
            )));
        }

        let username = parts[0].trim();
        let project = parts[1].trim();
        let portal = parts[2].trim();

        if username.is_empty() {
            return Err(Error::Parse(format!(
                "Invalid UserIdentifier - username cannot be empty '{}'",
                identifier
            )));
        };

        if project.is_empty() {
            return Err(Error::Parse(format!(
                "Invalid UserIdentifier - project cannot be empty '{}'",
                identifier
            )));
        };

        if portal.is_empty() {
            return Err(Error::Parse(format!(
                "Invalid UserIdentifier - portal cannot be empty '{}'",
                identifier
            )));
        };

        Ok(Self {
            username: username.to_string(),
            project: project.to_string(),
            portal: portal.to_string(),
        })
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn project(&self) -> String {
        self.project.clone()
    }

    pub fn portal(&self) -> String {
        self.portal.clone()
    }

    pub fn is_valid(&self) -> bool {
        !self.username.is_empty() && !self.project.is_empty() && !self.portal.is_empty()
    }
}

impl std::fmt::Display for UserIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.username, self.project, self.portal)
    }
}

/// Serialize and Deserialize via the string representation
/// of the UserIdentifier
impl Serialize for UserIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UserIdentifier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(serde::de::Error::custom)
    }
}

///
/// Struct that holds the mapping of a UserIdentifier to a local
/// username on a system
///
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UserMapping {
    user: UserIdentifier,
    local_user: String,
    local_project: String,
}

impl UserMapping {
    pub fn new(
        user: &UserIdentifier,
        local_user: &str,
        local_project: &str,
    ) -> Result<Self, Error> {
        let local_user = local_user.trim();
        let local_project = local_project.trim();

        if local_user.is_empty() {
            return Err(Error::Parse(format!(
                "Invalid UserMapping - local_user cannot be empty '{}'",
                local_user
            )));
        };

        if local_project.is_empty() {
            return Err(Error::Parse(format!(
                "Invalid UserMapping - local_project cannot be empty '{}'",
                local_project
            )));
        };

        Ok(Self {
            user: user.clone(),
            local_user: local_user.to_string(),
            local_project: local_project.to_string(),
        })
    }

    pub fn parse(identifier: &str) -> Result<Self, Error> {
        let parts: Vec<&str> = identifier.split(':').collect();

        if parts.len() != 3 {
            return Err(Error::Parse(format!("Invalid UserMapping: {}", identifier)));
        }

        let user = UserIdentifier::parse(parts[0])?;
        let local_user = parts[1].trim();
        let local_project = parts[2].trim();

        if local_user.is_empty() {
            return Err(Error::Parse(format!(
                "Invalid UserMapping - local_user cannot be empty '{}'",
                identifier
            )));
        };

        if local_project.is_empty() {
            return Err(Error::Parse(format!(
                "Invalid UserMapping - local_project cannot be empty '{}'",
                identifier
            )));
        };

        Ok(Self {
            user,
            local_user: local_user.to_string(),
            local_project: local_project.to_string(),
        })
    }

    pub fn user(&self) -> &UserIdentifier {
        &self.user
    }

    pub fn local_user(&self) -> &str {
        &self.local_user
    }

    pub fn local_project(&self) -> &str {
        &self.local_project
    }

    pub fn is_valid(&self) -> bool {
        self.user.is_valid() && !self.local_user.is_empty() && !self.local_project.is_empty()
    }
}

impl std::fmt::Display for UserMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.user, self.local_user, self.local_project
        )
    }
}

/// Serialize and Deserialize via the string representation
/// of the UserMapping
impl Serialize for UserMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UserMapping {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(serde::de::Error::custom)
    }
}

///
/// Enum of all of the instructions that can be sent to agents
///
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// An instruction to add a user
    AddUser(UserIdentifier),

    /// An instruction to remove a user
    RemoveUser(UserIdentifier),

    /// An instruction to add a local user
    AddLocalUser(UserMapping),

    /// An instruction to remove a local user
    RemoveLocalUser(UserMapping),

    /// An instruction to update the home directory of a user
    UpdateHomeDir(UserIdentifier, String),

    /// Placeholder for an invalid instruction
    Invalid(),
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::Invalid()
    }
}

impl Instruction {
    pub fn new(s: &str) -> Self {
        let parts: Vec<&str> = s.split(' ').collect();
        match parts[0] {
            "add_user" => match UserIdentifier::parse(&parts[1..].join(" ")) {
                Ok(user) => Instruction::AddUser(user),
                Err(_) => {
                    tracing::error!("add_user failed to parse: {}", &parts[1..].join(" "));
                    Instruction::Invalid()
                }
            },
            "remove_user" => match UserIdentifier::parse(&parts[1..].join(" ")) {
                Ok(user) => Instruction::RemoveUser(user),
                Err(_) => {
                    tracing::error!("remove_user failed to parse: {}", &parts[1..].join(" "));
                    Instruction::Invalid()
                }
            },
            "add_local_user" => match UserMapping::parse(&parts[1..].join(" ")) {
                Ok(mapping) => Instruction::AddLocalUser(mapping),
                Err(_) => {
                    tracing::error!("add_local_user failed to parse: {}", &parts[1..].join(" "));
                    Instruction::Invalid()
                }
            },
            "remove_local_user" => match UserMapping::parse(&parts[1..].join(" ")) {
                Ok(mapping) => Instruction::RemoveLocalUser(mapping),
                Err(_) => {
                    tracing::error!(
                        "remove_local_user failed to parse: {}",
                        &parts[1..].join(" ")
                    );
                    Instruction::Invalid()
                }
            },
            "update_homedir" => {
                if parts.len() < 3 {
                    tracing::error!("update_homedir failed to parse: {}", &parts[1..].join(" "));
                    return Instruction::Invalid();
                }

                let homedir = parts[2].trim().to_string();

                if homedir.is_empty() {
                    tracing::error!("update_homedir failed to parse: {}", &parts[1..].join(" "));
                    return Instruction::Invalid();
                }

                match UserIdentifier::parse(parts[1]) {
                    Ok(user) => Instruction::UpdateHomeDir(user, homedir),
                    Err(_) => {
                        tracing::error!(
                            "update_homedir failed to parse: {}",
                            &parts[1..].join(" ")
                        );
                        Instruction::Invalid()
                    }
                }
            }
            _ => {
                tracing::error!("Invalid instruction: {}", s);
                Instruction::Invalid()
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            Instruction::AddUser(user) => user.is_valid(),
            Instruction::RemoveUser(user) => user.is_valid(),
            Instruction::AddLocalUser(mapping) => mapping.is_valid(),
            Instruction::RemoveLocalUser(mapping) => mapping.is_valid(),
            Instruction::UpdateHomeDir(user, homedir) => user.is_valid() && !homedir.is_empty(),
            Instruction::Invalid() => false,
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::AddUser(user) => write!(f, "add_user {}", user),
            Instruction::RemoveUser(user) => write!(f, "remove_user {}", user),
            Instruction::AddLocalUser(mapping) => write!(f, "add_local_user {}", mapping),
            Instruction::RemoveLocalUser(mapping) => write!(f, "remove_local_user {}", mapping),
            Instruction::UpdateHomeDir(user, homedir) => {
                write!(f, "update_homedir {} {}", user, homedir)
            }
            Instruction::Invalid() => write!(f, "invalid"),
        }
    }
}

/// Serialize and Deserialize via the string representation
/// of the Instructionimpl Serialize for Instruction {
impl Serialize for Instruction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Instruction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Instruction::new(&s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_identifier() {
        let user = UserIdentifier::parse("user.project.portal").unwrap_or_default();
        assert_eq!(user.username(), "user");
        assert_eq!(user.project(), "project");
        assert_eq!(user.portal(), "portal");
        assert_eq!(user.to_string(), "user.project.portal");
    }

    #[test]
    fn test_user_mapping() {
        let user = UserIdentifier::parse("user.project.portal").unwrap_or_default();
        let mapping = UserMapping::new(&user, "local_user", "local_project").unwrap_or_default();
        assert_eq!(mapping.user(), &user);
        assert_eq!(mapping.local_user(), "local_user");
        assert_eq!(mapping.local_project(), "local_project");
        assert_eq!(
            mapping.to_string(),
            "user.project.portal:local_user:local_project"
        );
    }

    #[test]
    fn test_instruction() {
        let user = UserIdentifier::parse("user.project.portal").unwrap_or_default();
        let mapping = UserMapping::new(&user, "local_user", "local_project").unwrap_or_default();

        let instruction = Instruction::new("add_user user.project.portal");
        assert_eq!(instruction, Instruction::AddUser(user.clone()));

        let instruction = Instruction::new("remove_user user.project.portal");
        assert_eq!(instruction, Instruction::RemoveUser(user.clone()));

        let instruction =
            Instruction::new("add_local_user user.project.portal:local_user:local_project");
        assert_eq!(instruction, Instruction::AddLocalUser(mapping.clone()));

        let instruction =
            Instruction::new("remove_local_user user.project.portal:local_user:local_project");
        assert_eq!(instruction, Instruction::RemoveLocalUser(mapping.clone()));

        let instruction = Instruction::new("update_homedir user.project.portal /home/user");
        assert_eq!(
            instruction,
            Instruction::UpdateHomeDir(user.clone(), "/home/user".to_string())
        );

        let instruction = Instruction::new("invalid");
        assert_eq!(instruction, Instruction::Invalid());
    }

    #[test]
    fn assert_serialize_user() {
        let user = UserIdentifier::parse("user.project.portal").unwrap_or_default();
        let serialized = serde_json::to_string(&user).unwrap_or_default();
        assert_eq!(serialized, "\"user.project.portal\"");
    }

    #[test]
    fn assert_deserialize_user() {
        let user: UserIdentifier =
            serde_json::from_str("\"user.project.portal\"").unwrap_or_default();
        assert_eq!(user.to_string(), "user.project.portal");
    }

    #[test]
    fn assert_serialize_mapping() {
        let user = UserIdentifier::parse("user.project.portal").unwrap_or_default();
        let mapping = UserMapping::new(&user, "local_user", "local_project").unwrap_or_default();
        let serialized = serde_json::to_string(&mapping).unwrap_or_default();
        assert_eq!(
            serialized,
            "\"user.project.portal:local_user:local_project\""
        );
    }

    #[test]
    fn assert_deserialize_mapping() {
        let mapping: UserMapping =
            serde_json::from_str("\"user.project.portal:local_user:local_project\"")
                .unwrap_or_default();
        assert_eq!(
            mapping.to_string(),
            "user.project.portal:local_user:local_project"
        );
    }

    #[test]
    fn assert_serialize_instruction() {
        let user = UserIdentifier::parse("user.project.portal").unwrap_or_default();
        let mapping = UserMapping::new(&user, "local_user", "local_project").unwrap_or_default();

        let instruction = Instruction::AddUser(user.clone());
        let serialized = serde_json::to_string(&instruction).unwrap_or_default();
        assert_eq!(serialized, "\"add_user user.project.portal\"");

        let instruction = Instruction::RemoveUser(user.clone());
        let serialized = serde_json::to_string(&instruction).unwrap_or_default();
        assert_eq!(serialized, "\"remove_user user.project.portal\"");

        let instruction = Instruction::AddLocalUser(mapping.clone());
        let serialized = serde_json::to_string(&instruction).unwrap_or_default();
        assert_eq!(
            serialized,
            "\"add_local_user user.project.portal:local_user:local_project\""
        );

        let instruction = Instruction::RemoveLocalUser(mapping.clone());
        let serialized = serde_json::to_string(&instruction).unwrap_or_default();
        assert_eq!(
            serialized,
            "\"remove_local_user user.project.portal:local_user:local_project\""
        );

        let instruction = Instruction::UpdateHomeDir(user.clone(), "/home/user".to_string());
        let serialized = serde_json::to_string(&instruction).unwrap_or_default();
        assert_eq!(
            serialized,
            "\"update_homedir user.project.portal /home/user\""
        );

        let instruction = Instruction::Invalid();
        let serialized = serde_json::to_string(&instruction).unwrap_or_default();
        assert_eq!(serialized, "\"invalid\"");
    }

    #[test]
    fn assert_deserialize_instruction() {
        let user = UserIdentifier::parse("user.project.portal").unwrap_or_default();
        let mapping = UserMapping::new(&user, "local_user", "local_project").unwrap_or_default();

        let instruction: Instruction =
            serde_json::from_str("\"add_user user.project.portal\"").unwrap_or_default();
        assert_eq!(instruction, Instruction::AddUser(user.clone()));

        let instruction: Instruction =
            serde_json::from_str("\"remove_user user.project.portal\"").unwrap_or_default();
        assert_eq!(instruction, Instruction::RemoveUser(user.clone()));

        let instruction: Instruction =
            serde_json::from_str("\"add_local_user user.project.portal:local_user:local_project\"")
                .unwrap_or_default();
        assert_eq!(instruction, Instruction::AddLocalUser(mapping.clone()));

        let instruction: Instruction = serde_json::from_str(
            "\"remove_local_user user.project.portal:local_user:local_project\"",
        )
        .unwrap_or_default();
        assert_eq!(instruction, Instruction::RemoveLocalUser(mapping.clone()));

        let instruction: Instruction =
            serde_json::from_str("\"update_homedir user.project.portal /home/user\"")
                .unwrap_or_default();
        assert_eq!(
            instruction,
            Instruction::UpdateHomeDir(user.clone(), "/home/user".to_string())
        );

        let instruction: Instruction = serde_json::from_str("\"invalid\"").unwrap_or_default();
        assert_eq!(instruction, Instruction::Invalid());
    }
}
