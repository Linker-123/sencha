use std::fmt;

pub trait Assemble {
    fn to_asm(&self) -> String;
}

pub struct Context {
    pub external: Vec<String>,
    pub sections: Vec<Section>,
    pub constants: Vec<Ins>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            external: Default::default(),
            sections: Default::default(),
            constants: Default::default(),
        }
    }

    pub fn add_external_sym(&mut self, symbol: String) {
        self.external.push(symbol);
    }

    pub fn add_section(&mut self, sec: Section) {
        self.sections.push(sec);
    }

    pub fn add_constant(&mut self, constant: Ins) {
        self.constants.push(constant);
    }
}

impl Assemble for Context {
    fn to_asm(&self) -> String {
        let mut asm = String::from("global main\nextern ");
        asm.push_str(&self.external.join(","));

        for sec in &self.sections {
            asm.push('\n');
            asm.push_str(&sec.to_asm());
        }

        asm
    }
}

pub struct Section {
    pub kind: SectionKind,
    pub instructions: Vec<Ins>,
}

impl Section {
    pub fn new(kind: SectionKind) -> Section {
        Section {
            kind,
            instructions: Default::default(),
        }
    }

    pub fn add_instruction(&mut self, ins: Ins) {
        self.instructions.push(ins);
    }
}

impl Assemble for Section {
    fn to_asm(&self) -> String {
        let mut sec = match self.kind {
            SectionKind::Bss => String::from("section .bss\n"),
            SectionKind::Data => String::from("section .data\n"),
            SectionKind::Text => String::from("section .text\n"),
        };

        for ins in &self.instructions {
            sec.push_str(&ins.to_asm());
            sec.push('\n');
        }

        sec
    }
}

pub enum SectionKind {
    Data,
    Bss,
    Text,
}

#[allow(dead_code)]
pub enum Ins {
    /// Define a constant. Args: name, value
    Constant(String, String),
    /// Define a constant using %assign. Args: name value
    Assign(String, String),
    /// Define a constant using %define. Args: name, value
    Define(String, String),
    /// Macro instruction. Args: name, args count, body
    Macro(String, usize, Vec<Ins>),
    /// Initialized data. Args: name, data kind, value
    IData(String, DataKind, String),
    /// Uninitialized data. Args: name, data kind
    UData(String, DataKind),
    /// Inline data. Args: name, instruction arguments
    Inline(Op, Vec<Tgt>),
    /// Label instruction. Args: name, body
    Label(String, Vec<Ins>),
}

impl Assemble for Ins {
    fn to_asm(&self) -> String {
        match self {
            Self::Constant(name, value) => format!("{} equ {}", name, value),
            Self::Assign(name, value) => format!("%assign {} {}", name, value),
            Self::Define(name, value) => format!("%define {} {}", name, value),
            Self::Macro(name, args, body) => {
                let mut code = format!("%macro {} {}\n", name, args);
                for ins in body {
                    code.push_str(&ins.to_asm());
                    code.push('\n');
                }

                code.pop();
                code.push_str("%endmacro");
                code
            }
            Self::IData(name, kind, value) => format!("{} {} {}", name, &kind.to_asm(), value),
            Self::UData(name, kind) => format!("{} {} ?", name, &kind.to_asm()),
            Self::Inline(name, args) => {
                format!(
                    "{} {}",
                    name.to_asm(),
                    args.into_iter()
                        .map(|x| x.to_asm())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            Self::Label(name, body) => {
                let mut code = format!("{}:\n", name);
                for ins in body {
                    code.push_str(&ins.to_asm());
                    code.push('\n');
                }
                code.pop();
                code
            }
        }
    }
}

#[allow(dead_code)]
pub enum DataKind {
    /// 8 bits
    Byte,
    /// 16 bits
    Word,
    /// 32 bits
    Dword,
    /// 64 bits
    Qword,
    /// 80 bits
    Tword,
    /// 128 bits
    Oword,
    /// 256 bits
    Yword,
    /// 512 bits
    Zword,
}

impl Assemble for DataKind {
    fn to_asm(&self) -> String {
        match self {
            Self::Byte => "db",
            Self::Word => "dw",
            Self::Dword => "dd",
            Self::Qword => "dq",
            Self::Tword => "dt",
            Self::Oword => "do",
            Self::Yword => "dy",
            Self::Zword => "dz",
        }
        .to_string()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Tgt {
    // full 64-bit registers
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    // lower 32 bit registers
    Eax,
    Ebx,
    Ecx,
    Edx,
    Esi,
    Edi,
    Ebp,
    Esp,
    R8d,
    R9d,
    R10d,
    R11d,
    R12d,
    R13d,
    R14d,
    R15d,
    // lower 16-bit registers
    Ax,
    Bx,
    Cx,
    Dx,
    Si,
    Di,
    Bp,
    Sp,
    R8w,
    R9w,
    R10w,
    R11w,
    R13w,
    R14w,
    R15w,
    // Lower 8-bit registers
    Al,
    Bl,
    Cl,
    Dl,
    Sil,
    Dil,
    Bpl,
    Spl,
    R8b,
    R9b,
    R10b,
    R11b,
    R12b,
    R13b,
    R14b,
    // Variables
    Var(String),
    Vars(&'static str),
    // Raw value
    Val(String),
    Value(&'static str),
}

impl fmt::Display for Tgt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Assemble for Tgt {
    fn to_asm(&self) -> String {
        match self {
            Tgt::Var(name) => format!("[{}]", name),
            Tgt::Vars(var) => format!("[{}]", var),
            Tgt::Value(val) => val.to_string(),
            Tgt::Val(val) => val.to_owned(),
            _ => self.to_string().to_lowercase(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Op {
    Mov,
    Call,
    Ret,
    Add,
    Mul,
    Imul,
    Idiv,
    Jmp,
    Cmp,
    Ji,
    Push,
    Pop,
    Xor,
    Lea,
    Int,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Assemble for Op {
    fn to_asm(&self) -> String {
        self.to_string().to_lowercase()
    }
}
