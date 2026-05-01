#![forbid(unsafe_code)]

//! Proc-macro crate for the Robustone ISA backend framework.
//!
//! Provides declarative macros that architecture crates use to define
//! their instruction sets, register banks, formats, and aliases.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, LitInt, LitStr, Token, Type, braced};

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
    let _word_ty = &parsed.word;
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

    // Compile-time validation: duplicate feature bit values
    let mut duplicate_feature_checks = Vec::new();
    for i in 0..parsed.features.bits.len() {
        for j in (i + 1)..parsed.features.bits.len() {
            let a = &parsed.features.bits[i];
            let b = &parsed.features.bits[j];
            if a.value.base10_digits() == b.value.base10_digits() {
                let msg = format!(
                    "duplicate feature bit: {} and {} both use bit {}",
                    a.name, b.name, a.value
                );
                duplicate_feature_checks.push(quote! {
                    const _: () = ::core::compile_error!(#msg);
                });
            }
        }
    }

    let output = quote! {
        #(#duplicate_feature_checks)*

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
        let _ilen_val: syn::LitInt = _paren.parse()?;
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
    let parsed = syn::parse_macro_input!(input as DefineRegistersInput);
    let arch = &parsed.arch;
    let reg_class_enum = Ident::new(&format!("{}RegisterClass", arch), Span::call_site());

    let variants: Vec<_> = parsed
        .banks
        .iter()
        .map(|b| {
            let variant = to_pascal_case_ident(&b.name);
            quote! { #variant }
        })
        .collect();

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum #reg_class_enum {
            #(#variants),*
        }
    }
    .into()
}

struct DefineRegistersInput {
    arch: Ident,
    banks: Vec<RegisterBankDef>,
}

struct RegisterBankDef {
    name: Ident,
}

impl Parse for DefineRegistersInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // arch = <ident>;
        let _arch_kw: Ident = input.parse()?;
        let _eq: Token![=] = input.parse()?;
        let arch: Ident = input.parse()?;
        let _semi: Token![;] = input.parse()?;

        // Parse bank definitions
        let mut banks = Vec::new();
        while !input.is_empty() {
            let _bank_kw: Ident = input.parse()?;
            let name: Ident = input.parse()?;
            let content;
            braced!(content in input);
            // Consume bank body (count, prefix, aliases) but don't use yet
            while !content.is_empty() {
                let _key: Ident = content.parse()?;
                let _eq: Token![=] = content.parse()?;
                if content.peek(syn::token::Brace) {
                    let _nested;
                    braced!(_nested in content);
                } else {
                    let _val: syn::Expr = content.parse()?;
                }
                if !content.is_empty() {
                    let _semi: Token![;] = content.parse()?;
                }
            }
            banks.push(RegisterBankDef { name });
        }

        Ok(DefineRegistersInput { arch, banks })
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

    // Generate field enum variants from the fields block
    let field_variants: Vec<_> = parsed
        .fields
        .iter()
        .map(|f| {
            let variant = to_pascal_case_ident(f);
            quote! { #variant }
        })
        .collect();

    // Validate that all fields used in formats are declared in the fields block
    let declared_fields: std::collections::HashSet<String> =
        parsed.fields.iter().map(|f| f.to_string()).collect();
    for format in &parsed.formats {
        for field in &format.fields {
            if !declared_fields.contains(&field.name.to_string()) {
                let msg = format!(
                    "field '{}' used in format '{}' but not declared in fields block",
                    field.name, format.name
                );
                return syn::Error::new(field.name.span(), msg)
                    .to_compile_error()
                    .into();
            }
        }
    }

    let format_statics: Vec<_> = parsed.formats.iter().map(|f| {
        let name = &f.name;
        let field_specs: Vec<_> = f.fields.iter().map(|field| {
            let fname = &field.name;
            let start = &field.start;
            let length = &field.length;
            let field_type = to_pascal_case_ident(fname);
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
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum #field_enum {
            #(#field_variants),*
        }

        #(#format_statics)*
    }
    .into()
}

struct DefineFormatsInput {
    arch: Ident,
    fields: Vec<Ident>,
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

fn to_pascal_case_ident(ident: &Ident) -> Ident {
    let s = ident.to_string();
    let mut chars = s.chars();
    let pascal = match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
        None => String::new(),
    };
    Ident::new(&pascal, ident.span())
}

impl Parse for DefineFormatsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // arch = <ident>;
        let _arch_kw: Ident = input.parse()?;
        let _eq: Token![=] = input.parse()?;
        let arch: Ident = input.parse()?;
        let _semi: Token![;] = input.parse()?;

        // fields { rd; rj; rk; si12; }
        let _fields_kw: Ident = input.parse()?;
        let fields_content;
        braced!(fields_content in input);
        let mut fields = Vec::new();
        while !fields_content.is_empty() {
            let field_name: Ident = fields_content.parse()?;
            fields.push(field_name);
            if !fields_content.is_empty() {
                let _semi: Token![;] = fields_content.parse()?;
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

            let mut format_fields = Vec::new();
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
                format_fields.push(FormatFieldDef {
                    name: field_name,
                    _colon,
                    _bits_fn: bits_fn,
                    start,
                    _comma,
                    length,
                });
            }

            formats.push(FormatDef {
                name,
                fields: format_fields,
            });

            if !input.is_empty() {
                let _semi: Token![;] = input.parse()?;
            }
        }

        Ok(DefineFormatsInput {
            arch,
            fields,
            formats,
        })
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
fn is_exact_word_mask(expr: &Expr) -> bool {
    if let Expr::Macro(mac) = expr {
        let last = mac.mac.path.segments.last().map(|s| s.ident.to_string());
        if last.as_deref() == Some("mask_value") {
            let tokens = quote::quote!(#mac.mac.tokens)
                .to_string()
                .replace([' ', '\n'], "");
            if tokens.starts_with("(0xFFFF_FFFF,") || tokens.starts_with("(0xFFFFFFFFFFFFFFFF,") {
                return true;
            }
        }
    }
    false
}

fn is_non_empty_operands(expr: &Expr) -> bool {
    let tokens = quote::quote!(#expr).to_string().replace([' ', '\n'], "");
    tokens != "&[]"
}

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
                effect: None,
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

    // Compile-time validation: exact-word pattern with variable operands
    let exact_word_checks: Vec<_> = parsed
        .instructions
        .iter()
        .filter_map(|i| {
            if is_exact_word_mask(&i.pattern) && is_non_empty_operands(&i.operands) {
                let msg = format!(
                    "instruction '{}' has exact-word pattern but non-empty operands",
                    i.mnemonic.value()
                );
                Some(quote! {
                    const _: () = ::core::compile_error!(#msg);
                })
            } else {
                None
            }
        })
        .collect();

    // Compile-time validation: missing manual_ref
    let missing_manual_checks: Vec<_> = parsed
        .instructions
        .iter()
        .filter_map(|i| {
            if i.manual.is_none() {
                let msg = format!("instruction '{}' is missing manual_ref", i.mnemonic.value());
                Some(quote! {
                    const _: () = ::core::compile_error!(#msg);
                })
            } else {
                None
            }
        })
        .collect();

    // Generate specs array
    let spec_names: Vec<_> = parsed.instructions.iter().map(|i| &i.name).collect();

    quote! {
        #(#duplicate_checks)*

        #(#exact_word_checks)*

        #(#missing_manual_checks)*

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

/// Define instruction aliases applied post-decode.
///
/// Example:
/// ```ignore
/// define_aliases! {
///     arch = LoongArch;
///     alias "nop" for "ANDI" {
///         when [operand(0) == reg(0), operand(1) == reg(0), operand(2) == imm(0)];
///         mnemonic = "nop";
///         visible_operands = [];
///     }
/// }
/// ```
#[proc_macro]
pub fn define_aliases(input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as DefineAliasesInput);
    let aliases = parsed.aliases;

    let alias_branches = aliases.iter().map(|alias| {
        let opcode_id = &alias.opcode_id;
        let mnemonic = &alias.mnemonic;
        let visible = &alias.visible_operands;

        let cond_checks = alias.conditions.iter().map(|cond| {
            let idx = cond.operand_index;
            let idx_lit = syn::Index::from(idx);
            match &cond.kind {
                AliasConditionKind::Reg(expected) => {
                    let exp = *expected;
                    quote! {
                        {
                            let __v = match insn.operands.get(#idx_lit) {
                                Some(::robustone_core::ir::Operand::Register { register }) => register.id == #exp,
                                _ => false,
                            };
                            __v
                        }
                    }
                }
                AliasConditionKind::Imm(expected) => {
                    let exp = *expected;
                    quote! {
                        {
                            let __v = match insn.operands.get(#idx_lit) {
                                Some(::robustone_core::ir::Operand::Immediate { value }) => *value == #exp,
                                _ => false,
                            };
                            __v
                        }
                    }
                }
            }
        });

        let visible_arr = quote! { &[#(#visible),*] };

        quote! {
            if insn.opcode_id.as_deref() == Some(#opcode_id) {
                if true #(&& #cond_checks)* {
                    insn.render_hints.compat_mnemonic = Some(#mnemonic.into());
                    let mut hidden = Vec::new();
                    for i in 0..insn.operands.len() {
                        if !#visible_arr.contains(&i) {
                            hidden.push(i);
                        }
                    }
                    insn.render_hints.compat_hidden_operands = hidden;
                }
            }
        }
    });

    quote! {
        /// Apply architecture-specific aliases to a decoded instruction.
        pub fn apply_aliases(insn: &mut ::robustone_core::ir::DecodedInstruction) {
            #(#alias_branches)*
        }
    }
    .into()
}

struct DefineAliasesInput {
    _arch: Ident,
    aliases: Vec<AliasDef>,
}

struct AliasDef {
    opcode_id: String,
    mnemonic: String,
    conditions: Vec<AliasCondition>,
    visible_operands: Vec<usize>,
}

struct AliasCondition {
    operand_index: usize,
    kind: AliasConditionKind,
}

enum AliasConditionKind {
    Reg(u32),
    Imm(i64),
}

impl Parse for DefineAliasesInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let arch_kw: Ident = input.parse()?;
        if arch_kw != "arch" {
            return Err(syn::Error::new(arch_kw.span(), "expected `arch`"));
        }
        let _: Token![=] = input.parse()?;
        let arch_name: Ident = input.parse()?;
        let _: Token![;] = input.parse()?;

        let mut aliases = Vec::new();
        while !input.is_empty() {
            let alias_kw: Ident = input.parse()?;
            if alias_kw != "alias" {
                return Err(syn::Error::new(alias_kw.span(), "expected `alias`"));
            }
            let alias_mnemonic: LitStr = input.parse()?;
            let _: Token![for] = input.parse()?;
            let opcode_id: LitStr = input.parse()?;

            let content;
            braced!(content in input);

            let mut conditions = Vec::new();
            let mut mnemonic = None;
            let mut visible_operands = Vec::new();

            while !content.is_empty() {
                let key: Ident = content.parse()?;
                if key == "when" {
                    let conds;
                    syn::bracketed!(conds in content);
                    while !conds.is_empty() {
                        let cond = parse_alias_condition(&conds)?;
                        conditions.push(cond);
                        if !conds.is_empty() {
                            let _: Token![,] = conds.parse()?;
                        }
                    }
                    let _: Token![;] = content.parse()?;
                } else if key == "mnemonic" {
                    let _: Token![=] = content.parse()?;
                    let val: LitStr = content.parse()?;
                    mnemonic = Some(val.value());
                    let _: Token![;] = content.parse()?;
                } else if key == "visible_operands" {
                    let _: Token![=] = content.parse()?;
                    let vals;
                    syn::bracketed!(vals in content);
                    while !vals.is_empty() {
                        let v: LitInt = vals.parse()?;
                        visible_operands.push(v.base10_parse()?);
                        if !vals.is_empty() {
                            let _: Token![,] = vals.parse()?;
                        }
                    }
                    let _: Token![;] = content.parse()?;
                } else {
                    return Err(syn::Error::new(key.span(), "unknown alias property"));
                }
            }

            aliases.push(AliasDef {
                opcode_id: opcode_id.value(),
                mnemonic: mnemonic.unwrap_or_else(|| alias_mnemonic.value()),
                conditions,
                visible_operands,
            });
        }

        Ok(DefineAliasesInput {
            _arch: arch_name,
            aliases,
        })
    }
}

fn parse_alias_condition(input: ParseStream) -> syn::Result<AliasCondition> {
    let operand_kw: Ident = input.parse()?;
    if operand_kw != "operand" {
        return Err(syn::Error::new(operand_kw.span(), "expected `operand`"));
    }
    let idx_paren;
    syn::parenthesized!(idx_paren in input);
    let idx: LitInt = idx_paren.parse()?;
    let operand_index: usize = idx.base10_parse()?;

    let _: Token![==] = input.parse()?;

    let kind_kw: Ident = input.parse()?;
    let kind = if kind_kw == "reg" {
        let val_paren;
        syn::parenthesized!(val_paren in input);
        let val: LitInt = val_paren.parse()?;
        AliasConditionKind::Reg(val.base10_parse()?)
    } else if kind_kw == "imm" {
        let val_paren;
        syn::parenthesized!(val_paren in input);
        // Support negative immediates
        let val: Expr = val_paren.parse()?;
        let lit_val = match val {
            Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(i),
                ..
            }) => i.base10_parse()?,
            Expr::Unary(syn::ExprUnary {
                op: syn::UnOp::Neg(_),
                expr,
                ..
            }) => {
                if let Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(i),
                    ..
                }) = *expr
                {
                    -i.base10_parse::<i64>()?
                } else {
                    return Err(syn::Error::new(kind_kw.span(), "expected integer literal"));
                }
            }
            _ => return Err(syn::Error::new(kind_kw.span(), "expected integer literal")),
        };
        AliasConditionKind::Imm(lit_val)
    } else {
        return Err(syn::Error::new(kind_kw.span(), "expected `reg` or `imm`"));
    };

    Ok(AliasCondition {
        operand_index,
        kind,
    })
}
