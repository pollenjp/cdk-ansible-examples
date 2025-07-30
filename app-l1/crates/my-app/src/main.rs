use anyhow::Result;
use cdk_ansible::App;
use my_app::{HostA, HostPool, LocalHost, SampleStack};

fn main() {
    if let Err(e) = main2() {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    }
}

fn main2() -> Result<()> {
    let host_pool = HostPool {
        localhost: LocalHost {
            name: "localhost".into(),
        },
        host_a: HostA {
            name: "host_a".into(),
        },
    };

    let mut app = App::new(std::env::args().collect());
    app.add_inventory(host_pool.to_inventory()?)?;
    app.add_stack(Box::new(SampleStack::new(&host_pool)?))?;
    app.run()
}
