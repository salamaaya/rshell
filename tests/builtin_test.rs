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

    run_builtin(&mut stdout, &proc);
    assert_eq!(stdout, b" \n");
}

#[test]
fn echo_basic() {
    let mut args = Vec::new();
    let mut stdout = Vec::new();

    args.push("hello".to_string());
    args.push("world".to_string());
    let proc = Process {
        cmd: "echo".to_string(),
        args: args,
    };

    run_builtin(&mut stdout, &proc);
    assert_eq!(stdout, b"hello world \n");
}

#[test]
fn echo_no_newline() {
    let mut args = Vec::new();
    let mut stdout = Vec::new();

    args.push("-n".to_string());
    args.push("hello".to_string());
    args.push("world".to_string());
    let proc = Process {
        cmd: "echo".to_string(),
        args: args,
    };

    run_builtin(&mut stdout, &proc);
    assert_eq!(stdout, b"hello world ");
}

#[test]
fn echo_newline_after_args() {
    let mut args = Vec::new();
    let mut stdout = Vec::new();

    args.push("hello".to_string());
    args.push("-n".to_string());
    args.push("world".to_string());
    let proc = Process {
        cmd: "echo".to_string(),
        args: args,
    };

    run_builtin(&mut stdout, &proc);
    assert_eq!(stdout, b"hello -n world \n");
}

#[test]
fn echo_no_newline_escape() {
    let mut args = Vec::new();
    let mut stdout = Vec::new();

    args.push("-ne".to_string());
    args.push("hello\nworld".to_string());
    let proc = Process {
        cmd: "echo".to_string(),
        args: args,
    };

    run_builtin(&mut stdout, &proc);
    assert_eq!(stdout, b"hello\nworld ");
}

#[test]
fn echo_escapes() {
    let mut args = Vec::new();
    let mut stdout = Vec::new();

    args.push("-e".to_string());
    args.push("hello\\nworld".to_string());
    args.push("bye".to_string());
    let proc = Process {
        cmd: "echo".to_string(),
        args: args,
    };

    run_builtin(&mut stdout, &proc);
    assert_eq!(stdout, b"hello\nworld bye \n");
}

#[test]
fn echo_disable_escapes() {
    let mut args = Vec::new();
    let mut stdout = Vec::new();

    args.push("-E".to_string());
    args.push("hello\\nworld".to_string());
    let proc = Process {
        cmd: "echo".to_string(),
        args: args,
    };

    run_builtin(&mut stdout, &proc);
    assert_eq!(stdout, b"hello\\nworld \n");
}

#[test]
fn echo_escape_sequence() {
    let mut args = Vec::new();
    let mut stdout = Vec::new();

    args.push("-e".to_string());
    args.push("\\ \\a \\b \\e \\f \\n \\r \\t \\v".to_string());
    let proc = Process {
        cmd: "echo".to_string(),
        args: args,
    };

    run_builtin(&mut stdout, &proc);
    assert_eq!(stdout, b"\\ \x07 \x08 \x1B \x0C \n \r \t \x0B \n".to_vec());
}

#[test]
fn echo_backslash_c() {
    let mut args = Vec::new();
    let mut stdout = Vec::new();

    args.push("-e".to_string());
    args.push("hello\\cworld".to_string());
    let proc = Process {
        cmd: "echo".to_string(),
        args: args,
    };

    run_builtin(&mut stdout, &proc);
    assert_eq!(stdout, b"hello\n");
}

#[test]
fn echo_octal() {
    let mut args = Vec::new();
    let mut stdout = Vec::new();

    args.push("-e".to_string());
    args.push("\\0111\\012end".to_string());
    let proc = Process {
        cmd: "echo".to_string(),
        args: args,
    };

    run_builtin(&mut stdout, &proc);
    assert_eq!(stdout, b"I\nend \n");
}

#[test]
fn echo_hex() {
    let mut args = Vec::new();
    let mut stdout = Vec::new();

    args.push("-e".to_string());
    args.push("\\x41\\012end".to_string());
    let proc = Process {
        cmd: "echo".to_string(),
        args: args,
    };

    run_builtin(&mut stdout, &proc);
    assert_eq!(stdout, b"A\nend \n");
}
