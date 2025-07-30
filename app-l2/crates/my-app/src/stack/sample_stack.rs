use crate::inventory::{Host, HostPool, VAR_NAME_INSTANTIATED_AT};
use anyhow::Result;
use cdk_ansible::{
    ExePlayL2, HostsL2, LazyPlayL2, OptU, PlayL2, PlayOptions, StackL2, Task, TaskOptions,
};
use futures::future::{BoxFuture, FutureExt as _};
use std::sync::Arc;

pub struct SampleStack {
    exe_play: ExePlayL2,
}

impl SampleStack {
    pub fn new() -> Self {
        Self {
            // exe_play: ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample"))),
            exe_play: ExePlayL2::Sequential(vec![
                ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample1"))),
                ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample2"))),
                ExePlayL2::Parallel(vec![
                    ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample3"))),
                    ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample4"))),
                ]),
            ]),
        }
    }
}

impl StackL2 for SampleStack {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .expect("Failed to get a stack name")
    }
    fn exe_play(&self) -> &ExePlayL2 {
        &self.exe_play
    }
}

struct SampleLazyPlayL2Helper {
    name: String,
}

impl SampleLazyPlayL2Helper {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl LazyPlayL2 for SampleLazyPlayL2Helper {
    fn create_play_l2(&self) -> BoxFuture<'static, Result<PlayL2>> {
        let name = self.name.clone();
        async move {
            let hp = HostPool::new(); // Each hosts are instantiated here!!
            Ok(PlayL2 {
                name,
                hosts: HostsL2::new(vec![Arc::clone(&hp.localhost) as _]),
                options: PlayOptions::default(),
                tasks: create_tasks_helper(Arc::clone(&hp.localhost) as _, 2)?,
            })
        }
        .boxed()
    }
}

fn create_tasks_helper(h: Arc<dyn Host>, n: usize) -> Result<Vec<Task>> {
    let mut tasks = vec![::cdk_ansible::Task {
        name: "debug".into(),
        options: TaskOptions::default(),
        command: Box::new(::cdkam::ansible::builtin::debug::Module {
            module: ::cdkam::ansible::builtin::debug::Args {
                options: ::cdkam::ansible::builtin::debug::Opt {
                    msg: OptU::Some(format!(
                        "Hello '{}'! Instantiated at '{{{{ {} | default('N/A') }}}}'",
                        h.common_field().name.clone(),
                        VAR_NAME_INSTANTIATED_AT
                    )),
                    ..Default::default()
                },
            },
        }),
    }];

    // Don't sleep in CI
    if std::env::var("CI_JOB").is_err() {
        tasks.extend((0..n).map(|_| ::cdk_ansible::Task {
            name: "sleep".into(),
            options: TaskOptions {
                changed_when: OptU::Some(false.into()),
                ..Default::default()
            },
            command: Box::new(::cdkam::ansible::builtin::command::Module {
                module: ::cdkam::ansible::builtin::command::Args {
                    options: ::cdkam::ansible::builtin::command::Opt {
                        cmd: OptU::Some("sleep 3".into()),
                        ..Default::default()
                    },
                },
            }),
        }));
    }

    Ok(tasks)
}
