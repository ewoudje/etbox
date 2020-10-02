pub struct InstructionGraph {
    origin: u16,
}

impl InstructionGraph {
    pub fn new(start: u16) -> InstructionGraph {
        InstructionGraph {
            origin: start
        }
    }
}