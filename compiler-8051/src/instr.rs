#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub par1: InstructionParameter,
    pub par2: InstructionParameter,
    pub par3: InstructionParameter
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ReferenceType {
    Branch,
    Goto,
    Call
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum InstructionParameter {
    A,
    Rn(u8),
    PRi(bool),
    Const(u16),
    Direct(u8),
    Instr(usize, ReferenceType),
    None
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct InstructionNotFound {}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct OpcodeNotFound {}

macro_rules! opcode {
    ($($op:ident => ($(($p1:ident $(( $(| $p2:ident $(( $(| $p3:ident $(=> $r3:ident)?)+))? $(=> $r2:ident)?)+))? $(=> $r1:ident)?)),+)),+) => {
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        pub enum Opcode {
            $($op),+
        }

        impl Opcode {
            pub fn from_str(string: &str) -> Result<Opcode, OpcodeNotFound> {
                match string {
                    $(stringify!($op) => Ok(Opcode::$op)),+,
                    _ => Err(OpcodeNotFound {})
                }
            }

            pub fn make_instruction(self, par1: (i32, i32), par2: (i32, i32), par3: (i32, i32)) -> Result<Instruction, InstructionNotFound> {
                let v = 0; //I have to make the compiler happy by adding a dummy variable
                let (par1, (par2, par3)) = match self {
                    $(Opcode::$op => match par1 {
                        $((oppar!($p1)) => (oppar!(into $p1 v), some2!($(match par2 {
                           $((oppar!($p2)) => (oppar!(into $p2 v), some!($(match par3 {
                                $((oppar!($p3)) => oppar!(into $p3 v)),+,
                                _ => return Err(InstructionNotFound {})
                            })?))),+,
                            _ => return Err(InstructionNotFound {})
                        })?))),+,
                        _ => return Err(InstructionNotFound {})
                    }),+
                };

                Ok(Instruction {
                    opcode: self,
                    par1,
                    par2,
                    par3
                })
            }
        }
    }
}

macro_rules! some {
    ($p:expr) => ($p);
    () => (InstructionParameter::None);
}

macro_rules! some2 {
    ($p:expr) => ($p);
    () => ((InstructionParameter::None, InstructionParameter::None));
}

macro_rules! oppar {
    (A) => {
        (2, 1)
    };
    (Rn) => {
        (2, v)
    };
    (Direct) => {
        (1, v)
    };
    (PRi) => {
        (3, v)
    };
    (Const) => {
        (5, v)
    };
    (RelB) => {
        (0, v)
    };

    (into A $v:ident) => {
        InstructionParameter::A
    };
    (into Direct $v:ident) => {
        InstructionParameter::Direct($v as u8)
    };
     (into Rn $v:ident) => {
        InstructionParameter::Rn(($v - 3) as u8)
    };
    (into PRi $v:ident) => {
        InstructionParameter::PRi($v == 1)
    };
    (into Const $v:ident) => {
        InstructionParameter::Const($v as u16)
    };
    (into RelB $v:ident) => {
        InstructionParameter::Instr($v as usize, ReferenceType::Branch)
    };
}

opcode!(
    ADD => (
       (A   (
            | Rn
            | Direct
            | PRi
            | Const
            ) => A
       )
    ),
    DJNZ => (
        (A  (
            | RelB
            ) => A
        )
    )
);