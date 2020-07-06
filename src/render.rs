
use crate::template::{Instruction, Template, DataIndex};

pub type InstructionPointer = usize;

pub enum RenderError {
  InvalidDataIndex(InstructionPointer, DataIndex)
}

pub fn render_template(template: &Template) -> Result<String, RenderError> {
  let mut result = String::new();

  for (ip, instruction) in template.instructions.iter().enumerate() {
    match instruction {
      Instruction::Emit(index) => {
        match template.data.get(*index) {
          Some(data) => result.push_str(data),
          None => return Err(RenderError::InvalidDataIndex(ip, *index))
        }
      }
    }
  }

  Ok(result)
}
