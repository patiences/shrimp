This is an attempt at modelling the front-end-est part of a compiler for a tiny language with Java-like syntax, which I am calling Shrimp. Based on a [course project](http://www.ugrad.cs.ubc.ca/~cs411/2016w2/project1.html), but using Rust! How exciting. Grammar below:

Expressions
-----------
Program ::= Statement* PRINT Expression

Statement ::= Assign 

Assign ::= Identifier "=" Expression ";" 

Expression ::= CompExpression ( "?" Expression ":" Expression ) ?

CompExpression ::= AddExpression ( "<" AddExpression ) ?

AddExpression ::= MultExpression ( ("+"|"-") MultExpression ) ? 

MultExpression ::= NotExpression ( * NotExpression ) ?

NotExpression ::= "!" NotExpression | PrimaryExpression 

PrimaryExpression ::= INTEGER_LITERAL | Identifier | "(" Expression ")"

Identifier ::= IDENTIFIER 

Functions 
---------
Statement ::= .... FunctionDeclaration 

FunctionDeclaration ::= Type Identifier "(" FormalList ")" "{"
                            Assign * 
                            "return" Expression ";" 
                        "}"

FormalList ::= ( Type Identifier ( "," Type Identifier ) * ) ? 

Type ::= "int" | "boolean" 

PrimaryExpression ::= .... Identifier "(" ExpressionList ")" 

ExpressionList ::= ( Expression ( "," Expression ) * ) ? 

---------------------------------------------------

3/12/17: Removed "zero or more (*)" matches on Add/MultExpressions to avoid 
dealing with matching on vectors in .lalrpop. 
Let's just require parens around addition/multiplication expressions like a + (b + c).    
