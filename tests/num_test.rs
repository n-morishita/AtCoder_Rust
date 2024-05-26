// use cli_test_dir::*;

// const BIN: &'static str = "./main";

// #[test]
// fn case1() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(r#"
//         3 5
// 5 1 8 9 7
// "#)
//         .tee_output()
//         .expect_success();
//     let expected_number = 4;
//     assert_eq!(output.stdout_str().trim(), expected_number.to_string());
//     assert!(output.stderr_str().is_empty());
// }

// #[test]
// fn case2() {
//   let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(r#"
//         3 5
// 4 2 9 7 5
// "#)
//         .tee_output()
//         .expect_success();
//     let expected_number = -1;
//     assert_eq!(output.stdout_str().trim(), expected_number.to_string());
//     assert!(output.stderr_str().is_empty());
// }

// #[test]
// fn case3() {
//   let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(r#"
//         4 12
// 13 9 6 5 2 7 16 14 8 3 10 11
// "#)
//         .tee_output()
//         .expect_success();
//     let expected_number = 9;
//     assert_eq!(output.stdout_str().trim(), expected_number.to_string());
//     assert!(output.stderr_str().is_empty());
// }