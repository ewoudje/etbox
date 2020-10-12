use crate::graph::InstructionGraph;
use std::os::raw::c_char;
use std::rc::Rc;
use crate::instr::InstructionParameters;

mod graph;
mod instr;
mod graph_builder;

pub struct Compiler {
    graphs: Vec<Rc<graph::InstructionGraph<instr::Instruction>>>,
    current_graph: usize
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
        current_graph: 0
    };

    transmute(alloc::COMPILER.as_mut_ptr())
}

#[no_mangle]
pub fn new_graph(compiler: &mut Compiler,
                                  pos: i32) {
    compiler.graphs.push(InstructionGraph::new(pos as u16));
    compiler.current_graph = compiler.graphs.len() - 1;
}

#[no_mangle]
pub fn instruction(compiler: &mut Compiler,
                                   instr: *const c_char,
                                   type1: i32, value1: i32,
                                   type2: i32, value2: i32,
                                   type3: i32, value3: i32,
                                   id: i32) {

    let instr: String = to_string(instr);

    let p1 = (type1, value1).into();

    let ins = instr::Instruction::from_str(instr.as_str(), p1, p2, p3).unwrap();

    if ins.valid() {

    } else {
        unimplemented!()
    }
}

fn to_string(pointer: *const c_char) -> String {
    use std::ffi::{CStr,CString};
    let slice = unsafe { CStr::from_ptr(pointer).to_bytes() };
    String::from_utf8_lossy(slice).to_string()
}

impl From<(i32, i32)> for InstructionParameters {

    fn from(java: (i32, i32)) -> Self {
        let (ty, value) = java;
        match ty {
            0 => InstructionParameters::Const(value as u16),
            5 => InstructionParameters::Direct(value as u8),
            _ => unimplemented!()
        }
    }
}