use snake::runner;

macro_rules! mk_test {
    ($test_name:ident, $file_name:expr, $expected_output:expr) => {
        #[test]
        fn $test_name() -> std::io::Result<()> {
            test_example_file($file_name, $expected_output)
        }
    };
}
macro_rules! mk_any_test {
    ($test_name:ident, $file_name:expr) => {
        #[test]
        fn $test_name() -> std::io::Result<()> {
            test_example_any($file_name)
        }
    };
}

macro_rules! mk_fail_test {
    ($test_name:ident, $file_name:expr, $expected_output:expr) => {
        #[test]
        fn $test_name() -> std::io::Result<()> {
            test_example_fail($file_name, $expected_output)
        }
    };
}

/*
 * YOUR TESTS GO HERE
 */

// IMPLEMENTATION
fn test_example_file(f: &str, expected_str: &str) -> std::io::Result<()> {
    use std::path::Path;
    let p_name = format!("examples/{}", f);
    let path = Path::new(&p_name);

    let tmp_dir = tempfile::TempDir::new()?;
    let mut w = Vec::new();
    match runner::compile_and_run_file(path, tmp_dir.path(), &mut w) {
        Ok(()) => {
            let stdout = std::str::from_utf8(&w).unwrap();
            assert_eq!(stdout.trim(), expected_str)
        }
        Err(e) => {
            panic!("Expected {}, got an error: {}", expected_str, e)
        }
    }
    Ok(())
}

fn test_example_any(f: &str) -> std::io::Result<()> {
    use std::path::Path;
    let p_name = format!("examples/{}", f);
    let path = Path::new(&p_name);

    let tmp_dir = tempfile::TempDir::new()?;
    let mut w = Vec::new();
    match runner::compile_and_run_file(path, tmp_dir.path(), &mut w) {
        Ok(()) => {}
        Err(e) => {
            panic!("Got an error: {}", e)
        }
    }
    Ok(())
}

fn test_example_fail(f: &str, includes: &str) -> std::io::Result<()> {
    use std::path::Path;

    let tmp_dir = tempfile::TempDir::new()?;
    let mut w_run = Vec::new();
    match runner::compile_and_run_file(
        Path::new(&format!("examples/{}", f)),
        tmp_dir.path(),
        &mut w_run,
    ) {
        Ok(()) => {
            let stdout = std::str::from_utf8(&w_run).unwrap();
            panic!("Expected a failure but got: {}", stdout.trim())
        }
        Err(e) => {
            let msg = format!("{}", e);
            assert!(
                msg.contains(includes),
                "Expected error message to include the string \"{}\" but got the error: {}",
                includes,
                msg
            )
        }
    }
    Ok(())
}
/* flaot tests */ 
mod float_tests{
    use super::*;
    mk_test!(test0, "test0.float", "5.5");
    //mk_test!(test1, "test1.float", "10.0");
    //mk_test!(test2, "test2.float", "true");
    //mk_test!(test3, "test3.float", "true");
    //mk_test!(test4, "test4.float", "true");
    //mk_test!(test5, "test5.float", "true");
    //mk_test!(test6, "test6.float", "true");
    //mk_test!(test7, "test7.float", "114");
    //mk_test!(test8, "test8.float", "true");
    //mk_test!(test9, "test9.float", "true");
}
mod old_tests {
    use super::*;
    mk_test!(test1, "old/test1.adder", "1");
    mk_test!(test2, "old/test2.adder", "2");
    mk_test!(test3, "old/test3.adder", "10");
    mk_test!(test4, "old/test4.adder", "11");
    mk_test!(test5, "old/test5.adder", "21");
    mk_test!(test6, "old/test6.adder", "21");
    mk_any_test!(test7, "old/test7.adder");
    mk_any_test!(test15, "old/test1.adder");
    mk_fail_test!(test14, "old/parse_error.adder", "");
    mk_test!(test8, "old/test8.adder", "3");
    mk_test!(test9, "old/test9.boa", "35");
    mk_test!(test10, "old/test10.boa", "10");
    mk_test!(test11, "old/test11.boa", "10");
    mk_test!(test12, "old/test12.boa", "13");
    mk_fail_test!(test13, "old/test13.boa", "overflow");
    mk_test!(test16, "old/test16.cobra", "true");
    mk_test!(test17, "old/test17.cobra", "false");
    mk_test!(test18, "old/test18.cobra", "true");
    mk_test!(test19, "old/test19.cobra", "true");
    mk_test!(test20, "old/test20.cobra", "true");
    mk_fail_test!(test21, "old/test21.cobra", "overflow");
    mk_fail_test!(test22, "old/test22.cobra", "arithmetic expected a number");
    mk_test!(test23, "old/test23.cobra", "-4611686018427387904");
    mk_fail_test!(test24, "old/test24.cobra", "overflow");
    mk_fail_test!(test25, "old/test25.cobra", "overflow");
    mk_test!(test26, "old/test26.cobra", "5\n5");
    mk_test!(test27, "old/test27.cobra", "5\n7\n12");
    mk_test!(test28, "old/test28.diamondback", "25");
    mk_test!(test29, "old/test29.diamondback", "5");
    mk_test!(test30, "old/test30.diamondback", "14\n35");
    mk_test!(test31, "old/test31.diamondback", "80");
    mk_test!(test32, "old/test32.diamondback", "80");
    mk_test!(test33, "old/test33.diamondback", "15");
    mk_test!(test34, "old/test34.diamondback", "true");
    mk_test!(test35, "old/test35.diamondback", "1");
    mk_test!(test36, "old/test36.egg", "true");
    mk_test!(test37, "old/test37.egg", "[1, 2, 3, 4, 5]");
    mk_test!(test38, "old/test38.egg", "[1, 0, 0]");
    mk_test!(test39, "old/test39.egg", "[0, <loop>]");
    mk_test!(test40, "old/test40.egg", "12");
    mk_test!(test41, "old/test41.egg", "3");
    mk_test!(test42, "old/test42.diamondback", "false");
    mk_test!(test43, "old/test43.egg", "8");
    mk_test!(test44, "old/test44.egg", "36");
    mk_test!(test45, "old/test45.egg", "true");
}