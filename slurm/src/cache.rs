// SPDX-FileCopyrightText: © 2024 Christopher Woods <Christopher.Woods@bristol.ac.uk>
// SPDX-License-Identifier: MIT

use anyhow::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use templemeads::grammar::{Date, ProjectIdentifier};
use templemeads::usagereport::DailyProjectUsageReport;
use templemeads::Error;
use tokio::sync::RwLock;

use crate::slurm::{SlurmAccount, SlurmNode, SlurmNodes, SlurmUser};

#[derive(Debug, Clone, Default)]
struct UsageDatabase {
    reports: HashMap<Date, DailyProjectUsageReport>,
}

#[derive(Debug, Clone, Default)]
struct Database {
    cluster: Option<String>,
    accounts: HashMap<String, SlurmAccount>,
    users: HashMap<String, SlurmUser>,
    nodes: Option<SlurmNodes>,
    reports: HashMap<ProjectIdentifier, UsageDatabase>,
}

static CACHE: Lazy<RwLock<Database>> = Lazy::new(|| RwLock::new(Database::default()));

pub async fn get_option_cluster() -> Result<Option<String>, Error> {
    let cache = CACHE.read().await;
    Ok(cache.cluster.clone())
}

pub async fn get_cluster() -> Result<String, Error> {
    let cache = CACHE.read().await;

    match cache.cluster {
        Some(ref cluster) => Ok(cluster.clone()),
        None => Ok("linux".to_string()),
    }
}

pub async fn set_cluster(cluster: &str) -> Result<(), Error> {
    let mut cache = CACHE.write().await;
    cache.cluster = Some(cluster.to_string());
    Ok(())
}

pub async fn get_account(name: &str) -> Result<Option<SlurmAccount>, Error> {
    let cache = CACHE.read().await;
    Ok(cache.accounts.get(name).cloned())
}

pub async fn add_account(account: &SlurmAccount) -> Result<(), Error> {
    let mut cache = CACHE.write().await;
    cache
        .accounts
        .insert(account.name().to_string(), account.clone());
    Ok(())
}

pub async fn add_user(user: &SlurmUser) -> Result<(), Error> {
    let mut cache = CACHE.write().await;
    cache.users.insert(user.name().to_string(), user.clone());
    Ok(())
}

pub async fn get_user(name: &str) -> Result<Option<SlurmUser>, Error> {
    let cache = CACHE.read().await;
    Ok(cache.users.get(name).cloned())
}

pub async fn set_default_node(node: &SlurmNode) -> Result<(), Error> {
    let mut cache = CACHE.write().await;

    match cache.nodes {
        Some(ref mut nodes) => nodes.set_default(node),
        None => cache.nodes = Some(SlurmNodes::new(node)),
    }

    Ok(())
}

#[allow(dead_code)]
pub async fn set_node(name: &str, node: &SlurmNode) -> Result<(), Error> {
    let mut cache = CACHE.write().await;

    match cache.nodes {
        Some(ref mut nodes) => nodes.set(name, node),
        None => {
            let mut nodes = SlurmNodes::new(node);
            nodes.set(name, node);
            cache.nodes = Some(nodes);
        }
    }

    Ok(())
}

pub async fn get_nodes() -> Result<SlurmNodes, Error> {
    let cache = CACHE.read().await;

    match cache.nodes {
        Some(ref nodes) => Ok(nodes.clone()),
        None => Err(Error::Bug(
            "No nodes have been set in the cache".to_string(),
        )),
    }
}

pub async fn get_report(
    project: &ProjectIdentifier,
    date: &Date,
) -> Result<Option<DailyProjectUsageReport>, Error> {
    let cache = CACHE.read().await;

    match cache.reports.get(project) {
        Some(usage) => Ok(usage.reports.get(date).cloned()),
        None => Ok(None),
    }
}

pub async fn set_report(
    project: &ProjectIdentifier,
    date: &Date,
    report: &DailyProjectUsageReport,
) -> Result<(), Error> {
    let today = Date::today();

    if date > &today {
        return Err(Error::Bug(format!(
            "Cannot cache a report for project '{}' for future date: {} - {}",
            project, date, report
        )));
    }

    if !report.is_complete() {
        return Err(Error::Bug(format!(
            "Cannot cache an incomplete report for project '{}' for date: {} - {}",
            project, date, report
        )));
    }

    let mut cache = CACHE.write().await;

    match cache.reports.get_mut(project) {
        Some(usage) => {
            // delete the oldest reports while there are >= 30 reports cached
            // This ensures we only cache a maximum of 30 days of reports
            // per project
            while usage.reports.len() >= 30 {
                let mut oldest = today.clone();

                for date in usage.reports.keys() {
                    if date < &oldest {
                        oldest = date.clone();
                    }
                }

                usage.reports.remove(&oldest);
            }

            usage.reports.insert(date.clone(), report.clone());
        }
        None => {
            let mut usage = UsageDatabase::default();
            usage.reports.insert(date.clone(), report.clone());
            cache.reports.insert(project.clone(), usage);
        }
    }

    Ok(())
}

///
/// Clear the cache - we need to do this if Slurm is changed behine
/// our back
///
pub async fn clear() -> Result<(), Error> {
    let mut cache = CACHE.write().await;
    cache.accounts.clear();
    cache.users.clear();
    Ok(())
}
