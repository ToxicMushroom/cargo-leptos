#[cfg(test)]
mod tests;

mod assets;
mod change;
mod front;
mod hash;
mod sass;
mod server;
mod style;
mod tailwind;

use std::path::Path;

pub use assets::assets;
use camino::Utf8PathBuf;
pub use change::{Change, ChangeSet};
pub use front::{front, front_cargo_process, front_cargo_process_with_args};
pub use hash::add_hashes_to_site;
pub use server::{server, server_cargo_process, server_cargo_process_with_args};
pub use style::style;

use itertools::Itertools;
use tokio::{
    fs::File, io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, process::{ChildStdout, Command}, task::JoinHandle
};
use tracing::debug;

fn build_cargo_command_string(command: &Command) -> String {
    let std_command = command.as_std();
    let program = std_command.get_program();
    let args = std_command.get_args();

    [program]
        .into_iter()
        .chain(args)
        .map(|arg| match arg.to_string_lossy() {
            arg if arg.contains(' ') => format!("'{arg}'"),
            arg => arg.into_owned(),
        })
        .join(" ")
}

pub fn spawn_cargo_log_writer(
    stdout: ChildStdout,
    stdout_file: Option<Utf8PathBuf>,
    target_dir: Option<String>,
) -> JoinHandle<std::io::Result<()>> {
    tokio::spawn({
        async move {
            let Some(stdout_file) = stdout_file.clone() else {
                return Ok(());
            };
            let stdout_file_base = target_dir.clone().unwrap_or("target".to_string());
            let stdout_file_path = Path::new(stdout_file_base.as_str()).join(stdout_file.as_str());
            let mut file = File::create(&stdout_file_path).await?;
            let stdout = stdout;
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            debug!("CARGO PIPING STDOUT TO: {:?}", &stdout_file_path);
            
            while reader.read_line(&mut line).await? > 0 {
                file.write_all(line.as_bytes()).await?;
                file.flush().await?;
                line.clear();
            }
            Ok(())
        }
    })
}
