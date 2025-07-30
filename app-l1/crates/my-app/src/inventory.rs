use anyhow::Result;
use cdk_ansible::{
    AllInventoryVarsGen, HostInventoryVars, HostInventoryVarsGenerator, Inventory, InventoryChild,
    InventoryRoot, OptU,
};

#[derive(AllInventoryVarsGen)]
pub struct HostPool {
    pub localhost: LocalHost,
    pub host_a: HostA,
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
            inventory_vars: vec![("ansible_connection".into(), "local".into())],
        })
    }
}

pub struct HostA {
    pub name: String,
}

impl HostInventoryVarsGenerator for HostA {
    fn gen_host_vars(&self) -> Result<HostInventoryVars> {
        Ok(HostInventoryVars {
            ansible_host: self.name.clone(),
            inventory_vars: vec![],
        })
    }
}
