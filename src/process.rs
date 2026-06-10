use crate::builtin::is_builtin;
use crate::builtin::run_builtin;

use std::io;
use std::process::Command;
use std::process::ExitStatus;

pub struct Process {
    pub cmd: String,
    pub args: Vec<String>,
}

/*
 * possible return values:
 *   1: exit
 *   0: continue
 */
pub fn run_cmd(proc: &Process) -> Result<ExitStatus, String> {
    if is_builtin(proc.cmd.as_str()) {
        run_builtin(&mut io::stdout(), proc);
        return Ok(ExitStatus::default());
    }

    let cmd = &proc.cmd;
    let mut child = Command::new(cmd.clone())
        .args(&proc.args)
        .spawn()
        .map_err(|e| e.to_string())?;
    let result = child.wait().expect("failed to wait on child");
    Ok(result)
}
