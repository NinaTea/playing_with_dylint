// This feature must be added so we can use compiler APIs.
#![feature(rustc_private)]

// We need to import them like this otherwise it doesn't work.
extern crate rustc_lint;

use rustc_lint::LintStore;
use rustc_tools::with_lints;

fn main() {
    // We skip the first argument because it's the current binary.
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    with_lints(args, vec![], |store: &mut LintStore| {
        // This is where we will register the lints.
    }).expect("with_lints failed");
}