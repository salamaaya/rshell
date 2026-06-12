use rshell::lexer::lex;
use rshell::parser::build_ast;

#[test]
fn single_command_ast() {
    let input = String::from("ls");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(r#"[Command { program: "ls", args: [] }]"#, output);
}

#[test]
fn command_with_args_ast() {
    let input = String::from("echo hello world");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Command { program: "echo", args: ["hello", "world"] }]"#,
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
        r#"[Command { program: "pwd", args: [] }, Command { program: "ls", args: [] }]"#,
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

    assert_eq!(r#"[Command { program: "echo", args: ["hello"] }]"#, output);
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
        r#"[Command { program: "echo", args: ["before", "hello", "after"] }]"#,
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

    assert_eq!(r#"[Command { program: "echo", args: ["$"] }]"#, output);
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

    assert_eq!(r#"[Subshell { command: "ls " }]"#, output);
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

    assert_eq!(r#"[Subshell { command: "" }]"#, output);
}
