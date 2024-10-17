// SPDX-FileCopyrightText: © 2024 Christopher Woods <Christopher.Woods@bristol.ac.uk>
// SPDX-License-Identifier: MIT

use crate::agent_core::Config;
use crate::error::Error;
use crate::handler::{process_message, set_service_details};
use crate::runnable::AsyncRunnable;

///
/// Run the filesystem service
///
pub async fn run(config: Config, runner: AsyncRunnable) -> Result<(), Error> {
    if config.service().name().is_empty() {
        return Err(Error::Misconfigured("Service name is empty".to_string()));
    }

    // pass the service details onto the handler
    set_service_details(&config.service().name(), &config.agent(), Some(runner)).await?;

    // run the Provider OpenPortal agent
    paddington::set_handler(process_message).await?;
    paddington::run(config.service()).await?;

    Ok(())
}
