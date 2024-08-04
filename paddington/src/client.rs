// SPDX-FileCopyrightText: © 2024 Christopher Woods <Christopher.Woods@bristol.ac.uk>
// SPDX-License-Identifier: MIT

use anyhow::Error as AnyError;
use std::io::Error as IOError;
use thiserror::Error;

use crate::config::{PeerConfig, ServiceConfig};
use crate::connection::{Connection, ConnectionError};
use crate::crypto;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("{0}")]
    IO(#[from] IOError),

    #[error("{0}")]
    Any(#[from] AnyError),

    #[error("{0}")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::error::Error),

    #[error("{0}")]
    Crypto(#[from] crypto::CryptoError),

    #[error("{0}")]
    Connection(#[from] ConnectionError),

    #[error("{0}")]
    UnknownPeer(String),
}

pub async fn run_once(config: ServiceConfig, peer: PeerConfig) -> Result<(), ClientError> {
    let service_name = config.name.clone().unwrap_or_default();

    if service_name.is_empty() {
        return Err(ClientError::UnknownPeer(
            "Cannot connect as service must have a name".to_string(),
        ));
    }

    let peer_name = peer.name().clone().unwrap_or_default();

    if peer_name.is_empty() {
        return Err(ClientError::UnknownPeer(
            "Cannot connect as peer must have a name".to_string(),
        ));
    }

    tracing::info!(
        "Initiating connection: {:?} <=> {:?}",
        service_name,
        peer_name
    );

    // create a connection object to make the connection - these are
    // mutable as they hold the state of the connection in this
    // throwaway client
    let mut connection = Connection::new(config.clone());

    // this will loop until the connection is closed
    connection.make_connection(&peer).await?;

    Ok(())
}

pub async fn run(config: ServiceConfig, peer: PeerConfig) -> Result<(), ClientError> {
    loop {
        match run_once(config.clone(), peer.clone()).await {
            Ok(_) => {
                tracing::info!("Client exited successfully.");
            }
            Err(e) => {
                tracing::error!("Client exited with error: {:?}", e);
            }
        }

        // sleep for a bit before trying again
        tracing::info!("Sleeping for 5 seconds before retrying the connection...");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
