#![forbid(unsafe_code)]

//! Proc-macro crate for the Robustone ISA backend framework.
//!
//! Provides declarative macros that architecture crates use to define
//! their instruction sets, register banks, formats, and aliases.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, LitStr, Token, Type, braced};

// ============================================================================
// define_arch!
// ============================================================================

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
    let parsed = syn::parse_macro_input!(input as DefineArchInput);
    let vis = &parsed.vis;
    let name = &parsed.name;
    let word_ty = &parsed.word;
    let modes_name = Ident::new(&format!("{}Mode", name), Span::call_site());
    let feature_name = Ident::new(&format!("{}Feature", name), Span::call_site());
    let backend_name = Ident::new(&format!("{}Backend", name), Span::call_site());

    let mode_variants: Vec<_> = parsed
        .modes
        .iter()
        .map(|m| {
            let variant = &m.name;
            quote! { #variant }
        })
        .collect();

    let feature_bits: Vec<_> = parsed
        .features
        .bits
        .iter()
        .map(|b| {
            let name = &b.name;
            let value = &b.value;
            quote! { const #name = 1 << #value; }
        })
        .collect();

    let feature_ty = &parsed.features.ty;

    let output = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #vis enum #modes_name {
            #(#mode_variants),*
        }

        ::bitflags::bitflags! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #vis struct #feature_name: #feature_ty {
                #(#feature_bits)*
            }
        }

        impl ::robustone_isa::FeatureSet for #feature_name {
            fn empty() -> Self { Self::empty() }
            fn all_supported_for_tests() -> Self { Self::all() }
            fn contains(self, required: Self) -> bool { self.bits() & required.bits() == required.bits() }
        }

        #vis struct #backend_name;

        impl ::robustone_isa::ArchitectureBackend for #backend_name {
            type Word = #word_ty;
            type Mode = #modes_name;
            type Feature = #feature_name;
            // Field, RegisterClass, and remaining trait methods must be provided
            // by the architecture crate after invoking define_arch!.
        }
    };

    output.into()
}

struct DefineArchInput {
    vis: syn::Visibility,
    _arch_token: Ident,
    name: Ident,
    _brace: syn::token::Brace,
    word: Type,
    modes: Vec<ModeDef>,
    features: FeatureDef,
}

struct ModeDef {
    name: Ident,
    _eq: Token![=],
    _value: LitStr,
}

struct FeatureDef {
    ty: Type,
    bits: Vec<FeatureBit>,
}

struct FeatureBit {
    name: Ident,
    _eq: Token![=],
    value: syn::LitInt,
}

impl Parse for DefineArchInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis: syn::Visibility = input.parse()?;
        let _arch_token: Ident = input.parse()?;
        let name: Ident = input.parse()?;
        let content;
        let _brace = braced!(content in input);

        // word = <type>;
        let word_kw: Ident = content.parse()?;
        if word_kw != "word" {
            return Err(syn::Error::new(word_kw.span(), "expected `word`"));
        }
        let _eq: Token![=] = content.parse()?;
        let word: Type = content.parse()?;
        let _semi: Token![;] = content.parse()?;

        // endian = <ident>;
        let endian_kw: Ident = content.parse()?;
        if endian_kw != "endian" {
            return Err(syn::Error::new(endian_kw.span(), "expected `endian`"));
        }
        let _eq: Token![=] = content.parse()?;
        let _endian: Ident = content.parse()?;
        let _semi: Token![;] = content.parse()?;

        // instruction_length = <ident>(<lit>);
        let ilen_kw: Ident = content.parse()?;
        if ilen_kw != "instruction_length" {
            return Err(syn::Error::new(
                ilen_kw.span(),
                "expected `instruction_length`",
            ));
        }
        let _eq: Token![=] = content.parse()?;
        let _ilen_kind: Ident = content.parse()?;
        let _paren;
        syn::parenthesized!(_paren in content);
        let _ilen_val: syn::LitInt = content.parse()?;
        let _semi: Token![;] = content.parse()?;

        // modes { ... }
        let modes_kw: Ident = content.parse()?;
        if modes_kw != "modes" {
            return Err(syn::Error::new(modes_kw.span(), "expected `modes`"));
        }
        let modes_content;
        braced!(modes_content in content);
        let mut modes = Vec::new();
        while !modes_content.is_empty() {
            let name: Ident = modes_content.parse()?;
            let _eq: Token![=] = modes_content.parse()?;
            let _value: LitStr = modes_content.parse()?;
            let _semi: Token![;] = modes_content.parse()?;
            modes.push(ModeDef { name, _eq, _value });
        }
        let _semi: Token![;] = content.parse()?;

        // features: <type> { ... }
        let features_kw: Ident = content.parse()?;
        if features_kw != "features" {
            return Err(syn::Error::new(features_kw.span(), "expected `features`"));
        }
        let _colon: Token![:] = content.parse()?;
        let ty: Type = content.parse()?;
        let bits_content;
        braced!(bits_content in content);
        let mut bits = Vec::new();
        while !bits_content.is_empty() {
            let name: Ident = bits_content.parse()?;
            let _eq: Token![=] = bits_content.parse()?;
            let value: syn::LitInt = bits_content.parse()?;
            let _semi: Token![;] = bits_content.parse()?;
            bits.push(FeatureBit { name, _eq, value });
        }
        let _semi: Token![;] = content.parse()?;

        // registers = <path>;
        let _registers_kw: Ident = content.parse()?;
        let _eq: Token![=] = content.parse()?;
        let _registers_path: syn::Path = content.parse()?;
        let _semi: Token![;] = content.parse()?;

        // formats = <path>;
        let _formats_kw: Ident = content.parse()?;
        let _eq: Token![=] = content.parse()?;
        let _formats_path: syn::Path = content.parse()?;
        let _semi: Token![;] = content.parse()?;

        // specs = <path>;
        let _specs_kw: Ident = content.parse()?;
        let _eq: Token![=] = content.parse()?;
        let _specs_path: syn::Path = content.parse()?;
        let _semi: Token![;] = content.parse()?;

        // render = <path>;
        let _render_kw: Ident = content.parse()?;
        let _eq: Token![=] = content.parse()?;
        let _render_path: syn::Path = content.parse()?;
        let _semi: Token![;] = content.parse()?;

        Ok(DefineArchInput {
            vis,
            _arch_token,
            name,
            _brace,
            word,
            modes,
            features: FeatureDef { ty, bits },
        })
    }
}

// ============================================================================
// define_registers!
// ============================================================================

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
    let _parsed = syn::parse_macro_input!(input as DefineRegistersInput);
    quote! {
        // Register bank metadata is stored as a static table for now.
        // Full implementation will expand in a subsequent round.
    }
    .into()
}

struct DefineRegistersInput;

impl Parse for DefineRegistersInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // arch = <ident>;
        let _arch_kw: Ident = input.parse()?;
        let _eq: Token![=] = input.parse()?;
        let _arch_name: Ident = input.parse()?;
        let _semi: Token![;] = input.parse()?;

        // Minimal parse: consume remaining tokens as register bank definitions
        while !input.is_empty() {
            let _bank_kw: Ident = input.parse()?;
            let _bank_name: Ident = input.parse()?;
            let _content;
            braced!(_content in input);
        }

        Ok(DefineRegistersInput)
    }
}

// ============================================================================
// define_formats!
// ============================================================================

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
    let parsed = syn::parse_macro_input!(input as DefineFormatsInput);
    let arch = &parsed.arch;
    let field_enum = Ident::new(&format!("{}Field", arch), Span::call_site());

    let format_statics: Vec<_> = parsed.formats.iter().map(|f| {
        let name = &f.name;
        let field_specs: Vec<_> = f.fields.iter().map(|field| {
            let fname = &field.name;
            let start = &field.start;
            let length = &field.length;
            let name_str = fname.to_string();
            let field_type_str = if name_str.is_empty() {
                String::new()
            } else {
                let mut chars = name_str.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                    None => String::new(),
                }
            };
            let field_type = Ident::new(&field_type_str, Span::call_site());
            quote! {
                ::robustone_isa::field(stringify!(#fname), #start, #length, #field_enum::#field_type)
            }
        }).collect();

        quote! {
            pub static #name: ::robustone_isa::FormatSpec<#field_enum> = ::robustone_isa::FormatSpec {
                name: stringify!(#name),
                fields: &[#(#field_specs),*],
            };
        }
    }).collect();

    quote! {
        #(#format_statics)*
    }
    .into()
}

struct DefineFormatsInput {
    arch: Ident,
    formats: Vec<FormatDef>,
}

struct FormatDef {
    name: Ident,
    fields: Vec<FormatFieldDef>,
}

struct FormatFieldDef {
    name: Ident,
    _colon: Token![:],
    _bits_fn: Ident,
    start: syn::LitInt,
    _comma: Token![,],
    length: syn::LitInt,
}

impl Parse for DefineFormatsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // arch = <ident>;
        let _arch_kw: Ident = input.parse()?;
        let _eq: Token![=] = input.parse()?;
        let arch: Ident = input.parse()?;
        let _semi: Token![;] = input.parse()?;

        // fields { ... } - consume but don't use yet
        let _fields_kw: Ident = input.parse()?;
        let _fields_content;
        braced!(_fields_content in input);
        while !_fields_content.is_empty() {
            let _field_name: Ident = _fields_content.parse()?;
            if !_fields_content.is_empty() {
                let _semi: Token![;] = _fields_content.parse()?;
            }
        }
        let _semi: Token![;] = input.parse()?;

        // Parse format definitions
        let mut formats = Vec::new();
        while !input.is_empty() {
            let _format_kw: Ident = input.parse()?;
            let name: Ident = input.parse()?;
            let content;
            braced!(content in input);

            let mut fields = Vec::new();
            while !content.is_empty() {
                let field_name: Ident = content.parse()?;
                let _colon: Token![:] = content.parse()?;
                let bits_fn: Ident = content.parse()?;
                if bits_fn != "bits" {
                    return Err(syn::Error::new(bits_fn.span(), "expected `bits`"));
                }
                let paren;
                syn::parenthesized!(paren in content);
                let start: syn::LitInt = paren.parse()?;
                let _comma: Token![,] = paren.parse()?;
                let length: syn::LitInt = paren.parse()?;
                if !content.is_empty() {
                    let _comma: Token![,] = content.parse()?;
                }
                fields.push(FormatFieldDef {
                    name: field_name,
                    _colon,
                    _bits_fn: bits_fn,
                    start,
                    _comma,
                    length,
                });
            }

            formats.push(FormatDef { name, fields });

            if !input.is_empty() {
                let _semi: Token![;] = input.parse()?;
            }
        }

        Ok(DefineFormatsInput { arch, formats })
    }
}

// ============================================================================
// define_instructions!
// ============================================================================

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
    let parsed = syn::parse_macro_input!(input as DefineInstructionsInput);
    let arch = &parsed.arch;
    let backend = Ident::new(&format!("{}Backend", arch), Span::call_site());
    let _feature = Ident::new(&format!("{}Feature", arch), Span::call_site());
    let _mode = Ident::new(&format!("{}Mode", arch), Span::call_site());
    let _field_enum = Ident::new(&format!("{}Field", arch), Span::call_site());
    let _reg_class = Ident::new(&format!("{}RegisterClass", arch), Span::call_site());

    let spec_statics: Vec<_> = parsed.instructions.iter().map(|insn| {
        let name = &insn.name;
        let mnemonic = &insn.mnemonic;
        let opcode_id = &insn.opcode_id;
        let pattern_expr = &insn.pattern;
        let format_expr = &insn.format;
        let operands_expr = &insn.operands;
        let features_expr = &insn.features;
        let modes_expr = &insn.modes;
        let groups_expr = &insn.groups;
        let manual_ref = if let Some(manual) = &insn.manual {
            quote! { Some(#manual) }
        } else {
            quote! { None }
        };
        let priority = if let Some(p) = &insn.priority {
            quote! { #p }
        } else {
            quote! { 0 }
        };

        quote! {
            pub static #name: ::robustone_isa::InstructionSpec<#backend> = ::robustone_isa::InstructionSpec {
                mnemonic: #mnemonic,
                opcode_id: #opcode_id,
                pattern: #pattern_expr,
                format: #format_expr,
                operands: #operands_expr,
                features: #features_expr,
                modes: #modes_expr,
                groups: #groups_expr,
                manual_ref: #manual_ref,
                priority: #priority,
            };
        }
    }).collect();

    // Generate compile-time validation: duplicate opcode_id check
    let opcode_ids: Vec<_> = parsed.instructions.iter().map(|i| &i.opcode_id).collect();

    let mut duplicates = Vec::new();
    for (i, a) in opcode_ids.iter().enumerate() {
        for (j, b) in opcode_ids.iter().enumerate() {
            if i < j && a.value() == b.value() {
                duplicates.push((a.value(), i, j));
            }
        }
    }

    let duplicate_checks: Vec<_> = duplicates
        .iter()
        .map(|(id, _, _)| {
            let msg = format!("duplicate opcode_id: {}", id);
            quote! {
                const _: () = ::core::compile_error!(#msg);
            }
        })
        .collect();

    // Generate specs array
    let spec_names: Vec<_> = parsed.instructions.iter().map(|i| &i.name).collect();

    quote! {
        #(#duplicate_checks)*

        #(#spec_statics)*

        pub static SPECS: &[::robustone_isa::InstructionSpec<#backend>] = &[
            #(#spec_names),*
        ];
    }
    .into()
}

struct DefineInstructionsInput {
    arch: Ident,
    _module_kw: Ident,
    _module_name: Ident,
    instructions: Vec<InstructionDef>,
}

struct InstructionDef {
    name: Ident,
    mnemonic: LitStr,
    opcode_id: LitStr,
    pattern: Expr,
    format: Expr,
    operands: Expr,
    features: Expr,
    modes: Expr,
    groups: Expr,
    manual: Option<LitStr>,
    priority: Option<syn::LitInt>,
}

impl Parse for DefineInstructionsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // arch = <ident>; module = <ident>;
        let _arch_kw: Ident = input.parse()?;
        let _eq: Token![=] = input.parse()?;
        let arch: Ident = input.parse()?;
        let _semi: Token![;] = input.parse()?;
        let _module_kw: Ident = input.parse()?;
        let _eq: Token![=] = input.parse()?;
        let _module_name: Ident = input.parse()?;
        let _semi: Token![;] = input.parse()?;

        let mut instructions = Vec::new();
        while !input.is_empty() {
            let _insn_kw: Ident = input.parse()?;
            let name: Ident = input.parse()?;
            let content;
            braced!(content in input);

            let mut mnemonic = None;
            let mut opcode_id = None;
            let mut pattern = None;
            let mut format = None;
            let mut operands = None;
            let mut features = None;
            let mut modes = None;
            let mut groups = None;
            let mut manual = None;
            let mut priority = None;

            while !content.is_empty() {
                let key: Ident = content.parse()?;
                let _eq: Token![=] = content.parse()?;
                match key.to_string().as_str() {
                    "mnemonic" => mnemonic = Some(content.parse()?),
                    "opcode_id" => opcode_id = Some(content.parse()?),
                    "pattern" => pattern = Some(content.parse()?),
                    "format" => format = Some(content.parse()?),
                    "operands" => operands = Some(content.parse()?),
                    "features" => features = Some(content.parse()?),
                    "modes" => modes = Some(content.parse()?),
                    "groups" => groups = Some(content.parse()?),
                    "manual" => manual = Some(content.parse()?),
                    "priority" => priority = Some(content.parse()?),
                    other => {
                        return Err(syn::Error::new(
                            key.span(),
                            format!("unknown field: {}", other),
                        ));
                    }
                }
                let _semi: Token![;] = content.parse()?;
            }

            let name_clone = name.clone();
            instructions.push(InstructionDef {
                name,
                mnemonic: mnemonic
                    .ok_or_else(|| syn::Error::new(name_clone.span(), "missing mnemonic"))?,
                opcode_id: opcode_id
                    .ok_or_else(|| syn::Error::new(name_clone.span(), "missing opcode_id"))?,
                pattern: pattern
                    .ok_or_else(|| syn::Error::new(name_clone.span(), "missing pattern"))?,
                format: format
                    .ok_or_else(|| syn::Error::new(name_clone.span(), "missing format"))?,
                operands: operands
                    .ok_or_else(|| syn::Error::new(name_clone.span(), "missing operands"))?,
                features: features
                    .ok_or_else(|| syn::Error::new(name_clone.span(), "missing features"))?,
                modes: modes.ok_or_else(|| syn::Error::new(name_clone.span(), "missing modes"))?,
                groups: groups
                    .ok_or_else(|| syn::Error::new(name_clone.span(), "missing groups"))?,
                manual,
                priority,
            });
        }

        Ok(DefineInstructionsInput {
            arch,
            _module_kw,
            _module_name,
            instructions,
        })
    }
}

// ============================================================================
// define_aliases!
// ============================================================================

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
    let _parsed = syn::parse_macro_input!(input as DefineAliasesInput);
    quote! {
        // Alias table will be expanded in a subsequent round.
        // For now, aliases are applied post-decode in the architecture handler.
    }
    .into()
}

struct DefineAliasesInput;

impl Parse for DefineAliasesInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // arch = <ident>;
        let _arch_kw: Ident = input.parse()?;
        let _eq: Token![=] = input.parse()?;
        let _arch_name: Ident = input.parse()?;
        let _semi: Token![;] = input.parse()?;

        // Consume remaining alias definitions
        while !input.is_empty() {
            let _alias_kw: Ident = input.parse()?;
            let _alias_name: Ident = input.parse()?;
            let _for_kw: Ident = input.parse()?;
            let _base_name: Ident = input.parse()?;
            let _content;
            braced!(_content in input);
        }

        Ok(DefineAliasesInput)
    }
}
