use crate::{
    parser::{Intermediate, Node},
    template::{Instruction, Template},
};

#[derive(Debug)]
pub enum CompileError {}

pub fn compile_intermediate(intermediate: &Intermediate) -> Result<Template, CompileError> {
    let mut template = Template::new();

    for node in intermediate.nodes.iter() {
        compile_node(&mut template, node)?;
    }

    Ok(template)
}

pub fn compile_node(template: &mut Template, node: &Node) -> Result<(), CompileError> {
    match node {
        Node::Span(span) => {
            let span_index = template.add_span(span);
            template.add_instruction(Instruction::LoadSpan(span_index));
            template.add_instruction(Instruction::Emit);
        }

        Node::Expression { keypath } => {
            template.add_instruction(Instruction::LoadKeypath(keypath.clone()));
            template.add_instruction(Instruction::Emit);
        }
    }
    Ok(())
}
