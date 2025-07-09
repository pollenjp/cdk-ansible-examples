use anyhow::Result;
use cdk_ansible::{
    AllInventoryVarsGen, HostInventoryVars, HostInventoryVarsGenerator, Inventory, InventoryChild,
    InventoryRoot, OptU,
};
use std::cell::RefCell;
use std::rc::Rc;

/// `HostPool` is not a required struct, but it is recommended to use it to generate an inventory file.
/// cdk-ansible supply some utility functions to generate an inventory file.
#[derive(AllInventoryVarsGen)]
pub struct HostPool {
    pub localhost: LocalHost,
    // Rc or RefCell also works
    pub host_a: Rc<HostA>,
    pub host_b: RefCell<HostB>,
}

impl HostPool {
    pub fn to_inventory(&self) -> Result<Inventory> {
        Ok(Inventory {
            name: "dev".into(), // generate 'dev.yaml' file
            root: InventoryRoot {
                all: InventoryChild {
                    hosts: OptU::Some(self.inventory_vars()?.into_iter().collect()),
                    ..Default::default()
                },
            },
        })
    }
}

pub struct LocalHost {
    pub name: String,
}

impl HostInventoryVarsGenerator for LocalHost {
    fn gen_host_vars(&self) -> Result<HostInventoryVars> {
        Ok(HostInventoryVars {
            ansible_host: self.name.clone(),
            inventory_vars: vec![],
        })
    }
}

pub struct HostA {
    pub name: String,
    pub fqdn: String,
}

impl HostInventoryVarsGenerator for HostA {
    fn gen_host_vars(&self) -> Result<HostInventoryVars> {
        Ok(HostInventoryVars {
            ansible_host: self.fqdn.clone(),
            inventory_vars: vec![],
        })
    }
}

pub struct HostB {
    pub name: String,
    pub fqdn: String,
}

impl HostInventoryVarsGenerator for HostB {
    fn gen_host_vars(&self) -> Result<HostInventoryVars> {
        Ok(HostInventoryVars {
            ansible_host: self.fqdn.clone(),
            inventory_vars: vec![],
        })
    }
}
