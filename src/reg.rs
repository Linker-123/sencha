use crate::error;
use log::debug;

#[derive(PartialEq, Debug, Clone)]
pub enum RegisterSize {
    Byte,
    Word,
    Dword,
    Qword,
    Oword,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RegisterLabel {
    // 128bit XMM registers
    Xmm0,
    Xmm1,
    Xmm2,
    Xmm3,
    Xmm4,
    Xmm5,
    Xmm6,
    Xmm7,
    // 64bit
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    // Lower 32 bits
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
    // Lower 16 bits
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
    R12w,
    R13w,
    R14w,
    R15w,
    // Lower 8 bits
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
    R15b,
}

pub struct Register {
    label: RegisterLabel,
    size: RegisterSize,
    used: bool,
}

impl Register {
    fn new(label: RegisterLabel, size: RegisterSize) -> Register {
        Register {
            label,
            size,
            used: false,
        }
    }
}

pub struct RegisterManager {
    registers: Vec<Register>,
}

impl RegisterManager {
    pub fn new() -> RegisterManager {
        let mut registers: Vec<Register> = Vec::new();
        // 128 BIT
        registers.push(Register::new(RegisterLabel::Xmm0, RegisterSize::Oword));
        registers.push(Register::new(RegisterLabel::Xmm1, RegisterSize::Oword));
        registers.push(Register::new(RegisterLabel::Xmm2, RegisterSize::Oword));
        registers.push(Register::new(RegisterLabel::Xmm3, RegisterSize::Oword));
        registers.push(Register::new(RegisterLabel::Xmm4, RegisterSize::Oword));
        registers.push(Register::new(RegisterLabel::Xmm5, RegisterSize::Oword));
        registers.push(Register::new(RegisterLabel::Xmm6, RegisterSize::Oword));
        registers.push(Register::new(RegisterLabel::Xmm7, RegisterSize::Oword));
        // 64 BIT
        registers.push(Register::new(RegisterLabel::Rax, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::Rbx, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::Rcx, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::Rdx, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::Rsi, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::Rdi, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::Rbp, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::Rsp, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::R8, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::R9, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::R10, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::R11, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::R12, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::R13, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::R14, RegisterSize::Qword));
        registers.push(Register::new(RegisterLabel::R15, RegisterSize::Qword));
        // 32 BIT
        registers.push(Register::new(RegisterLabel::Eax, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::Ebx, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::Ecx, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::Edx, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::Esi, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::Edi, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::Ebp, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::Esp, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::R8d, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::R9d, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::R10d, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::R11d, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::R12d, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::R13d, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::R14d, RegisterSize::Dword));
        registers.push(Register::new(RegisterLabel::R15d, RegisterSize::Dword));
        // 16 BIT
        registers.push(Register::new(RegisterLabel::Ax, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::Bx, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::Cx, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::Dx, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::Si, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::Di, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::Bp, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::Sp, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::R8w, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::R9w, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::R10w, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::R11w, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::R12w, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::R13w, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::R14w, RegisterSize::Word));
        registers.push(Register::new(RegisterLabel::R15w, RegisterSize::Word));
        // 8 BITS
        registers.push(Register::new(RegisterLabel::Al, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::Bl, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::Cl, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::Dl, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::Sil, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::Dil, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::Bpl, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::Spl, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::R8b, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::R9b, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::R10b, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::R11b, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::R12b, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::R13b, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::R14b, RegisterSize::Byte));
        registers.push(Register::new(RegisterLabel::R15b, RegisterSize::Byte));
        RegisterManager { registers }
    }

    pub fn allocate(&mut self, size: RegisterSize) -> RegisterLabel {
        for reg in &mut self.registers {
            if !reg.used && reg.size == size {
                reg.used = true;
                debug!("Allocated a register: {:#?}", reg.label);
                return reg.label.clone();
            }
        }

        error::panic(format!(
            "Couldn't find a free register of size: {:#?}",
            size
        ))
    }

    pub fn deallocate(&mut self, label: RegisterLabel) {
        for reg in &mut self.registers {
            if reg.used && reg.label == label {
                reg.used = false;
                debug!("Deallocated a register: {:#?}", reg.label);
                return;
            }
        }
        error::panic(format!(
            "Tried to deallocate a non-used register: {:#?}",
            label
        ))
    }

    #[allow(dead_code)]
    pub fn table(&self, size: Option<RegisterSize>) {
        println!("| {: <10} | {: <10} | {: <6}|", "Label:", "Size:", "Used:");
        for reg in &self.registers {
            if let Some(s) = &size {
                if reg.size != *s {
                    continue;
                }
            }

            let label = format!("{:#?}", reg.label);
            let size = format!("{:#?}", reg.size);
            let used = format!("{:#?}", reg.used);
            println!("| {: <10} | {: <10} | {: <6}|", label, size, used);
        }
    }
}

pub fn size_to_reg_size(bytes: usize) -> RegisterSize {
    match bytes {
        1 => RegisterSize::Byte,
        2 => RegisterSize::Word,
        4 => RegisterSize::Dword,
        8 => RegisterSize::Qword,
        16 => RegisterSize::Oword,
        _ => error::panic(format!("No register size for {} bytes", bytes)),
    }
}

pub fn reg_size_to_str(size: RegisterSize) -> &'static str {
    match size {
        RegisterSize::Byte => "byte",
        RegisterSize::Word => "word",
        RegisterSize::Dword => "dword",
        RegisterSize::Qword => "qword",
        RegisterSize::Oword => "oword",
    }
}
