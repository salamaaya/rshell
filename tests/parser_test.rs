use rshell::lexer::lex;
use rshell::parser::build_ast;

#[test]
fn single_command_ast() {
    let input = String::from("ls");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(r#"[Command { cmd: "ls", args: [] }]"#, output);
}

#[test]
fn command_with_args_ast() {
    let input = String::from("echo hello world");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Command { cmd: "echo", args: ["hello", "world"] }]"#,
        output
    );
}

#[test]
fn multiple_commands_ast() {
    let input = String::from("pwd; ls");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Command { cmd: "pwd", args: [] }, Command { cmd: "ls", args: [] }]"#,
        output
    );
}

#[test]
fn multiple_semicolons_ast() {
    let input = String::from("pwd;;;ls");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    assert_eq!(2, ast.len());
}

#[test]
fn env_var_expansion_ast() {
    unsafe {
        std::env::set_var("TEST_VAR", "hello");
    }

    let input = String::from("echo $TEST_VAR");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(r#"[Command { cmd: "echo", args: ["hello"] }]"#, output);
}

#[test]
fn env_var_middle_of_args_ast() {
    unsafe {
        std::env::set_var("TEST_VAR", "hello");
    }

    let input = String::from("echo before $TEST_VAR after");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Command { cmd: "echo", args: ["before", "hello", "after"] }]"#,
        output
    );
}

#[test]
fn undefined_env_var_error() {
    unsafe {
        std::env::remove_var("DOES_NOT_EXIST");
    }

    let input = String::from("echo $DOES_NOT_EXIST");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn lone_dollar_ast() {
    let input = String::from("echo $");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(r#"[Command { cmd: "echo", args: ["$"] }]"#, output);
}

#[test]
fn invalid_dollar_argument() {
    let input = String::from("echo $;");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn simple_subshell_ast() {
    let input = String::from("(ls)");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(r#"[Subshell { cmd: "ls " }]"#, output);
}

#[test]
fn nested_subshell_ast() {
    let input = String::from("(ls;(pwd))");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    assert_eq!(1, ast.len());
}

#[test]
fn deeply_nested_subshell_ast() {
    let input = String::from("(((ls)))");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_ok());
}

#[test]
fn unmatched_right_paren_error() {
    let input = String::from(")");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn unmatched_left_paren_error() {
    let input = String::from("(ls");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn empty_subshell_ast() {
    let input = String::from("()");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(r#"[Subshell { cmd: "" }]"#, output);
}

#[test]
fn simple_inline_group() {
    let input = String::from("{ls}");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[InlineGroup { cmds: [Command { cmd: "ls", args: [] }] }]"#,
        output
    );
}

#[test]
fn nested_inline_group_ast() {
    let input = String::from("{ls;{pwd}}");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[InlineGroup { cmds: [Command { cmd: "ls", args: [] }, InlineGroup { cmds: [Command { cmd: "pwd", args: [] }] }] }]"#,
        output
    );
}

#[test]
fn deeply_nested_inline_group_ast() {
    let input = String::from("{{{ls}}}");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_ok());
}

#[test]
fn unmatched_right_inline_error() {
    let input = String::from("}");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn unmatched_left_inline_error() {
    let input = String::from("{ls");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn empty_inline_group_ast() {
    let input = String::from("{}");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(r#"[InlineGroup { cmds: [] }]"#, output);
}

#[test]
fn inline_group_with_multiple_commands_ast() {
    let input = String::from("{pwd; ls}");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    assert_eq!(1, ast.len());
}

#[test]
fn inline_group_inside_command_ast() {
    let input = String::from("echo {ls}");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_ok());
}

#[test]
fn env_var_inside_inline_group_ast() {
    unsafe {
        std::env::set_var("TEST_VAR", "hello");
    }

    let input = String::from("{echo $TEST_VAR}");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    assert!(format!("{:?}", ast).contains("hello"));
}

#[test]
fn undefined_env_var_inside_inline_group_error() {
    unsafe {
        std::env::remove_var("DOES_NOT_EXIST");
    }

    let input = String::from("{echo $DOES_NOT_EXIST}");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn inline_group_containing_subshell_ast() {
    let input = String::from("{(ls)}");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_ok());
}

#[test]
fn subshell_containing_inline_group_ast() {
    let input = String::from("({ls})");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_ok());
}
