use crate::graph::Graph;
use std::os::raw::c_char;
use std::rc::Rc;
use crate::instr::Instruction;

mod graph;
mod instr;
mod graph_builder;

#[derive(Debug)]
pub struct Compiler {
    graphs: Vec<Rc<graph::Graph<Instruction>>>
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
        graphs: Vec::new()
    };

    transmute(alloc::COMPILER.as_mut_ptr())
}

#[no_mangle]
pub fn new_graph(pos: i32) -> usize {
    let result = Box::new(graph_builder::GraphBuilder::<Instruction, usize>::new());
    Box::into_raw(result) as usize
}

#[no_mangle]
pub fn instruction(compiler: &mut Compiler, graph_builder: &mut graph_builder::GraphBuilder<Instruction, usize>, 
                                   instr: *const c_char,
                                   type1: i32, value1: i32,
                                   type2: i32, value2: i32,
                                   type3: i32, value3: i32,
                                   id: i32) {

    let instr: String = to_string(instr);

    let opcode = instr::Opcode::from_str(instr.as_str()).unwrap();

    let ins = opcode.make_instruction((type1, value1), (type2, value2), (type3, value3)).unwrap();

    graph_builder.add(ins, compiler);
}

fn to_string(pointer: *const c_char) -> String {
    use std::ffi::{CStr,CString};
    let slice = unsafe { CStr::from_ptr(pointer).to_bytes() };
    String::from_utf8_lossy(slice).to_string()
}

impl graph_builder::Graphifier<Instruction, usize> for Compiler {
    //TODO
}