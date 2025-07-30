use anyhow::Result;
use cdk_ansible::AppL2;
use my_app::SampleStack;
use std::sync::Arc;

fn main() {
    if let Err(e) = main2() {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    }
}

fn main2() -> Result<()> {
    AppL2::new(std::env::args().collect())
        .stack(Arc::new(SampleStack::new()))
        .expect("Failed to add sample stack")
        .run()
}
