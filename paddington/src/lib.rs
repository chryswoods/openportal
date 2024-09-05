// SPDX-FileCopyrightText: © 2024 Christopher Woods <Christopher.Woods@bristol.ac.uk>
// SPDX-License-Identifier: MIT

// internal API
mod client;
mod connection;
mod crypto;
mod eventloop;
mod exchange;
mod server;

// public API
pub mod command;
pub mod config;
pub use crypto::{Error as CryptoError, Key, SecretKey, Signature};
pub use eventloop::run;
pub use exchange::send;
pub use exchange::set_handler;
pub use exchange::Error;
pub mod invite;
pub mod message;
