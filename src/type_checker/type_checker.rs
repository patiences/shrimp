use ast::*;

pub trait TypeCheckerImplementation {
  fn type_check(&self);
}

impl TypeCheckerImplementation for Program {
  fn type_check(&self) {
    // 1st pass: build symbol table TODO
    // 2nd pass: typecheck TODO
  }

}
