#![forbid(unsafe_code)]

//! Proc-macro crate for the Robustone ISA backend framework.
//!
//! Provides declarative macros that architecture crates use to define
//! their instruction sets, register banks, formats, and aliases.

use proc_macro::TokenStream;
use quote::quote;

/// Define architecture-level facts (word type, modes, features, profiles).
///
/// Example:
/// ```ignore
/// define_arch! {
///     pub arch LoongArch {
///         word = u32;
///         endian = little;
///         instruction_length = fixed(4);
///         modes { LA32 = "loongarch32"; LA64 = "loongarch64"; }
///         features: u128 { BASE = 0; LA64 = 1; FLOAT32 = 2; }
///         registers = loongarch_registers;
///         formats = loongarch_formats;
///         specs = loongarch_specs;
///         render = LoongArchRenderPolicy;
///     }
/// }
/// ```
#[proc_macro]
pub fn define_arch(input: TokenStream) -> TokenStream {
    // Stub: full implementation will be expanded in subsequent rounds.
    let _ = input;
    quote! {
        compile_error!("define_arch! is not yet fully implemented; stub used in Round 0");
    }
    .into()
}

/// Define register banks for an architecture.
///
/// Example:
/// ```ignore
/// define_registers! {
///     arch = LoongArch;
///     bank Gpr { count = 32; prefix = "$r"; aliases { 0 = "$zero"; 1 = "$ra"; } }
/// }
/// ```
#[proc_macro]
pub fn define_registers(input: TokenStream) -> TokenStream {
    let _ = input;
    quote! {
        compile_error!("define_registers! is not yet fully implemented; stub used in Round 0");
    }
    .into()
}

/// Define instruction formats and field layouts.
///
/// Example:
/// ```ignore
/// define_formats! {
///     arch = LoongArch;
///     fields { rd; rj; rk; si12; }
///     format R3 { rd: bits(0, 5), rj: bits(5, 5), rk: bits(10, 5) }
/// }
/// ```
#[proc_macro]
pub fn define_formats(input: TokenStream) -> TokenStream {
    let _ = input;
    quote! {
        compile_error!("define_formats! is not yet fully implemented; stub used in Round 0");
    }
    .into()
}

/// Define instruction specifications for an architecture.
///
/// Example:
/// ```ignore
/// define_instructions! {
///     arch = LoongArch; module = base;
///     insn add_w {
///         mnemonic = "add.w"; opcode_id = ADD_W;
///         pattern = mask_value(0xFF00_0000, 0x0100_0000);
///         format = R3;
///         operands = [reg(Gpr, rd, Write), reg(Gpr, rj, Read), reg(Gpr, rk, Read)];
///         modes = [LA32, LA64]; features = [BASE];
///         groups = [Integer, Arithmetic];
///         manual = "LoongArch Vol.1";
///     }
/// }
/// ```
#[proc_macro]
pub fn define_instructions(input: TokenStream) -> TokenStream {
    let _ = input;
    quote! {
        compile_error!("define_instructions! is not yet fully implemented; stub used in Round 0");
    }
    .into()
}

/// Define instruction aliases.
///
/// Example:
/// ```ignore
/// define_aliases! {
///     arch = LoongArch;
///     alias nop for andi { when [rd == 0, rj == 0, si12 == 0]; mnemonic = "nop"; visible_operands = []; }
/// }
/// ```
#[proc_macro]
pub fn define_aliases(input: TokenStream) -> TokenStream {
    let _ = input;
    quote! {
        compile_error!("define_aliases! is not yet fully implemented; stub used in Round 0");
    }
    .into()
}
