use anyhow::Result;
use cdk_ansible::{HostInventoryVars, HostInventoryVarsGenerator};
use chrono::{DateTime, Utc};
use std::sync::Arc;

pub static VAR_NAME_INSTANTIATED_AT: &str = "var_instantiated_at";

pub struct HostPool {
    pub localhost: Arc<LocalHost>,
    pub host_a: Arc<HostA>,
}

impl HostPool {
    pub fn new() -> Self {
        Self {
            localhost: Arc::new(LocalHost::new()),
            host_a: Arc::new(HostA::new()),
        }
    }
}

pub struct LocalHost {
    common_field: CommonField,
}

impl LocalHost {
    pub fn new() -> Self {
        Self {
            common_field: CommonField {
                name: "localhost".into(),
                instantiated_at: Utc::now(),
            },
        }
    }
}

impl Host for LocalHost {
    fn common_field(&self) -> &CommonField {
        &self.common_field
    }
}

impl HostInventoryVarsGenerator for LocalHost {
    fn gen_host_vars(&self) -> Result<HostInventoryVars> {
        Ok(HostInventoryVars {
            ansible_host: self.common_field.name.clone(),
            inventory_vars: vec![
                ("ansible_connection".to_string(), "local".into()),
                (
                    VAR_NAME_INSTANTIATED_AT.to_string(),
                    self.common_field.instantiated_at.to_rfc3339().into(),
                ),
            ],
        })
    }
}

pub struct HostA {
    common_field: CommonField,
}

impl HostA {
    pub fn new() -> Self {
        Self {
            common_field: CommonField {
                name: "host_a".into(),
                instantiated_at: Utc::now(),
            },
        }
    }
}

impl Host for HostA {
    fn common_field(&self) -> &CommonField {
        &self.common_field
    }
}

impl HostInventoryVarsGenerator for HostA {
    fn gen_host_vars(&self) -> Result<HostInventoryVars> {
        Ok(HostInventoryVars {
            ansible_host: self.common_field.name.clone(),
            inventory_vars: vec![
                // If you want to connect to a remote host, you can change "local" to "ssh" or remove this line.
                ("ansible_connection".to_string(), "local".into()),
                (
                    VAR_NAME_INSTANTIATED_AT.to_string(),
                    self.common_field.instantiated_at.to_rfc3339().into(),
                ),
            ],
        })
    }
}

pub struct CommonField {
    pub name: String,
    pub instantiated_at: DateTime<Utc>,
}

pub trait Host: Send + Sync {
    fn common_field(&self) -> &CommonField;
}
