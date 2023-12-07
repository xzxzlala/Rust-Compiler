#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Reg {
    Rax,
    Rbx,
    Rdx,
    Rcx,
    Rsp,
    Rbp,
    Rsi,
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FpuReg {
    ST0,
    ST1,
    ST2,
    ST3,
    ST4,
    ST5,
    ST6,
    ST7,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemRef {
    pub reg: Reg,
    pub offset: Offset,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Offset {
    Constant(i32),
    Computed {
        reg: Reg,
        factor: i32,
        constant: i32,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Arg64 {
    Reg(Reg),
    Signed(i64),
    Unsigned(u64),
    Mem(MemRef),
    Label(String),
    FpuReg(FpuReg),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arg32 {
    Reg(Reg),
    Signed(i32),
    Unsigned(u32),
    Mem(MemRef),
    FpuReg(FpuReg),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reg32 {
    Reg(Reg),
    Signed(i32),
    Unsigned(u32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MovArgs {
    ToReg(Reg, Arg64),
    ToMem(MemRef, Reg32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinArgs {
    ToReg(Reg, Arg32),
    ToMem(MemRef, Reg32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JmpArg {
    Label(String),
    Reg(Reg),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FpuInstr {
    /// Faddp - Adds the floating-point value in ST(0) to the value in the specified register,
    /// stores the result in ST(0), and then pops the register stack.
    /// 
    /// Usage: Faddp(Arg32::FpuReg(FpuReg::ST1)) performs ST(0) = ST(0) + ST(1) and pops the FPU stack.
    Faddp(Arg32),

    /// Fsubp - Subtracts the floating-point value in the specified register from the value in ST(0),
    /// stores the result in ST(0), and then pops the register stack.
    /// 
    /// Usage: Fsubp(Arg32::FpuReg(FpuReg::ST1)) performs ST(0) = ST(0) - ST(1) and pops the FPU stack.
    Fsubp(Arg32),

    /// Fmulp - Multiplies the floating-point value in ST(0) with the value in the specified register,
    /// stores the result in ST(0), and then pops the register stack.
    /// 
    /// Usage: Fmulp(Arg32::FpuReg(FpuReg::ST1)) performs ST(0) = ST(0) * ST(1) and pops the FPU stack.
    Fmulp(Arg32),

    /// Fld - Loads a floating-point value from the specified memory location or register
    /// onto the top of the FPU stack (ST(0)).
    /// 
    /// Usage: Fld(Arg32::Mem(mem_ref)) loads a float from memory push it onto the FPU stack.
    Fld(Arg32),
    
    /// Fstp - Copies the floating-point value in ST(0) to the specified memory location or register.
    /// Pops the register stack after the copy is made.
    /// 
    /// Usage: Fstp(Arg32::Mem(mem_ref)) stores the float from ST(0) into memory and pops the FPU stack.
    /// Usage: Fstp(Arg32::FpuReg(FpuReg::ST1)) stores the float from ST(0) into ST(1) and pops the FPU stack.
    Fstp(Arg32),

    /// Fcomip - Compares the floating-point value in ST(0) with the value in the specified register.
    /// Sets the CPU flags based on the result and then pops the FPU stack.
    /// 
    /// Usage: Fcomip(Arg32::FpuReg(FpuReg::ST1)) compares ST(0) with ST(1), sets flags, and pops register stack.
    Fcomip(Arg32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instr {
    Mov(MovArgs),
    RelativeLoadAddress(Reg, String),

    Add(BinArgs),
    Sub(BinArgs),
    IMul(BinArgs),
    And(BinArgs),
    Or(BinArgs),
    Xor(BinArgs),
    Shr(BinArgs),
    Sar(BinArgs),
    Shl(BinArgs),
    Cmp(BinArgs),
    Test(BinArgs),

    Push(Arg32),
    Pop(Arg32),

    Comment(String),
    Label(String),

    Call(JmpArg),
    Ret,

    Jmp(JmpArg),
    Je(JmpArg),
    Jne(JmpArg),
    Jl(JmpArg),
    Jle(JmpArg),
    Jg(JmpArg),
    Jge(JmpArg),

    Jz(JmpArg),
    Jnz(JmpArg),
    Jo(JmpArg),
    Jno(JmpArg),

    Fpu(FpuInstr),
}

pub fn reg_to_string(r: Reg) -> String {
    match r {
        Reg::Rax => String::from("rax"),
        Reg::Rbx => String::from("rbx"),
        Reg::Rcx => String::from("rcx"),
        Reg::Rdx => String::from("rdx"),
        Reg::Rsi => String::from("rsi"),
        Reg::Rdi => String::from("rdi"),
        Reg::Rsp => String::from("rsp"),
        Reg::Rbp => String::from("rbp"),
        Reg::R8 => String::from("r8"),
        Reg::R9 => String::from("r9"),
        Reg::R10 => String::from("r10"),
        Reg::R11 => String::from("r11"),
        Reg::R12 => String::from("r12"),
        Reg::R13 => String::from("r13"),
        Reg::R14 => String::from("r14"),
        Reg::R15 => String::from("r15"),
    }
}

fn imm32_to_string(i: i32) -> String {
    i.to_string()
}

fn offset_to_string(off: Offset) -> String {
    match off {
        Offset::Constant(n) => format!("{}", n),
        Offset::Computed {
            reg,
            factor,
            constant,
        } => format!("{} * {} + {}", reg_to_string(reg), factor, constant),
    }
}
pub fn mem_ref_to_string(m: MemRef) -> String {
    format!(
        "QWORD [{} + {}]",
        reg_to_string(m.reg),
        offset_to_string(m.offset)
    )
}

fn reg32_to_string(r_or_i: Reg32) -> String {
    match r_or_i {
        Reg32::Reg(r) => reg_to_string(r),
        Reg32::Signed(i) => i.to_string(),
        Reg32::Unsigned(u) => format!("0x{:08x}", u),
    }
}

fn fpu_reg_to_string(fpu_reg: FpuReg) -> String {
    match fpu_reg {
        FpuReg::ST0 => String::from("st(0)"),
        FpuReg::ST1 => String::from("st(1)"),
        FpuReg::ST2 => String::from("st(2)"),
        FpuReg::ST3 => String::from("st(3)"),
        FpuReg::ST4 => String::from("st(4)"),
        FpuReg::ST5 => String::from("st(5)"),
        FpuReg::ST6 => String::from("st(6)"),
        FpuReg::ST7 => String::from("st(7)"),
    }
}

fn arg32_to_string(arg: Arg32) -> String {
    match arg {
        Arg32::Reg(r) => reg_to_string(r),
        Arg32::Signed(i) => imm32_to_string(i),
        Arg32::Unsigned(u) => format!("0x{:08x}", u),
        Arg32::Mem(m) => mem_ref_to_string(m),
        Arg32::FpuReg(fpu_reg) => fpu_reg_to_string(fpu_reg),
    }
}

fn arg64_to_string(arg: &Arg64) -> String {
    match arg {
        Arg64::Reg(r) => reg_to_string(*r),
        Arg64::Signed(i) => i.to_string(),
        Arg64::Unsigned(u) => format!("0x{:016x}", u),
        Arg64::Mem(m) => mem_ref_to_string(*m),
        Arg64::Label(l) => l.clone(),
        Arg64::FpuReg(fpu_reg) => fpu_reg_to_string(*fpu_reg),
    }
}

fn mov_args_to_string(args: &MovArgs) -> String {
    match args {
        MovArgs::ToReg(r, arg) => {
            format!("{}, {}", reg_to_string(*r), arg64_to_string(arg))
        }
        MovArgs::ToMem(mem, arg) => {
            format!("{}, {}", mem_ref_to_string(*mem), reg32_to_string(*arg))
        }
    }
}

fn bin_args_to_string(args: BinArgs) -> String {
    match args {
        BinArgs::ToReg(r, arg) => {
            format!("{}, {}", reg_to_string(r), arg32_to_string(arg))
        }
        BinArgs::ToMem(mem, arg) => {
            format!("{}, {}", mem_ref_to_string(mem), reg32_to_string(arg))
        }
    }
}

fn jmp_arg_to_string(arg: &JmpArg) -> String {
    match arg {
        JmpArg::Label(s) => s.clone(),
        JmpArg::Reg(r) => reg_to_string(*r),
    }
}

fn instr_to_string(i: &Instr) -> String {
    match i {
        Instr::RelativeLoadAddress(reg, label) => {
            format!("        lea {}, [rel {}]", reg_to_string(*reg), label)
        }
        Instr::Mov(args) => {
            format!("        mov {}", mov_args_to_string(args))
        }
        Instr::Add(args) => {
            format!("        add {}", bin_args_to_string(*args))
        }
        Instr::Sub(args) => {
            format!("        sub {}", bin_args_to_string(*args))
        }
        Instr::Ret => {
            format!("        ret")
        }
        Instr::IMul(args) => {
            format!("        imul {}", bin_args_to_string(*args))
        }
        Instr::And(args) => {
            format!("        and {}", bin_args_to_string(*args))
        }
        Instr::Or(args) => {
            format!("        or {}", bin_args_to_string(*args))
        }
        Instr::Xor(args) => {
            format!("        xor {}", bin_args_to_string(*args))
        }
        Instr::Shr(args) => {
            format!("        shr {}", bin_args_to_string(*args))
        }
        Instr::Sar(args) => {
            format!("        sar {}", bin_args_to_string(*args))
        }
        Instr::Shl(args) => {
            format!("        shl {}", bin_args_to_string(*args))
        }
        Instr::Cmp(args) => {
            format!("        cmp {}", bin_args_to_string(*args))
        }
        Instr::Test(args) => {
            format!("        test {}", bin_args_to_string(*args))
        }
        Instr::Push(arg) => {
            format!("        push {}", arg32_to_string(*arg))
        }
        Instr::Pop(arg) => {
            format!("        pop {}", arg32_to_string(*arg))
        }
        Instr::Label(s) => {
            format!("{}:", s)
        }
        Instr::Comment(s) => {
            format!(";;; {}", s)
        }
        Instr::Jmp(s) => {
            format!("        jmp {}", jmp_arg_to_string(s))
        }
        Instr::Call(s) => {
            format!("        call {}", jmp_arg_to_string(s))
        }
        Instr::Je(s) => {
            format!("        je {}", jmp_arg_to_string(s))
        }
        Instr::Jne(s) => {
            format!("        jne {}", jmp_arg_to_string(s))
        }
        Instr::Jle(s) => {
            format!("        jle {}", jmp_arg_to_string(s))
        }
        Instr::Jl(s) => {
            format!("        jl {}", jmp_arg_to_string(s))
        }
        Instr::Jg(s) => {
            format!("        jg {}", jmp_arg_to_string(s))
        }
        Instr::Jge(s) => {
            format!("        jge {}", jmp_arg_to_string(s))
        }
        Instr::Jz(s) => {
            format!("        jz {}", jmp_arg_to_string(s))
        }
        Instr::Jnz(s) => {
            format!("        jnz {}", jmp_arg_to_string(s))
        }
        Instr::Jo(s) => {
            format!("        jo {}", jmp_arg_to_string(s))
        }
        Instr::Jno(s) => {
            format!("        jno {}", jmp_arg_to_string(s))
        }
        Instr::Fpu(fpu_instr) => {
            match fpu_instr {
                FpuInstr::Faddp(arg) => {
                    format!("        faddp {}", arg32_to_string(*arg))
                }
                FpuInstr::Fsubp(arg) => {
                    format!("        fsubp {}", arg32_to_string(*arg))
                }
                FpuInstr::Fmulp(arg) => {
                    format!("        fmulp {}", arg32_to_string(*arg))
                }
                FpuInstr::Fld(arg) => {
                    format!("        fld {}", arg32_to_string(*arg))
                }
                FpuInstr::Fstp(arg) => {
                    format!("        fstp {}", arg32_to_string(*arg))
                }
                FpuInstr::Fcomip(arg) => {
                    format!("        fcomip {}", arg32_to_string(*arg))
                }
            }
        }
    }
}

pub fn instrs_to_string(is: &[Instr]) -> String {
    let mut buf = String::new();
    for i in is {
        buf.push_str(&instr_to_string(&i));
        buf.push_str("\n");
    }
    buf
}
