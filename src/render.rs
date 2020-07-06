use crate::{
    context::Context,
    template::{Instruction, SpanIndex, Template},
};

pub type InstructionPointer = usize;

#[derive(Debug)]
pub struct RenderState {
    pub register: String,
}

impl RenderState {
    fn new() -> Self {
        RenderState {
            register: "".to_string(),
        }
    }
}

pub enum RenderError {
    InvalidSpanIndex(InstructionPointer, SpanIndex),
}

pub fn render_template(template: &Template, context: &Context) -> Result<String, RenderError> {
    let mut state = RenderState::new();

    let mut result = String::new();

    for (ip, instruction) in template.instructions.iter().enumerate() {
        // println!("Instruction {}: {:?}", ip, instruction);

        match instruction {
            Instruction::LoadSpan(index) => match template.spans.get(*index) {
                Some(span) => state.register = span.clone(),
                None => return Err(RenderError::InvalidSpanIndex(ip, *index)),
            },

            Instruction::LoadKeypath(keypath) => match context.keypath(keypath) {
                Some(value) => state.register = value,
                None => state.register = "".to_string(),
            },

            Instruction::Emit => {
                result.push_str(&state.register);
                state.register.clear();
            }
        }
    }

    Ok(result)
}
