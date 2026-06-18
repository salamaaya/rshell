use crate::builtin::is_builtin;
use crate::builtin::run_builtin;

use std::io;
use std::io::pipe;
use std::process::{Command, ExitStatus, Stdio};

pub struct Process {
    pub cmd: String,
    pub args: Vec<String>,
}

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

pub fn run_cmd_pipe(proc1: &Process, proc2: &Process) -> Result<ExitStatus, String> {
    let (reader, writer) = pipe().map_err(|e| e.to_string())?;

    let mut cmd1 = Command::new(&proc1.cmd);
    cmd1.args(&proc1.args);
    cmd1.stdout(Stdio::from(writer));

    let mut cmd2 = Command::new(&proc2.cmd);
    cmd2.args(&proc2.args);
    cmd2.stdin(Stdio::from(reader));
    cmd2.stdout(Stdio::inherit());

    let mut child1 = cmd1.spawn().map_err(|e| e.to_string())?;
    let mut child2 = cmd2.spawn().map_err(|e| e.to_string())?;

    drop(cmd1);
    drop(cmd2);

    let status2 = child2.wait().map_err(|e| e.to_string())?;
    let _ = child1.wait().map_err(|e| e.to_string())?;
    Ok(status2)
}
