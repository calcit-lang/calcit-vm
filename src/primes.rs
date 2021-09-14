/*!
 * Calx is a simplied VM from Calcit, but not lower level enough.
 * Data in Calx is mutable, and has basic types and structures, such as Lists.
 * Calx uses a mixed form of prefix and postix instructions.
 *
 * (I'm not equiped enough for building a bytecode VM yet...)
 */

use std::fmt;

use cirru_parser::Cirru;

/// Simplied from Calcit, but trying to be basic and mutable
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Calx {
  Nil,
  Bool(bool),
  I64(i64),
  F64(f64),
  Str(String),
  List(Vec<Calx>),
  // to simultate linked structures
  Link(Box<Calx>, Box<Calx>, Box<Calx>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CalxType {
  Nil,
  Bool,
  I64,
  F64,
  List,
  Link,
}

impl fmt::Display for Calx {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Calx::Nil => f.write_str("nil"),
      Calx::Bool(b) => f.write_str(&b.to_string()),
      Calx::I64(n) => f.write_str(&n.to_string()),
      Calx::F64(n) => f.write_str(&n.to_string()),
      Calx::Str(s) => f.write_str(&s),
      Calx::List(xs) => {
        f.write_str("(")?;
        let mut at_head = true;
        for x in xs {
          if at_head {
            at_head = false
          } else {
            f.write_str(" ")?;
          }
          x.fmt(f)?;
        }
        f.write_str(")")?;
        Ok(())
      }
      Calx::Link(..) => f.write_str("TODO LINK"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CalxFunc {
  pub name: String,
  pub params_type: Vec<CalxType>,
  pub instrs: Vec<CalxInstr>,
}

/// learning from WASM but for dynamic data
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CalxInstr {
  Param, // load variable from parameter
  Local, // new local variable
  LocalSet(usize),
  LocalGet(usize),
  GlobalSet(usize),
  GlobalGet(usize),
  Load(Calx),
  Dup,
  Drop,
  // number operations
  IntAdd,
  IntMul,
  IntRem,
  IntNeg,
  IntShr,
  IntShl,
  Add,
  Mul,
  Div,
  Neg,
  // string operations
  // list operations
  NewList,
  ListGet(usize),
  ListSet(usize),
  // Link
  NewLink,
  // bool operations
  And,
  Or,
  // control stuctures
  Br(usize),
  BrIf(usize),
  Block {
    // bool oo to indicate loop
    looped: bool,
    from: usize,
    range: usize,
  },
  /// TODO use function name at first
  Echo, // pop and println current value
  Call(usize, String), // during running, only use index,
  Unreachable,
  Nop,
  Quit, // quit and return value
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CalxError {
  pub message: String,
  pub instrs: Vec<CalxInstr>,
  pub recent_stack: Vec<CalxInstr>, // maybe partial of stack
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CalxFrame {
  pub locals: Vec<Calx>, // params + added locals
  pub instrs: Vec<CalxInstr>,
}
