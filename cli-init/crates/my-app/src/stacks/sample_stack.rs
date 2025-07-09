use crate::inventory::HostPool;
use ::cdk_ansible::{
    DeployStack, ExeParallel, ExePlay, ExeSequential, ExeSingle, OptU, Play, PlayOptions,
    StringOrVecString, TaskOptions,
};

pub struct SampleStack {
    exe_play: ExePlay,
}

impl SampleStack {
    pub fn new(hp: &HostPool) -> Self {
        let hosts = hp.localhost.name.as_str();

        Self {
            exe_play: ExeSequential(vec![
                // ExeSingle(create_play_helper("sample1", hosts.into())),
                // ExeSingle(create_play_helper("sample2", hosts.into())),
                ExeParallel(vec![
                    ExeParallel(vec![
                        ExeSingle(create_play_helper("sample0", hosts.into(), 5)),
                        ExeSingle(create_play_helper("sample1", hosts.into(), 10)),
                        ExeSingle(create_play_helper("sample2", hosts.into(), 15)),
                    ]),
                    ExeSequential(vec![
                        ExeSingle(create_play_helper("sample3", hosts.into(), 1)),
                        ExeSingle(create_play_helper("sample4", hosts.into(), 1)),
                        ExeSingle(create_play_helper("sample5", hosts.into(), 1)),
                    ]),
                    ExeSingle(create_play_helper("sample6", hosts.into(), 1)),
                ]),
            ]),
        }
    }
}

impl DeployStack for SampleStack {
    /// TODO: May be converted to derive macro in the future
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

fn create_play_helper(name: &str, hosts: StringOrVecString, n: usize) -> Box<Play> {
    let mut tasks = vec![::cdk_ansible::Task {
        name: "debug".into(),
        options: TaskOptions::default(),
        command: Box::new(::cdkam_ansible::builtin::debug::Module {
            module: ::cdkam_ansible::builtin::debug::Args {
                options: ::cdkam_ansible::builtin::debug::Opt {
                    msg: OptU::Some("Hello, world!".into()),
                    ..Default::default()
                },
            },
        }),
    }];

    // Don't sleep in CI
    if std::env::var("CI_JOB").is_err() {
        tasks.extend((0..n).map(|_| ::cdk_ansible::Task {
            name: "sleep 2 seconds".into(),
            options: TaskOptions {
                changed_when: OptU::Some(false.into()),
                ..Default::default()
            },
            command: Box::new(::cdkam_ansible::builtin::command::Module {
                module: ::cdkam_ansible::builtin::command::Args {
                    options: ::cdkam_ansible::builtin::command::Opt {
                        cmd: OptU::Some("sleep 3".into()),
                        ..Default::default()
                    },
                },
            }),
        }));
    }

    // tasks.push(::cdk_ansible::Task {
    //     name: "interrupt play".into(),
    //     options: TaskOptions {
    //         changed_when: OptU::Some(false.into()),
    //         ..Default::default()
    //     },
    //     command: Box::new(::sample_cdkam_ansible::builtin::shell::Module {
    //         module: ::sample_cdkam_ansible::builtin::shell::Args {
    //             options: ::sample_cdkam_ansible::builtin::shell::Opt {
    //                 cmd: OptU::Some("exit 1".into()),
    //                 ..Default::default()
    //             },
    //         },
    //     }),
    // });

    Box::new(Play {
        name: name.into(),
        hosts,
        options: PlayOptions::default(),
        tasks,
    })
}
