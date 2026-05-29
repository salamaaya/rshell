use rshell::builtin::run_builtin;
use rshell::process::Process;

#[test]
fn echo_empty() {
    let mut args = Vec::new();
    let mut stdout = Vec::new();

    args.push("".to_string());
    let proc = Process {
        cmd: "echo".to_string(),
        args: args,
    };

    let result = run_builtin(&mut stdout, &proc);
    assert_eq!(stdout, b" \n");
    assert_eq!(result, 0);
}
