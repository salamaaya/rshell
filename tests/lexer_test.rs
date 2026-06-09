use rshell::lexer::lex;

#[test]
fn amount_of_tokens() {
    let input = String::from("echo this is a test \\n\\t >> somefile");
    let result = lex(&input);
    match result {
        Ok(r) => assert_eq!(9, r.len()),
        Err(_) => eprintln!("error getting the return value."),
    }
}

#[test]
fn token_types() {
    let input = String::from("echo this is a test \\n $PATH\\t >> somefile");
    let result = lex(&input);
    match result {
        Ok(r) => {
            let output = format!("{:?}", r);
            assert_eq!(
                r#"[Id("echo"), Id("this"), Id("is"), Id("a"), Id("test"), Escape("n"), Dollar, Id("PATH"), Escape("t"), RedirectOutputAppend, Id("somefile")]"#,
                output
            )
        }
        Err(_) => println!("Error getting the return value."),
    }
}
