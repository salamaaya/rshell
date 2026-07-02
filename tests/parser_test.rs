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

#[test]
fn multiple_pipes_ast() {
    let input = String::from("a | b | c | d");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_ok());
}

#[test]
fn pipe_followed_by_command_ast() {
    let input = String::from("ls | wc; pwd");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    assert_eq!(2, ast.len());
}

#[test]
fn command_before_pipe_ast() {
    let input = String::from("pwd; ls | wc");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    assert_eq!(2, ast.len());
}

#[test]
fn undefined_env_var_inside_pipe_error() {
    unsafe {
        std::env::remove_var("DOES_NOT_EXIST");
    }

    let input = String::from("echo $DOES_NOT_EXIST | wc");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn subshell_in_middle_of_pipe_ast() {
    let input = String::from("echo hi | (cat) | wc");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_ok());
}

#[test]
fn inline_group_in_middle_of_pipe_ast() {
    let input = String::from("echo hi | {cat} | wc");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_ok());
}

#[test]
fn pipe_inside_subshell_ast() {
    let input = String::from("(ls | wc)");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_ok());
}

#[test]
fn deeply_nested_pipe_ast() {
    let input = String::from("({{ls | grep rs}}) | {wc}");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_ok());
}

#[test]
fn leading_pipe_error() {
    let input = String::from("| ls");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn trailing_pipe_error() {
    let input = String::from("ls |");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn empty_pipe_segment_error() {
    let input = String::from("ls | | wc");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn pipe_with_empty_subshell_ast() {
    let input = String::from("() | wc");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_ok());
}

#[test]
fn pipe_with_empty_inline_group_ast() {
    let input = String::from("{} | wc");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_ok());
}

#[test]
fn semicolon_inside_pipe_segment_error() {
    let input = String::from("ls; pwd | wc");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_ok());
}

#[test]
fn simple_pipe_ast() {
    let input = String::from("ls | wc");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: Pipe, cmds: [Command { cmd: "ls", args: [] }, Command { cmd: "wc", args: [] }], file: "" }]"#,
        output
    );
}

#[test]
fn pipe_with_arguments_ast() {
    let input = String::from("echo hello | grep hello");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: Pipe, cmds: [Command { cmd: "echo", args: ["hello"] }, Command { cmd: "grep", args: ["hello"] }], file: "" }]"#,
        output
    );
}

#[test]
fn three_command_pipe_ast() {
    let input = String::from("cat file | grep test | wc");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: Pipe, cmds: [Command { cmd: "cat", args: ["file"] }, Command { cmd: "grep", args: ["test"] }, Command { cmd: "wc", args: [] }], file: "" }]"#,
        output
    );
}

#[test]
fn env_var_inside_pipe_ast() {
    unsafe {
        std::env::set_var("TEST_VAR", "hello");
    }

    let input = String::from("echo $TEST_VAR | wc");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: Pipe, cmds: [Command { cmd: "echo", args: ["hello"] }, Command { cmd: "wc", args: [] }], file: "" }]"#,
        output
    );
}

#[test]
fn subshell_on_left_side_of_pipe_ast() {
    let input = String::from("(ls) | wc");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: Pipe, cmds: [Subshell { cmd: "ls " }, Command { cmd: "wc", args: [] }], file: "" }]"#,
        output
    );
}

#[test]
fn subshell_on_right_side_of_pipe_ast() {
    let input = String::from("ls | (wc)");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: Pipe, cmds: [Command { cmd: "ls", args: [] }, Subshell { cmd: "wc " }], file: "" }]"#,
        output
    );
}

#[test]
fn inline_group_on_left_side_of_pipe_ast() {
    let input = String::from("{ls} | wc");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: Pipe, cmds: [InlineGroup { cmds: [Command { cmd: "ls", args: [] }] }, Command { cmd: "wc", args: [] }], file: "" }]"#,
        output
    );
}

#[test]
fn inline_group_on_right_side_of_pipe_ast() {
    let input = String::from("ls | {wc}");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: Pipe, cmds: [Command { cmd: "ls", args: [] }, InlineGroup { cmds: [Command { cmd: "wc", args: [] }] }], file: "" }]"#,
        output
    );
}

#[test]
fn pipe_inside_inline_group_ast() {
    let input = String::from("{ls | wc}");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[InlineGroup { cmds: [Redirect { op: Pipe, cmds: [Command { cmd: "ls", args: [] }, Command { cmd: "wc", args: [] }], file: "" }] }]"#,
        output
    );
}

#[test]
fn nested_pipe_group_subshell_ast() {
    unsafe {
        std::env::set_var("TEST_VAR", "hello");
    }

    let input = String::from("{echo $TEST_VAR | (cat)} | wc");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: Pipe, cmds: [InlineGroup { cmds: [Redirect { op: Pipe, cmds: [Command { cmd: "echo", args: ["hello"] }, Subshell { cmd: "cat " }], file: "" }] }, Command { cmd: "wc", args: [] }], file: "" }]"#,
        output
    );
}

#[test]
fn simple_input_redirect_ast() {
    let input = String::from("cat < input.txt");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: RedirectInput, cmds: [Command { cmd: "cat", args: [] }], file: "input.txt" }]"#,
        output
    );
}

#[test]
fn input_redirect_with_arguments_ast() {
    let input = String::from("grep hello world < input.txt");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: RedirectInput, cmds: [Command { cmd: "grep", args: ["hello", "world"] }], file: "input.txt" }]"#,
        output
    );
}

#[test]
fn input_redirect_between_commands_ast() {
    let input = String::from("pwd; cat < input.txt; ls");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Command { cmd: "pwd", args: [] }, Redirect { op: RedirectInput, cmds: [Command { cmd: "cat", args: [] }], file: "input.txt" }, Command { cmd: "ls", args: [] }]"#,
        output
    );
}

#[test]
fn env_var_inside_input_redirect_ast() {
    unsafe {
        std::env::set_var("TEST_VAR", "hello");
    }

    let input = String::from("grep $TEST_VAR < input.txt");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: RedirectInput, cmds: [Command { cmd: "grep", args: ["hello"] }], file: "input.txt" }]"#,
        output
    );
}

#[test]
fn subshell_input_redirect_ast() {
    let input = String::from("(cat) < input.txt");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: RedirectInput, cmds: [Subshell { cmd: "cat " }], file: "input.txt" }]"#,
        output
    );
}

#[test]
fn inline_group_input_redirect_ast() {
    let input = String::from("{cat} < input.txt");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: RedirectInput, cmds: [InlineGroup { cmds: [Command { cmd: "cat", args: [] }] }], file: "input.txt" }]"#,
        output
    );
}

#[test]
fn input_redirect_inside_inline_group_ast() {
    let input = String::from("{cat < input.txt}");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[InlineGroup { cmds: [Redirect { op: RedirectInput, cmds: [Command { cmd: "cat", args: [] }], file: "input.txt" }] }]"#,
        output
    );
}

#[test]
fn input_redirect_on_left_side_of_pipe_ast() {
    let input = String::from("cat < input.txt | wc");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: Pipe, cmds: [Redirect { op: RedirectInput, cmds: [Command { cmd: "cat", args: [] }], file: "input.txt" }, Command { cmd: "wc", args: [] }], file: "" }]"#,
        output
    );
}

#[test]
fn leading_input_redirect_error() {
    let input = String::from("< input.txt");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn invalid_input_redirect_target_error() {
    let input = String::from("cat < |");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn input_redirect_on_right_side_of_pipe_error() {
    let input = String::from("ls | cat < input.txt");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn simple_output_redirect_ast() {
    let input = String::from("echo hello > output.txt");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: RedirectOutput, cmds: [Command { cmd: "echo", args: ["hello"] }], file: "output.txt" }]"#,
        output
    );
}

#[test]
fn output_redirect_with_arguments_ast() {
    let input = String::from("echo hello world > output.txt");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: RedirectOutput, cmds: [Command { cmd: "echo", args: ["hello", "world"] }], file: "output.txt" }]"#,
        output
    );
}

#[test]
fn output_redirect_between_commands_ast() {
    let input = String::from("pwd; echo hello > output.txt; ls");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Command { cmd: "pwd", args: [] }, Redirect { op: RedirectOutput, cmds: [Command { cmd: "echo", args: ["hello"] }], file: "output.txt" }, Command { cmd: "ls", args: [] }]"#,
        output
    );
}

#[test]
fn env_var_inside_output_redirect_ast() {
    unsafe {
        std::env::set_var("TEST_VAR", "hello");
    }

    let input = String::from("echo $TEST_VAR > output.txt");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: RedirectOutput, cmds: [Command { cmd: "echo", args: ["hello"] }], file: "output.txt" }]"#,
        output
    );
}

#[test]
fn subshell_output_redirect_ast() {
    let input = String::from("(echo hello) > output.txt");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: RedirectOutput, cmds: [Subshell { cmd: "echo hello " }], file: "output.txt" }]"#,
        output
    );
}

#[test]
fn inline_group_output_redirect_ast() {
    let input = String::from("{echo hello} > output.txt");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: RedirectOutput, cmds: [InlineGroup { cmds: [Command { cmd: "echo", args: ["hello"] }] }], file: "output.txt" }]"#,
        output
    );
}

#[test]
fn pipe_followed_by_output_redirect_ast() {
    let input = String::from("echo hello | wc > output.txt");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: RedirectOutput, cmds: [Redirect { op: Pipe, cmds: [Command { cmd: "echo", args: ["hello"] }, Command { cmd: "wc", args: [] }], file: "" }], file: "output.txt" }]"#,
        output
    );
}

#[test]
fn output_redirect_on_left_side_of_pipe_ast() {
    let input = String::from("echo hello > output.txt | wc");

    let tokens = lex(&input).unwrap();
    let ast = build_ast(&tokens).unwrap();

    let output = format!("{:?}", ast);

    assert_eq!(
        r#"[Redirect { op: Pipe, cmds: [Redirect { op: RedirectOutput, cmds: [Command { cmd: "echo", args: ["hello"] }], file: "output.txt" }, Command { cmd: "wc", args: [] }], file: "" }]"#,
        output
    );
}

#[test]
fn leading_output_redirect_error() {
    let input = String::from("> output.txt");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn invalid_output_redirect_target_error() {
    let input = String::from("echo hello > |");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}

#[test]
fn output_redirect_inside_inline_group_error() {
    let input = String::from("{echo hello > output.txt}");

    let tokens = lex(&input).unwrap();

    assert!(build_ast(&tokens).is_err());
}
