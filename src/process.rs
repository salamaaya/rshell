use std::process::Command;

pub struct Process {
    pub cmd: String,
    pub args: Vec<String>,
}

static BUILTINS: [&str; 5] = ["cd", "exit", "pwd", "echo", "clear"];

/*
 * possible return values:
 *   1: exit
 *   0: continue
 */
pub fn run_cmd(proc: Process) -> i32 {
    if BUILTINS.contains(&proc.cmd.as_str()) {
        return builtin(proc);
    }

    let cmd = proc.cmd;
    let spawn = Command::new(cmd.clone()).args(proc.args).spawn();
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

fn builtin(proc: Process) -> i32 {
    let cmd = proc.cmd;
    if cmd == "exit" {
        return 1;
    }
    println!("TODO!");
    return 0;
}
