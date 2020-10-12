use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InstructionParameters {
    A,
    Rn(u8),
    PRi(bool),
    Const(u16),
    Direct(u8),
    None
}

struct InstructionNotFound {}

macro_rules! instr {
    ($($instr:ident => ($(($p1:ident, $p2:ident, $p3:ident) $(=> $r:ident)?),+) $(=> $rt:ident)?),+) => {
        pub enum Instruction {
            $($instr(InstructionParameters, InstructionParameters, InstructionParameters)),+
        }

        impl Instruction {
            pub fn modifies(&self) -> InstructionParameters {
                instr!(match impl (self) modifies ($($instr ($(($p1, $p2, $p3) $(=> $r)?),+) $(=> $rt)?),+) | unimplemented!())
            }

            pub fn valid(&self) -> bool {
                instr!(match impl (self) valid ($($instr ($(($p1, $p2, $p3) $(=> $r)?),+) $(=> $rt)?),+) | false)
            }

            pub fn from_str(s: &str, p1: InstructionParameters, p2: InstructionParameters, p3: InstructionParameters) -> Result<Instruction, InstructionNotFound> {
                match s {
                    $(stringify!($instr) => Ok(Instruction::$instr(p1, p2, p3))),+,
                    _ => Err(InstructionNotFound {})
                }
            }

        }
    };

    (full impl ($self:expr) modifies $instr:ident ($(($p1:ident, $p2:ident, $p3:ident) => $r:ident),+) | $st:stmt) => {
        match $self {
            $(
                Instruction::$instr(InstructionParameters::$p1,InstructionParameters::$p2,InstructionParameters::$p3) => InstructionParameters::$r
            ),+,
            _ => { $st }
        }
    };

    (full impl ($self:expr) valid $instr:ident ($(($p1:ident, $p2:ident, $p3:ident) => $r:ident),+) | $st:stmt) => {
        match $self {
            $(
                Instruction::$instr(InstructionParameters::$p1,InstructionParameters::$p2,InstructionParameters::$p3) => true
            ),+,
            _ => { $st }
        }
    };

    (impl ($self:expr) $i:ident $instr:ident ($(($p1:ident, $p2:ident, $p3:ident)),+) => $r:ident $(| $st:stmt)?) => {
        instr!(r impl ($self) $i $instr ($(($p1, $p2, $p3) => $r),+) $(| $st)?)
    };


    (r impl ($self:expr) $i:ident $instr:ident ($(($p1:ident,$p2:ident,$p3:ident) => $r:ident),+) $(| $st:stmt)?) => {
        instr!(full impl ($self) $i $instr ($(($p1, $p2, $p3) => $r),+) $(| $st)?)
    };

    (r impl ($self:expr) $i:ident $instr:ident ($(($p1:ident,$p2:ident,$p3:ident)),+) $(| $st:stmt)?) => {
        instr!(full impl ($self) $i $instr ($(($p1, $p2, $p3) => None),+) $(| $st)?)
    };

    (match impl ($self:expr) $i:ident ($instr:ident ($(($($p1:ident $(,$p2:ident $(, $p3:ident)?)?)?) $(=> $r:ident)?),+) $(=> $rt:ident)? , $($instr2:ident ($(($($p12:ident $(,$p22:ident $(, $p32:ident)?)?)?)),+) $(=> $r2:ident)?),+) | $st:stmt) => {
        instr!(impl ($self) $i $instr ($(($($p1 $(,$p2 $(, $p3)?)?)?) $(=> $r)?),+) $(=> $rt)? | instr!(match impl modifies $(($instr2 ($(($($p12 $(,$p22 $(, $p32)?)?)?) $(=> $r2)?),+) $(=> $r2)?)),*) | $st)
    };

    (match impl ($self:expr) $i:ident ($instr:ident ($(($($p1:ident $(,$p2:ident $(, $p3:ident)?)?)?) $(=> $r:ident)?),+) $(=> $rt:ident)?) | $st:stmt) => {
        instr!(impl ($self) $i $instr ($(($($p1 $(,$p2 $(, $p3)?)?)?) $(=> $r)?),+) $(=> $rt)? | $st)
    };
}

macro_rules! some {
    ($p:ident) => ($p);
    () => (None);
}

instr!(
    ADD => (
       (A, Rn, None),
       (A, Direct, None),
       (A, PRi, None),
       (A, Const, None)
    ) => A
);

/*
pub fn reads(&self) -> (InstructionParameters, InstructionParameters, InstructionParameters) {
                (self.0, self.1, self.2)
            }
 */