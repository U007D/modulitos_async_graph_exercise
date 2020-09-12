use assert2::assert;
use structopt::StructOpt;

/// Args is a data structure representing the user's supplied command-line arguments supplied to the program.
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct TestArgs {
    /// Optional argument indicating absence or presence + amount of "some feature".
    pub some_arg: Option<usize>,
}

#[test]
fn args_given_a_valid_argument_succeeds() {
    // setup
    let app = "test";
    let arg = "3";
    let args = vec![app, arg];

    // given a `StructOpt::from_iter()` constructor
    let sut = TestArgs::from_iter_safe::<&Vec<&str>>;

    // when it is invoked
    let result = sut(&args);

    // then it should be OK
    assert!(result.is_ok() == true);
}

#[test]
fn args_given_a_valid_non_argument_succeeds() {
    // setup
    let app = "test";
    let args = vec![app];

    // given a `StructOpt::from_iter()` constructor
    let sut = TestArgs::from_iter_safe::<&Vec<&str>>;

    // when it is invoked
    let result = sut(&args);

    // then it should be OK
    assert!(result.is_ok() == true);
}

#[test]
fn args_given_an_invalid_argument_fails() {
    // setup
    let app = "test";
    let arg = "hello";
    let args = vec![app, arg];

    // given a `StructOpt::from_iter()` constructor
    let sut = TestArgs::from_iter_safe::<&Vec<&str>>;

    // when it is invoked
    let result = sut(&args);

    // then it should return an error
    assert!(result.is_err() == true);
}
