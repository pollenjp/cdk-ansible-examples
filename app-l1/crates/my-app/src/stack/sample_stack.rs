use crate::inventory::HostPool;
use anyhow::Result;
use cdk_ansible::{
    ExeParallel, ExePlay, ExeSequential, ExeSingle, OptU, Play, PlayOptions, Stack,
    StringOrVecString, Task, TaskOptions,
};

pub struct SampleStack {
    exe_play: ExePlay,
}

impl SampleStack {
    pub fn new(hp: &HostPool) -> Result<Self> {
        let hosts = hp.localhost.name.as_str();

        Ok(Self {
            exe_play: ExeSequential(vec![
                ExeParallel(vec![
                    ExeParallel(vec![
                        ExeSingle(create_play_helper("sample0", hosts.into(), 1)?.into()),
                        ExeSingle(create_play_helper("sample1", hosts.into(), 10)?.into()),
                        ExeSingle(create_play_helper("sample2", hosts.into(), 15)?.into()),
                    ]),
                    ExeSequential(vec![
                        ExeSingle(create_play_helper("sample3", hosts.into(), 1)?.into()),
                        ExeSingle(create_play_helper("sample4", hosts.into(), 1)?.into()),
                        ExeSingle(create_play_helper("sample5", hosts.into(), 1)?.into()),
                    ]),
                    ExeSingle(create_play_helper("sample6", hosts.into(), 1)?.into()),
                ]),
                ExeSequential(vec![
                    ExeSingle(create_play_helper("sample7", hosts.into(), 1)?.into()),
                    ExeSingle(create_play_helper("sample8", hosts.into(), 1)?.into()),
                    ExeSingle(create_play_helper("sample9", hosts.into(), 1)?.into()),
                ]),
            ]),
        })
    }
}

impl Stack for SampleStack {
    #[expect(clippy::expect_used, reason = "Logical failure")]
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .expect("Failed to get a stack name")
    }

    fn exe_play(&self) -> &ExePlay {
        &self.exe_play
    }
}

fn create_play_helper(name: &str, hosts: StringOrVecString, n: usize) -> Result<Play> {
    Ok(Play {
        name: name.into(),
        hosts,
        options: PlayOptions::default(),
        tasks: create_tasks_helper(n)?,
    })
}

fn create_tasks_helper(n: usize) -> Result<Vec<Task>> {
    let mut tasks = vec![::cdk_ansible::Task {
        name: "debug".into(),
        options: TaskOptions::default(),
        command: Box::new(::cdkam::ansible::builtin::debug::Module {
            module: ::cdkam::ansible::builtin::debug::Args {
                options: ::cdkam::ansible::builtin::debug::Opt {
                    msg: OptU::Some("Hello, world!".into()),
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
