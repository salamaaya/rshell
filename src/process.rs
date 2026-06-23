use crate::builtin::is_builtin;
use crate::builtin::run_builtin;

use std::fs::File;
use std::io;
use std::io::pipe;
use std::os::fd::OwnedFd;
use std::process::{Command, ExitStatus, Stdio};

#[derive(Debug, Clone)]
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

pub fn run_cmd_pipe(procs: &[Process]) -> Result<ExitStatus, String> {
    let num_procs = procs.len();
    let mut pipes = Vec::new();
    let mut children = Vec::new();
    let mut result = ExitStatus::default();

    for _ in 0..num_procs - 1 {
        pipes.push(pipe().map_err(|e| e.to_string())?);
    }

    for i in 0..num_procs {
        let stdin = if i == 0 {
            Stdio::inherit()
        } else {
            Stdio::from(pipes[i - 1].0.try_clone().map_err(|e| e.to_string())?)
        };

        let stdout = if i == num_procs - 1 {
            Stdio::inherit()
        } else {
            Stdio::from(pipes[i].1.try_clone().map_err(|e| e.to_string())?)
        };

        let child = Command::new(&procs[i].cmd)
            .args(&procs[i].args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn()
            .map_err(|e| e.to_string())?;

        children.push(child);
    }

    drop(pipes);

    for mut child in children {
        result = child.wait().map_err(|e| e.to_string())?;
    }

    Ok(result)
}

pub fn run_cmd_redirect_input(proc: &Process, input: &String) -> Result<ExitStatus, String> {
    let file = match File::open(input) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };
    let stdin = Stdio::from(OwnedFd::from(file));

    let mut child = Command::new(&proc.cmd)
        .args(&proc.args)
        .stdin(stdin)
        .spawn()
        .map_err(|e| e.to_string())?;
    let result = child.wait().map_err(|e| e.to_string())?;

    Ok(result)
}
