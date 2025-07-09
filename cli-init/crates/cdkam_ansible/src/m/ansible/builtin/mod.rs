#[cfg(feature = "ansible-builtin-debug")]
pub mod debug;

#[cfg(feature = "ansible-builtin-command")]
pub mod command;

#[cfg(feature = "ansible-builtin-shell")]
pub mod shell;
