use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn case1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"
        3 2 2
3 1 2 3 o
2 2 3 x
"#)
        .tee_output()
        .expect_success();
    let expected_number = 2;
    assert_eq!(output.stdout_str().trim(), expected_number.to_string());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn case2() {
  let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"
        4 5 3
        3 1 2 3 o
        3 2 3 4 o
        3 3 4 1 o
        3 4 1 2 o
        4 1 2 3 4 x        
"#)
        .tee_output()
        .expect_success();
    let expected_number = 0;
    assert_eq!(output.stdout_str().trim(), expected_number.to_string());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn case3() {
  let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"
        11 4 9
10 1 2 3 4 5 6 7 8 9 10 o
11 1 2 3 4 5 6 7 8 9 10 11 o
10 11 10 9 8 7 6 5 4 3 2 x
10 11 9 1 4 3 7 5 6 2 10 x
"#)
        .tee_output()
        .expect_success();
    let expected_number = 8;
    assert_eq!(output.stdout_str().trim(), expected_number.to_string());
    assert!(output.stderr_str().is_empty());
}