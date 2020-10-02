use crate::graph::InstructionGraph;
use std::os::raw::c_char;

mod graph;

pub struct Compiler {
    graphs: Vec<graph::InstructionGraph>,
    current_graph: usize
}

mod alloc {
    use std::mem::MaybeUninit;
    use super::Compiler;

    pub static mut compiler: MaybeUninit<Compiler> = MaybeUninit::uninit();
}


#[no_mangle]
pub unsafe extern "system" fn create_compiler(_env: JNIEnv, _class: JClass) -> &'static mut Compiler {
    use std::intrinsics::transmute;

    *alloc::compiler.as_mut_ptr() = Compiler {
        graphs: Vec::new(),
        current_graph: 0
    };

    transmute(alloc::compiler.as_mut_ptr())
}

#[no_mangle]
pub extern "system" fn new_origin(compiler: &mut Compiler,
                                  pos: i32) {
    compiler.graphs.push(InstructionGraph::new(pos as u16));
    compiler.current_graph = compiler.graphs.len() - 1;
}

#[no_mangle]
pub extern "system" fn instruction(compiler: &mut Compiler,
                                   instr: *const c_char,
                                   type1: i32, value1: i32,
                                   type2: i32, value2: i32,
                                   type3: i32, value3: i32) {

    let instr: String = to_string(instr);

}

fn to_string(pointer: *const c_char) -> String {
    use std::ffi::{CStr,CString};
    let slice = unsafe { CStr::from_ptr(pointer).to_bytes() };
    str::from_utf8(slice).unwrap().to_string()
}