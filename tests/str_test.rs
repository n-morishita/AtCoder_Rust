// use cli_test_dir::*;

// const BIN: &'static str = "./main";

// #[test]
// fn case1() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(r#"
//         2 3
// 10 20 30
// 20 0 10
// 0 100 100

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
//         2 4
// 10 20 30 40
// 20 0 10 30
// 0 100 100 0

// "#)
//         .tee_output()
//         .expect_success();
//     assert_eq!(output.stdout_str(), "No\n");
//     assert!(output.stderr_str().is_empty());
// }

// // #[test]
// // fn case3() {
// //     let testdir = TestDir::new(BIN, "");
// //     let output = testdir
// //         .cmd()
// //         .output_with_stdin(r#"
// //         10 1 10
// // "#)
// //         .tee_output()
// //         .expect_success();
// //     assert_eq!(output.stdout_str(), "10 9 8 7 6 5 4 3 2 1\n");
// //     assert!(output.stderr_str().is_empty());
// // }

