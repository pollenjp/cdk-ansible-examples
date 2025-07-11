mod inventory;
use inventory::*;
mod stack;
use stack::*;

use ::anyhow::Result;
use ::cdk_ansible::App;
use std::cell::RefCell;
use std::rc::Rc;

#[inline]
pub fn run() -> Result<()> {
    let host_pool = HostPool {
        localhost: LocalHost {
            name: "localhost".into(),
        },
        host_a: Rc::new(HostA {
            name: "host-a".into(),
            fqdn: "host-a.example.com".into(),
        }),
        host_b: RefCell::new(HostB {
            name: "host-b".into(),
            fqdn: "host-b.example.com".into(),
        }),
    };

    let mut app = App::new(std::env::args().collect());
    app.add_inventory(host_pool.to_inventory()?)?;
    app.add_stack(Box::new(SampleStack::new(&host_pool)))?;
    app.run()
}
