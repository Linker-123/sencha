use std::collections::HashMap;

pub struct Asm {
    labels: HashMap<String, AsmLabel>,
    #[allow(dead_code)]
    loc_counter: Vec<AsmLabel>,
}

impl Asm {
    pub fn new() -> Asm {
        Asm {
            labels: HashMap::new(),
            loc_counter: Vec::new(),
        }
    }

    pub fn add_label(&mut self, name: String, label: AsmLabel) {
        self.labels.insert(name, label);
    }

    #[allow(dead_code)]
    pub fn add_lc(&mut self, label: AsmLabel) {
        self.loc_counter.push(label);
    }

    pub fn export(self) -> String {
        let mut asm = String::new();
        for (k, v) in self.labels {
            asm.push_str(format!("{}:\n", k).as_str());
            asm.push_str(v.export().as_str());
        }

        asm
    }
}

#[derive(Debug)]
pub struct AsmLabel {
    instructions: Vec<String>,
}

impl AsmLabel {
    pub fn new() -> AsmLabel {
        AsmLabel {
            instructions: Vec::new(),
        }
    }

    pub fn mov<S: AsRef<str> + std::fmt::Display, T: AsRef<str> + std::fmt::Display>(
        &mut self,
        lhs: S,
        rhs: T,
    ) {
        self.instructions.push(format!("mov {}, {}", lhs, rhs));
    }

    pub fn push<S: AsRef<str> + std::fmt::Display>(&mut self, val: S) {
        self.instructions.push(format!("push {}", val));
    }

    pub fn add<S: AsRef<str> + std::fmt::Display, T: AsRef<str> + std::fmt::Display>(
        &mut self,
        lhs: S,
        rhs: T,
    ) {
        self.instructions.push(format!("add {}, {}", lhs, rhs));
    }

    pub fn sub<S: AsRef<str> + std::fmt::Display, T: AsRef<str> + std::fmt::Display>(
        &mut self,
        lhs: S,
        rhs: T,
    ) {
        self.instructions.push(format!("sub {}, {}", lhs, rhs));
    }

    pub fn imul<S: AsRef<str> + std::fmt::Display, T: AsRef<str> + std::fmt::Display>(
        &mut self,
        lhs: S,
        rhs: T,
    ) {
        self.instructions.push(format!("imul {}, {}", lhs, rhs));
    }

    pub fn mul<S: AsRef<str> + std::fmt::Display, T: AsRef<str> + std::fmt::Display>(
        &mut self,
        lhs: S,
        rhs: T,
    ) {
        self.instructions.push(format!("mul {}, {}", lhs, rhs));
    }

    pub fn idiv<S: AsRef<str> + std::fmt::Display>(&mut self, lhs: S) {
        self.instructions.push(format!("idiv {}", lhs));
    }

    pub fn div<S: AsRef<str> + std::fmt::Display>(&mut self, lhs: S) {
        self.instructions.push(format!("div {}", lhs));
    }

    pub fn lea<S: AsRef<str> + std::fmt::Display, T: AsRef<str> + std::fmt::Display>(
        &mut self,
        lhs: S,
        rhs: T,
    ) {
        self.instructions.push(format!("lea {}, {}", lhs, rhs));
    }

    pub fn pop<S: AsRef<str> + std::fmt::Display>(&mut self, val: S) {
        self.instructions.push(format!("pop {}", val));
    }

    pub fn cdq(&mut self) {
        self.instructions.push("cdq".to_string());
    }

    pub fn ret(&mut self) {
        self.instructions.push("ret".to_string());
    }

    pub fn export(self) -> String {
        let mut asm = String::new();
        for ins in self.instructions {
            asm.push('\t');
            asm.push_str(&ins);
            asm.push('\n');
        }

        asm
    }
}
