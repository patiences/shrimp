extern crate lalrpop_util;
extern crate shrimp;

use shrimp::parser::shrimp::parse_Program;
use shrimp::type_checker::type_checker::TypeCheckerImplementation;

#[test]
fn test_smallest() {
    // FIXME parse_Program returns a Result<Box<Program> ...> :-(
    assert!(parse_Program("1").type_check()).is_ok();
}
