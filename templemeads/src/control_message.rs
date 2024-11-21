// SPDX-FileCopyrightText: © 2024 Christopher Woods <Christopher.Woods@bristol.ac.uk>
// SPDX-License-Identifier: MIT

use crate::agent::{Peer, Type as AgentType};
use crate::command::Command;
use crate::error::Error;
use crate::job;

use anyhow::Result;
use paddington::command::Command as ControlCommand;

pub async fn process_control_message(
    agent_type: &AgentType,
    command: ControlCommand,
) -> Result<(), Error> {
    match command {
        ControlCommand::Connected { agent, zone } => {
            let peer = Peer::new(&agent, &zone);
            tracing::info!("Connected to agent: {}", peer);
            Command::register(agent_type).send_to(&peer).await?;

            // now send the current board to the peer, so that they
            // can restore their state
            job::sync_board(&peer).await?;

            // now they have their new state, we need to send all of the
            // queued jobs for this peer
            job::send_queued(&peer).await?;
        }
        ControlCommand::Disconnected { agent, zone } => {
            let peer = Peer::new(&agent, &zone);
            tracing::info!("Disconnected from agent: {}", peer);
        }
        ControlCommand::Error { error } => {
            tracing::error!("Received error: {}", error);
        }
    }

    Ok(())
}
