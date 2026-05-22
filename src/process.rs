use crate::builtins::is_builtin;
use crate::builtins::run_builtin;

use std::process::Command;

pub struct Process {
    pub cmd: String,
    pub args: Vec<String>,
}

/*
 * possible return values:
 *   1: exit
 *   0: continue
 */
pub fn run_cmd(proc: &Process) -> i32 {
    if is_builtin(proc.cmd.as_str()) {
        return run_builtin(proc);
    }

    let cmd = &proc.cmd;
    let spawn = Command::new(cmd.clone()).args(&proc.args).spawn();
    match spawn {
        Ok(mut child) => {
            let _result = child.wait().expect("failed to wait on child");
        }
        Err(_error) => {
            println!("failed to execute, unkown command {cmd}");
        }
    }

    0
}
