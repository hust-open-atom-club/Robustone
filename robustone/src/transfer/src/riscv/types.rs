//! RISC-V数据类型定义
//!
//! 基于Capstone RISC-V架构定义的数据结构

/// RISC-V指令操作数类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RiscVOperandType {
    /// 无效操作数
    Invalid,
    /// 寄存器操作数
    Register,
    /// 立即数操作数
    Immediate,
    /// 内存操作数
    Memory,
}

/// 内存操作数 (对应 RISCV_OP_MEM)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RiscVMemoryOperand {
    /// 基址寄存器
    pub base: u32,
    /// 位移/偏移值
    pub disp: i64,
}

/// RISC-V操作数 (对应 cs_riscv_op)
#[derive(Debug, Clone, PartialEq)]
pub struct RiscVOperand {
    /// 操作数类型
    pub op_type: RiscVOperandType,
    /// 访问权限
    pub access: Access,
    /// 操作数值
    pub value: RiscVOperandValue,
}

/// 操作数值
#[derive(Debug, Clone, PartialEq)]
pub enum RiscVOperandValue {
    /// 寄存器值
    Register(u32),
    /// 立即数
    Immediate(i64),
    /// 内存引用
    Memory(RiscVMemoryOperand),
}

/// 寄存器访问权限 (对应 cs_ac_type)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Access {
    /// 可读
    pub read: bool,
    /// 可写
    pub write: bool,
}

impl Access {
    pub fn read() -> Self { Self { read: true, write: false } }
    pub fn write() -> Self { Self { read: false, write: true } }
    pub fn read_write() -> Self { Self { read: true, write: true } }
    pub fn none() -> Self { Self { read: false, write: false } }
}

/// RISC-V指令详细信息 (对应 cs_riscv)
#[derive(Debug, Clone)]
pub struct RiscVInstructionDetail {
    /// 是否需要有效地址
    pub need_effective_addr: bool,
    /// 操作数数量
    pub op_count: u8,
    /// 操作数列表 (最多8个)
    pub operands: [RiscVOperand; 8],
    /// 读取的寄存器列表
    pub regs_read: Vec<u32>,
    /// 写入的寄存器列表
    pub regs_write: Vec<u32>,
    /// 指令分组
    pub groups: Vec<String>,
}

impl Default for RiscVInstructionDetail {
    fn default() -> Self {
        Self {
            need_effective_addr: false,
            op_count: 0,
            operands: [RiscVOperand {
                op_type: RiscVOperandType::Invalid,
                access: Access::none(),
                value: RiscVOperandValue::Immediate(0),
            }, RiscVOperand {
                op_type: RiscVOperandType::Invalid,
                access: Access::none(),
                value: RiscVOperandValue::Immediate(0),
            }, RiscVOperand {
                op_type: RiscVOperandType::Invalid,
                access: Access::none(),
                value: RiscVOperandValue::Immediate(0),
            }, RiscVOperand {
                op_type: RiscVOperandType::Invalid,
                access: Access::none(),
                value: RiscVOperandValue::Immediate(0),
            }, RiscVOperand {
                op_type: RiscVOperandType::Invalid,
                access: Access::none(),
                value: RiscVOperandValue::Immediate(0),
            }, RiscVOperand {
                op_type: RiscVOperandType::Invalid,
                access: Access::none(),
                value: RiscVOperandValue::Immediate(0),
            }, RiscVOperand {
                op_type: RiscVOperandType::Invalid,
                access: Access::none(),
                value: RiscVOperandValue::Immediate(0),
            }, RiscVOperand {
                op_type: RiscVOperandType::Invalid,
                access: Access::none(),
                value: RiscVOperandValue::Immediate(0),
            }],
            regs_read: Vec::new(),
            regs_write: Vec::new(),
            groups: Vec::new(),
        }
    }
}

/// RISC-V寄存器枚举 (对应 riscv_reg)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVRegister {
    Invalid = 0,

    // 通用寄存器
    X0 = 1,   // "zero"
    X1 = 2,   // "ra"
    X2 = 3,   // "sp"
    X3 = 4,   // "gp"
    X4 = 5,   // "tp"
    X5 = 6,   // "t0"
    X6 = 7,   // "t1"
    X7 = 8,   // "t2"
    X8 = 9,   // "s0/fp"
    X9 = 10,  // "s1"
    X10 = 11, // "a0"
    X11 = 12, // "a1"
    X12 = 13, // "a2"
    X13 = 14, // "a3"
    X14 = 15, // "a4"
    X15 = 16, // "a5"
    X16 = 17, // "a6"
    X17 = 18, // "a7"
    X18 = 19, // "s2"
    X19 = 20, // "s3"
    X20 = 21, // "s4"
    X21 = 22, // "s5"
    X22 = 23, // "s6"
    X23 = 24, // "s7"
    X24 = 25, // "s8"
    X25 = 26, // "s9"
    X26 = 27, // "s10"
    X27 = 28, // "s11"
    X28 = 29, // "t3"
    X29 = 30, // "t4"
    X30 = 31, // "t5"
    X31 = 32, // "t6"

    // 浮点寄存器 (32位)
    F0_32 = 33,
    F1_32 = 34,
    F2_32 = 35,
    F3_32 = 36,
    F4_32 = 37,
    F5_32 = 38,
    F6_32 = 39,
    F7_32 = 40,
    F8_32 = 41,
    F9_32 = 42,
    F10_32 = 43,
    F11_32 = 44,
    F12_32 = 45,
    F13_32 = 46,
    F14_32 = 47,
    F15_32 = 48,
    F16_32 = 49,
    F17_32 = 50,
    F18_32 = 51,
    F19_32 = 52,
    F20_32 = 53,
    F21_32 = 54,
    F22_32 = 55,
    F23_32 = 56,
    F24_32 = 57,
    F25_32 = 58,
    F26_32 = 59,
    F27_32 = 60,
    F28_32 = 61,
    F29_32 = 62,
    F30_32 = 63,
    F31_32 = 64,

    // 浮点寄存器 (64位)
    F0_64 = 65,
    F1_64 = 66,
    F2_64 = 67,
    F3_64 = 68,
    F4_64 = 69,
    F5_64 = 70,
    F6_64 = 71,
    F7_64 = 72,
    F8_64 = 73,
    F9_64 = 74,
    F10_64 = 75,
    F11_64 = 76,
    F12_64 = 77,
    F13_64 = 78,
    F14_64 = 79,
    F15_64 = 80,
    F16_64 = 81,
    F17_64 = 82,
    F18_64 = 83,
    F19_64 = 84,
    F20_64 = 85,
    F21_64 = 86,
    F22_64 = 87,
    F23_64 = 88,
    F24_64 = 89,
    F25_64 = 90,
    F26_64 = 91,
    F27_64 = 92,
    F28_64 = 93,
    F29_64 = 94,
    F30_64 = 95,
    F31_64 = 96,
}

impl RiscVRegister {
    /// 获取寄存器名称
    pub fn name(self) -> &'static str {
        match self {
            RiscVRegister::Invalid => "invalid",

            // 通用寄存器
            RiscVRegister::X0 => "zero",
            RiscVRegister::X1 => "ra",
            RiscVRegister::X2 => "sp",
            RiscVRegister::X3 => "gp",
            RiscVRegister::X4 => "tp",
            RiscVRegister::X5 => "t0",
            RiscVRegister::X6 => "t1",
            RiscVRegister::X7 => "t2",
            RiscVRegister::X8 => "s0",
            RiscVRegister::X9 => "s1",
            RiscVRegister::X10 => "a0",
            RiscVRegister::X11 => "a1",
            RiscVRegister::X12 => "a2",
            RiscVRegister::X13 => "a3",
            RiscVRegister::X14 => "a4",
            RiscVRegister::X15 => "a5",
            RiscVRegister::X16 => "a6",
            RiscVRegister::X17 => "a7",
            RiscVRegister::X18 => "s2",
            RiscVRegister::X19 => "s3",
            RiscVRegister::X20 => "s4",
            RiscVRegister::X21 => "s5",
            RiscVRegister::X22 => "s6",
            RiscVRegister::X23 => "s7",
            RiscVRegister::X24 => "s8",
            RiscVRegister::X25 => "s9",
            RiscVRegister::X26 => "s10",
            RiscVRegister::X27 => "s11",
            RiscVRegister::X28 => "t3",
            RiscVRegister::X29 => "t4",
            RiscVRegister::X30 => "t5",
            RiscVRegister::X31 => "t6",

            // 浮点寄存器名称
            RiscVRegister::F0_32 => "ft0",
            RiscVRegister::F1_32 => "ft1",
            RiscVRegister::F2_32 => "ft2",
            RiscVRegister::F3_32 => "ft3",
            RiscVRegister::F4_32 => "ft4",
            RiscVRegister::F5_32 => "ft5",
            RiscVRegister::F6_32 => "ft6",
            RiscVRegister::F7_32 => "ft7",
            RiscVRegister::F8_32 => "fs0",
            RiscVRegister::F9_32 => "fs1",
            RiscVRegister::F10_32 => "fa0",
            RiscVRegister::F11_32 => "fa1",
            RiscVRegister::F12_32 => "fa2",
            RiscVRegister::F13_32 => "fa3",
            RiscVRegister::F14_32 => "fa4",
            RiscVRegister::F15_32 => "fa5",
            RiscVRegister::F16_32 => "fa6",
            RiscVRegister::F17_32 => "fa7",
            RiscVRegister::F18_32 => "fs2",
            RiscVRegister::F19_32 => "fs3",
            RiscVRegister::F20_32 => "fs4",
            RiscVRegister::F21_32 => "fs5",
            RiscVRegister::F22_32 => "fs6",
            RiscVRegister::F23_32 => "fs7",
            RiscVRegister::F24_32 => "fs8",
            RiscVRegister::F25_32 => "fs9",
            RiscVRegister::F26_32 => "fs10",
            RiscVRegister::F27_32 => "fs11",
            RiscVRegister::F28_32 => "ft8",
            RiscVRegister::F29_32 => "ft9",
            RiscVRegister::F30_32 => "ft10",
            RiscVRegister::F31_32 => "ft11",

            // 64位浮点寄存器
            RiscVRegister::F0_64 => "ft0",
            RiscVRegister::F1_64 => "ft1",
            RiscVRegister::F2_64 => "ft2",
            RiscVRegister::F3_64 => "ft3",
            RiscVRegister::F4_64 => "ft4",
            RiscVRegister::F5_64 => "ft5",
            RiscVRegister::F6_64 => "ft6",
            RiscVRegister::F7_64 => "ft7",
            RiscVRegister::F8_64 => "fs0",
            RiscVRegister::F9_64 => "fs1",
            RiscVRegister::F10_64 => "fa0",
            RiscVRegister::F11_64 => "fa1",
            RiscVRegister::F12_64 => "fa2",
            RiscVRegister::F13_64 => "fa3",
            RiscVRegister::F14_64 => "fa4",
            RiscVRegister::F15_64 => "fa5",
            RiscVRegister::F16_64 => "fa6",
            RiscVRegister::F17_64 => "fa7",
            RiscVRegister::F18_64 => "fs2",
            RiscVRegister::F19_64 => "fs3",
            RiscVRegister::F20_64 => "fs4",
            RiscVRegister::F21_64 => "fs5",
            RiscVRegister::F22_64 => "fs6",
            RiscVRegister::F23_64 => "fs7",
            RiscVRegister::F24_64 => "fs8",
            RiscVRegister::F25_64 => "fs9",
            RiscVRegister::F26_64 => "fs10",
            RiscVRegister::F27_64 => "fs11",
            RiscVRegister::F28_64 => "ft8",
            RiscVRegister::F29_64 => "ft9",
            RiscVRegister::F30_64 => "ft10",
            RiscVRegister::F31_64 => "ft11",
        }
    }

    /// 从数字ID获取寄存器 (RISC-V寄存器编码：x0=0, x1=1, ..., x31=31)
    pub fn from_id(id: u32) -> Self {
        match id {
            0 => RiscVRegister::X0,
            1 => RiscVRegister::X1,
            2 => RiscVRegister::X2,
            3 => RiscVRegister::X3,
            4 => RiscVRegister::X4,
            5 => RiscVRegister::X5,
            6 => RiscVRegister::X6,
            7 => RiscVRegister::X7,
            8 => RiscVRegister::X8,
            9 => RiscVRegister::X9,
            10 => RiscVRegister::X10,
            11 => RiscVRegister::X11,
            12 => RiscVRegister::X12,
            13 => RiscVRegister::X13,
            14 => RiscVRegister::X14,
            15 => RiscVRegister::X15,
            16 => RiscVRegister::X16,
            17 => RiscVRegister::X17,
            18 => RiscVRegister::X18,
            19 => RiscVRegister::X19,
            20 => RiscVRegister::X20,
            21 => RiscVRegister::X21,
            22 => RiscVRegister::X22,
            23 => RiscVRegister::X23,
            24 => RiscVRegister::X24,
            25 => RiscVRegister::X25,
            26 => RiscVRegister::X26,
            27 => RiscVRegister::X27,
            28 => RiscVRegister::X28,
            29 => RiscVRegister::X29,
            30 => RiscVRegister::X30,
            31 => RiscVRegister::X31,
            _ => RiscVRegister::Invalid,
        }
    }
}

/// RISC-V指令集扩展
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RiscVExtension {
    /// 基础整数指令集
    I,
    /// 乘法和除法扩展
    M,
    /// 原子操作扩展
    A,
    /// 单精度浮点扩展
    F,
    /// 双精度浮点扩展
    D,
    /// 压缩指令扩展
    C,
}

/// RISC-V指令格式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RiscVInstructionFormat {
    /// R型指令 (寄存器-寄存器操作)
    R,
    /// I型指令 (立即数操作)
    I,
    /// S型指令 (存储操作)
    S,
    /// B型指令 (分支操作)
    B,
    /// U型指令 (上位立即数)
    U,
    /// J型指令 (跳转和链接)
    J,
    /// R4型指令 (浮点融合乘加)
    R4,
    /// 压缩指令格式
    CR, CI, CSS, CIW, CL, CS, CA, CB, CJ,
}