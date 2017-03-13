use objects::object::{Object, ObjectType};
use std::fmt::{Debug, Formatter, Error, Display};
use objects::single_character::SingleCharacter;


pub struct Symbol {
   pub sym_val: String,
}

impl Symbol{
   pub fn new(inp: String) -> Symbol {
      Symbol{sym_val: inp}
   }
}

impl Object for Symbol{
   fn obj_type(&self) -> ObjectType { ObjectType::Symbol }
   fn source(&self) -> Vec<SingleCharacter> {
      let mut ret = vec![];
      for chr in self.sym_val.to_string().chars(){
         ret.push(SingleCharacter::new(chr));
      }
      ret
   }

}


impl Display for Symbol{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.sym_val)
   }
}
impl Debug for Symbol{
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "S({})", self)
   }
}