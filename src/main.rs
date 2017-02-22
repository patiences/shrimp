extern crate lalrpop_util; 

pub mod shrimp; 
pub mod ast; 

// TODO: Revisit these string types...?
fn assert_ok(program : &str) {
    assert!(shrimp::parse_Program(&("print ".to_string() + program)).is_ok());
}

fn assert_err(program : &str) {
    assert!(shrimp::parse_Program(&("print ".to_string() + program)).is_err());
}

fn assert_exp_ok(exp : &str) {
    assert_ok(&("print ".to_string() + exp));
}

// Programs end with a print statement 
fn assert_exp_err(exp : &str) {
    assert_err(&("print ".to_string() + exp));
}

// Programs end with a print statement 
fn assert_stmt_ok(stmt : &str) {
    assert_ok(&(stmt.to_string() + " print 1")); 
}

fn assert_stmt_err(stmt : &str) {
    assert_err(&(stmt.to_string() + " print 1"));
}

// Many of these tests are invalid programs -- which is okay! 
// We only care about whether or not they pass the parsing phase. 

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
    assert_exp_ok("___");
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
    assert_exp_err("((1");
    assert_exp_err("1))");
}

#[test]
fn test_mult() {
    assert_exp_ok("10*9");
    assert_exp_ok("10*9*8");
    assert_exp_ok("foo*length");
    assert_exp_ok("10*9*8*7*x*y*foo");
    assert_exp_err("*");
    assert_exp_err("1*");
    assert_exp_err("*foo");
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
    assert_exp_err("+-");
    assert_exp_err("+1");
    assert_exp_err("123-");
}

#[test]
fn test_comp() {
    assert_exp_ok("10<9");
    assert_exp_ok("10+a*3<9-4+2");
    assert_exp_ok("length<1");
    assert_exp_ok("i<foo");
    assert_exp_err("<<<");
}

#[test]
fn test_conditional() {
    assert_exp_ok("10<9?x:y");
    assert_exp_ok("10+a*3<9-4+2 ? 3 + 4 : 5 * 7");
    assert_exp_ok("1 ? 2 ? 3 ? 4 : 5 : 6 : 7");
    assert_exp_ok("1 ? 2 ? 3 : 4 ? 5 : 6 : 7 ? 8 : 9");
    assert_exp_err("????");
    assert_exp_err("?");
    assert_exp_err("1 ? 2");
}

#[test]
fn test_assign() {
    assert_stmt_ok("numbers = numbers + 1;");
    assert_stmt_ok("foo=foo+1;");
    assert_stmt_err("foo=");
    assert_stmt_err("foo=foo");
}

#[test]
fn test_bigger() {
    assert_stmt_ok("i0 = 0; i1 = 1; total = i0 + i1;");
    assert_ok("this = 1 < 2 ? 3 < 4 ? 5 : 6 < 7 ? 8 : 9 : 10 < 11 ? 12 : 13; print this");
    assert_ok("zero = 0; one = 1; two = one + zero; three = two + one; four = three + two; print four");
    assert_ok("l = 5; r = 8; less = l < r; print less");
    assert_ok("less = 1 < 2; more = !less; print more");
    assert_ok("y = 2017; print y + 1");
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
