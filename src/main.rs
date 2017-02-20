extern crate lalrpop_util; 

pub mod shrimp; 

fn assert_ok(str program) {
    assert!(shrimp::parse_Program("print " + program).is_ok());
}
    
fn assert_err(str program) {
    assert!(shrimp::parse_Program("print " + program).is_err());
}

fn assert_exp_ok(str exp) {
    assert_ok("print " + exp);
}

fn assert_stmt_ok(str stmt) {
    assert_ok(stmt + " print 1"); 
}

#[test]
fn test_smallest() {
    assert_exp_ok("1");
}

#[test]
fn test_identifier() {
    assert_exp_ok("x");
    assert_exp_ok("y");
    assert_exp_ok("xy123");
    assert_exp_ok("x_y_123");
    assert_exp_ok("12345");
    assert_exp_ok("12_34");
}

#[test]
fn test_this() {
    assert_exp_ok("this");
}

#[test]
fn test_not() {
    assert_exp_ok("!x");
    assert_exp_ok("!!!!!!x");
}

#[test]
fn test_parens() {
    assert_exp_ok("(1)");
    assert_exp_ok("(((((1)))))");
}

#[test]
fn test_mult() {
    assert_exp_ok("10*9");
    assert_exp_ok("10*9*8");
    assert_exp_ok("foo*length");
    assert_exp_ok("10*9*8*7*x*y*foo");
}

#[test]
fn test_add() {
    assert_exp_ok("10+9");
    assert_exp_ok("10-9");
    assert_exp_ok("10+9+8");
    assert_exp_ok("10-9-8");
    assert_exp_ok("length+length");
    assert_exp_ok("length-length");
    assert_exp_ok("foo+foo");
    assert_exp_ok("foo+(foo)");
    assert_exp_ok("10+9+x*length-foo+array");
    assert_exp_ok("(a-b)*(a+b)");
    assert_exp_ok("a-b-c*5+4*3");
}

#[test]
fn test_comp() {
    assert_exp_ok("10<9");
    assert_exp_ok("10+a*3<9-4+2");
    assert_exp_ok("length<1");
    assert_exp_ok("i<foo");
}

#[test]
fn test_conditional() {
    assert_exp_ok("10<9?x:y");
    assert_exp_ok("10+a*3<9-4+2 ? 3 + 4 : 5 * 7");
    assert_exp_ok("1 ? 2 ? 3 ? 4 : 5 : 6 : 7");
    assert_exp_ok("1 ? 2 ? 3 : 4 ? 5 : 6 : 7 ? 8 : 9");
}

#[test]
fn test_assign() {
    assert_stmt_ok("numbers = numbers + 1;");
    assert_stmt_ok("foo=foo+1;");
}

#[test]
fn test_bigger() {
    assert_stmt_ok("i0 = 0; i1 = 1; total = i0 + i1;");
    assert_ok("this = 1 < 2 ? 3 < 4 ? 5 : 6 < 7 ? 8 : 9 : 10 < 11 ? 12 : 13; print this");
    assert_ok("zero = 0; one = 1; two = one + zero; three = two + one; four = three + two; print four");
    assert_ok("l = 5; r = 8; less = l < r; print less");
    assert_ok("less = 1 < 2; more = !less; print more");
    assert_ok("y = 2017; print y + 1")
}

fn main() {
    println!("Hello, world!");
}
