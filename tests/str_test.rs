// use cli_test_dir::*;

// const BIN: &'static str = "./main";

// #[test]
// fn case1() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(r#"
//         3 2
//         3 2 5
//         4 1
// "#)
//         .tee_output()
//         .expect_success();
//     assert_eq!(output.stdout_str(), "Yes\n");
//     assert!(output.stderr_str().is_empty());
// }

// #[test]
// fn case2() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(r#"
// 3 2
// 3 1 5
// 4 2
// "#)
//         .tee_output()
//         .expect_success();
//     assert_eq!(output.stdout_str(), "No\n");
//     assert!(output.stderr_str().is_empty());
// }

// #[test]
// fn case3() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(r#"
//         1 1
//         1
//         2
// "#)
//         .tee_output()
//         .expect_success();
//     assert_eq!(output.stdout_str(), "No\n");
//     assert!(output.stderr_str().is_empty());
// }

