pub enum Expression {
    IntegerLiteral(i32),
    IdentifierExp(str),  
    Plus(Box<Expression>, Box<Expression>),
    Minus(Box<Expression>, Box<Expression>),
    Times(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>), 
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>),
}

pub enum Statement {
    Assign(str, Box<Expression)
}

