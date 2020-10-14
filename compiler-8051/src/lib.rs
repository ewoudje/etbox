use crate::graph::Graph;
use std::os::raw::c_char;
use std::rc::Rc;
use std::ops::Range;

mod graph;
mod instr;
mod graph_builder;

type Instruction = (instr::Instruction, usize);

#[derive(Debug)]
pub struct Compiler {
    graphs: Vec<(Vec<usize>, Rc<graph::Graph<Instruction>>)>,
    current_graph: Option<graph_builder::GraphBuilder<Instruction, usize>>
}

mod alloc {
    use std::mem::MaybeUninit;
    use super::Compiler;

    pub static mut COMPILER: MaybeUninit<Compiler> = MaybeUninit::uninit();
}


#[no_mangle]
pub unsafe fn create_compiler() -> &'static mut Compiler {
    use std::intrinsics::transmute;

    *alloc::COMPILER.as_mut_ptr() = Compiler {
        graphs: Vec::new(),
        current_graph: None
    };

    transmute(alloc::COMPILER.as_mut_ptr())
}

#[no_mangle]
pub fn new_graph(compiler: &mut Compiler, pos: i32) {
    if let Some(graph) = compiler.current_graph.replace(graph_builder::GraphBuilder::new()) {
        compiler.graphs.push(graph.build());
    }
}

#[no_mangle]
pub fn instruction(compiler: &mut Compiler, 
                                   instr: *const c_char,
                                   type1: i32, value1: i32,
                                   type2: i32, value2: i32,
                                   type3: i32, value3: i32,
                                   id: i32) {

    let instr: String = to_string(instr);

    let opcode = instr::Opcode::from_str(instr.as_str()).unwrap();

    let ins = opcode.make_instruction((type1, value1), (type2, value2), (type3, value3)).unwrap();

    compiler.current_graph.unwrap().add((ins, id as usize), compiler);
}

fn to_string(pointer: *const c_char) -> String {
    use std::ffi::{CStr,CString};
    let slice = unsafe { CStr::from_ptr(pointer).to_bytes() };
    String::from_utf8_lossy(slice).to_string()
}

impl graph_builder::Graphifier<Instruction, usize> for Compiler {
    fn refered(&self, data: Instruction) -> Option<usize> {
        Some(data.1)
    }

    fn branch(&self, data: Instruction) -> Option<usize> {
        [data.0.par1, data.0.par2, data.0.par3].iter().find(|p| match p {
            instr::InstructionParameter::Instr(_, instr::ReferenceType::Branch) => true,
            _ => false
        }).map(|p| { if let instr::InstructionParameter::Instr(r, _) = p { Some(*r) } else { None }})
        .flatten()
    }

    fn goto(&self, data: Instruction) -> Option<usize> {
        [data.0.par1, data.0.par2, data.0.par3].iter().find(|p| match p {
            instr::InstructionParameter::Instr(_, instr::ReferenceType::Goto) => true,
            _ => false
        }).map(|p| { if let instr::InstructionParameter::Instr(r, _) = p { Some(*r) } else { None }})
        .flatten()
    }

    fn graph_reference(&self, data: Instruction) -> Option<Rc<Graph<Instruction>>> {
        [data.0.par1, data.0.par2, data.0.par3].iter().find(|p| match p {
            instr::InstructionParameter::Instr(_, instr::ReferenceType::Call) => true,
            _ => false
        }).map(|p| { if let instr::InstructionParameter::Instr(r, _) = p { Some(*r) } else { None }})
        .flatten().map(|id| {
            self.graphs.iter().find(|g| g.0.contains(&id)).map(|r| r.1.clone())
        }).flatten()
    }
}