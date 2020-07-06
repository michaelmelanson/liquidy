use serde::{Serialize, Deserialize};

pub type Data = String;
pub type DataIndex = usize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Instruction {
  Emit(DataIndex)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct Template {
  pub instructions: Vec<Instruction>,
  pub data: Vec<Data>
}

impl Template {
  pub fn new() -> Self {
    Template {
      instructions: Vec::new(),
      data: Vec::new()
    }
  }

  pub fn emit(&mut self, str: String) {
    let data_index = self.data.len();
    self.data.push(str);
    self.instructions.push(Instruction::Emit(data_index));
  }
}
