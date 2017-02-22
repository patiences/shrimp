pub enum Expression {
    IntegerLiteral(i32),
    IdentifierExp(String),  
    Plus(Box<Expression>, Box<Expression>),
    Minus(Box<Expression>, Box<Expression>),
    Times(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>), 
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>),
}

pub enum Statement {
    Assign(String, Box<Expression>),
    Print(Box<Expression>),
}

pub struct Program { 
    statements : Vec<Statement>, 
    print : Expression,

}
