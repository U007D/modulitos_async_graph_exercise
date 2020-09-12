use structopt::StructOpt;

/// See `unit_tests` for sample usages and how to drive CLI argument parsing from tests.
#[cfg(test)]
mod unit_tests;

/// Args is a data structure representing the user's supplied command-line arguments supplied to the program.
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Args {}
