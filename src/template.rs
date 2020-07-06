use serde::{Deserialize, Serialize};

pub type Data = String;
pub type SpanIndex = usize;
pub type Keypath = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Instruction {
    LoadSpan(SpanIndex),
    LoadKeypath(Keypath),
    Emit,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct Template {
    pub instructions: Vec<Instruction>,
    pub spans: Vec<Data>,
}

impl Template {
    pub fn new() -> Self {
        Template {
            instructions: Vec::new(),
            spans: Vec::new(),
        }
    }

    pub fn add_span(&mut self, span: &String) -> SpanIndex {
        let span_index = self.spans.len();
        self.spans.push(span.clone());
        span_index
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}
