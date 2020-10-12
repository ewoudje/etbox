#[derive(Clone, Eq, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
    par1: InstructionParameters,
    par2: InstructionParameters,
    par3: InstructionParameters
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InstructionParameters {
    A,
    Rn(u8),
    PRi(bool),
    Const(u16),
    Direct(u8),
    Rel(i8),
    Instr(usize),
    None
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct InstructionNotFound {}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct OpcodeNotFound {}

macro_rules! opcode {
    ($($op:ident => ($(($p1:ident $(( $(| $p2:ident $(( $(| $p3:ident $(=> $r3:ident)?)+))? $(=> $r2:ident)?)+))? $(=> $r1:ident)?)),+)),+) => {
        #[derive(Copy, Clone, Eq, PartialEq)]
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
    () => (InstructionParameters::None);
}

macro_rules! some2 {
    ($p:expr) => ($p);
    () => ((InstructionParameters::None, InstructionParameters::None));
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
    (Rel) => {
        (0, v)
    };

    (into A $v:ident) => {
        InstructionParameters::A
    };
    (into Direct $v:ident) => {
        InstructionParameters::Direct($v as u8)
    };
     (into Rn $v:ident) => {
        InstructionParameters::Rn(($v - 3) as u8)
    };
    (into PRi $v:ident) => {
        InstructionParameters::PRi($v == 1)
    };
    (into Const $v:ident) => {
        InstructionParameters::Const($v as u16)
    };
    (into Rel $v:ident) => {
        InstructionParameters::Instr($v as usize)
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
            | Rel
            ) => A
        )
    )
);