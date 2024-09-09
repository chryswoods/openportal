// SPDX-FileCopyrightText: © 2024 Christopher Woods <Christopher.Woods@bristol.ac.uk>
// SPDX-License-Identifier: MIT

use crate::destination::Destination;
use crate::grammar::Instruction;

use anyhow::Error as AnyError;
use anyhow::Result;
use thiserror::Error;

use chrono::serde::ts_seconds;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Pending,
    Complete,
    Error,
}

///
/// This is the internal representation of the parsed command. We don't
/// make this publicly visible as we don't want to confuse users with too
/// many "command" types.
///
#[derive(Clone, PartialEq)]
struct Command {
    destination: Destination,
    instruction: Instruction,
}

impl Command {
    pub fn new(command: &str) -> Self {
        // the format of commands is "destination command arguments..."
        let mut parts = command.split_whitespace();
        let destination = Destination::new(parts.next().unwrap_or(""));
        let instruction = Instruction::new(&parts.collect::<Vec<&str>>().join(" "));

        Self {
            destination,
            instruction,
        }
    }

    pub fn destination(&self) -> Destination {
        self.destination.clone()
    }

    pub fn instruction(&self) -> Instruction {
        self.instruction.clone()
    }

    pub fn is_valid(&self) -> bool {
        self.destination.is_valid() && self.instruction.is_valid()
    }
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.destination, self.instruction)
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.destination, self.instruction,)
    }
}

// serialise via the string representation - this looks better
impl Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

// deserialise via the string representation - this looks better

impl<'de> Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::new(&s))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Job {
    id: Uuid,
    #[serde(with = "ts_seconds")]
    created: chrono::DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    updated: chrono::DateTime<Utc>,
    version: u64,
    command: Command,
    state: Status,
    result: Option<String>,
}

impl Job {
    pub fn new(command: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created: now,
            updated: now,
            version: 1,
            command: Command::new(command),
            state: Status::Pending,
            result: None,
        }
    }

    pub fn parse(command: &str) -> Result<Self, Error> {
        let job = Self::new(command);

        if !job.command.is_valid() {
            return Err(Error::Parse("Invalid command".to_owned()));
        }

        Ok(job)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn destination(&self) -> Destination {
        self.command.destination()
    }

    pub fn instruction(&self) -> Instruction {
        self.command.instruction()
    }

    pub fn state(&self) -> Status {
        self.state.clone()
    }

    pub fn created(&self) -> chrono::DateTime<Utc> {
        self.created
    }

    pub fn updated(&self) -> chrono::DateTime<Utc> {
        self.updated
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn completed<T>(&mut self, result: T) -> Result<(), Error>
    where
        T: serde::Serialize,
    {
        if self.state != Status::Pending {
            return Err(Error::InvalidState(
                "Cannot set result on non-pending job".to_owned(),
            ));
        }

        self.state = Status::Complete;
        self.result = Some(serde_json::to_string(&result)?);
        self.updated = Utc::now();
        self.version += 1;

        Ok(())
    }

    pub fn errored(&mut self, message: &str) -> Result<(), Error> {
        if self.state != Status::Pending {
            return Err(Error::InvalidState(
                "Cannot set error on non-pending job".to_owned(),
            ));
        }

        self.state = Status::Error;
        self.result = Some(message.to_owned());
        self.updated = Utc::now();
        self.version += 1;

        Ok(())
    }

    pub fn result<T>(&self) -> Result<Option<T>, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        match self.state {
            Status::Pending => Ok(None),
            Status::Error => match &self.result {
                Some(result) => Err(Error::RunError(result.clone())),
                None => Err(Error::InvalidState("Unknown error".to_owned())),
            },
            Status::Complete => match &self.result {
                Some(result) => Ok(Some(serde_json::from_str(result)?)),
                None => Err(Error::Unknown("No result available".to_owned())),
            },
        }
    }

    pub async fn execute(&self) -> Result<Job, Error> {
        // execute the command
        tracing::info!("Running job.execute() for job: {:?}", self);

        let mut job = self.clone();

        job.completed("Hello, World! - we have run!")?;

        Ok(job)
    }
}

// Errors

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    AnyError(#[from] AnyError),

    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("{0}")]
    RunError(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("{0}")]
    Parse(String),

    #[error("{0}")]
    Unknown(String),
}
