//! RISC-V instruction aliases.
//!
//! This module contains reference-compatible alias logic that is shared
//! between the legacy decoder and the shared-ISA backend (AC-9 migration).
//!
//! Aliases here set `render_hints.compat_mnemonic` / `compat_hidden_operands`
//! for assembly-level synonyms (e.g. `addi x1, x0, 5` → `li`).
//! Pseudo-instructions that mutate operands (e.g. `ori` → `prefetch.i`)
//! are also handled because they are required for downstream compatibility.

use robustone_core::ir::{DecodedInstruction, Operand};

/// Apply RISC-V aliases to a decoded instruction.
///
/// This should be called after the base mnemonic and operands have been
/// populated.  It mutates `render_hints` (and in the `ori` case the full
/// instruction) in place.
pub fn apply_riscv_aliases(decoded: &mut DecodedInstruction) {
    match decoded.mnemonic.as_str() {
        "addi" => {
            if let [_, Operand::Register { register: rs1 }, _] = decoded.operands.as_slice()
                && rs1.id == 0
                && decoded
                    .registers_written
                    .first()
                    .map(|r| r.id != 0)
                    .unwrap_or(false)
            {
                decoded.render_hints.compat_mnemonic = Some("li".to_string());
                decoded.render_hints.compat_hidden_operands = vec![1];
            }
        }
        "ori" => {
            if let Some(rd) = decoded.registers_written.first()
                && rd.id == 0
            {
                let (rs1, imm) = if let [
                    _,
                    Operand::Register { register: rs1 },
                    Operand::Immediate { value: imm },
                ] = decoded.operands.as_slice()
                {
                    (Some(*rs1), *imm)
                } else {
                    (None, 0)
                };
                if let Some(rs1) = rs1 {
                    let prefetch = match imm {
                        0 => Some("prefetch.i"),
                        1 => Some("prefetch.r"),
                        2 => Some("prefetch.t"),
                        3 => Some("prefetch.w"),
                        _ => None,
                    };
                    if let Some(pf) = prefetch {
                        decoded.mnemonic = pf.to_string();
                        decoded.operands = vec![Operand::Memory {
                            base: Some(rs1),
                            displacement: 0,
                        }];
                        decoded.registers_read = vec![rs1];
                        decoded.registers_written.clear();
                        decoded.groups = vec!["load".to_string()];
                        decoded.render_hints = Default::default();
                    }
                }
            }
        }
        "jal" => {
            if let Some(rd) = decoded.registers_written.first() {
                if rd.id == 0 {
                    decoded.render_hints.compat_mnemonic = Some("j".to_string());
                    decoded.render_hints.compat_hidden_operands = vec![0];
                } else if rd.id == 1 {
                    decoded.render_hints.compat_hidden_operands = vec![0];
                }
            }
        }
        "jalr" => {
            if let Some(rd) = decoded.registers_written.first()
                && rd.id == 1
            {
                decoded.render_hints.compat_hidden_operands = vec![0];
            }
        }
        "beq" => {
            if let [_, Operand::Register { register: rs2 }, _] = decoded.operands.as_slice()
                && rs2.id == 0
            {
                decoded.render_hints.compat_mnemonic = Some("beqz".to_string());
                decoded.render_hints.compat_hidden_operands = vec![1];
            }
        }
        "bne" => {
            if let [_, Operand::Register { register: rs2 }, _] = decoded.operands.as_slice()
                && rs2.id == 0
            {
                decoded.render_hints.compat_mnemonic = Some("bnez".to_string());
                decoded.render_hints.compat_hidden_operands = vec![1];
            }
        }
        "csrrs" | "csrrw" | "csrrc" => {
            if let [
                _,
                Operand::Immediate { value: csr },
                Operand::Register { register: rs1 },
            ] = decoded.operands.as_slice()
            {
                let mnemonic = decoded.mnemonic.as_str();
                let rd_zero = decoded
                    .registers_written
                    .first()
                    .map(|r| r.id == 0)
                    .unwrap_or(false);
                let (alias, hidden) = match (
                    mnemonic,
                    decoded.registers_written.first(),
                    rs1.id,
                    *csr as u16,
                ) {
                    ("csrrs", _, 0, 0xC00) => (Some("rdcycle"), vec![1, 2]),
                    ("csrrs", _, 0, 0xC01) => (Some("rdtime"), vec![1, 2]),
                    ("csrrs", _, 0, 0xC02) => (Some("rdinstret"), vec![1, 2]),
                    ("csrrs", _, 0, 0xC80) => (Some("rdcycleh"), vec![1, 2]),
                    ("csrrs", _, 0, 0xC81) => (Some("rdtimeh"), vec![1, 2]),
                    ("csrrs", _, 0, 0xC82) => (Some("rdinstreth"), vec![1, 2]),
                    ("csrrs", Some(_), 0, _) => (Some("csrr"), vec![2]),
                    ("csrrw", _, _, _) if rd_zero => (Some("csrw"), vec![0]),
                    ("csrrs", _, _, _) if rd_zero => (Some("csrs"), vec![0]),
                    ("csrrc", _, _, _) if rd_zero => (Some("csrc"), vec![0]),
                    _ => (None, Vec::new()),
                };
                if let Some(a) = alias {
                    decoded.render_hints.compat_mnemonic = Some(a.to_string());
                    decoded.render_hints.compat_hidden_operands = hidden;
                }
            }
        }
        "csrrwi" | "csrrsi" | "csrrci" => {
            let rd_zero = decoded
                .registers_written
                .first()
                .map(|r| r.id == 0)
                .unwrap_or(false);
            if rd_zero {
                let alias = match decoded.mnemonic.as_str() {
                    "csrrwi" => Some("csrwi"),
                    "csrrsi" => Some("csrsi"),
                    "csrrci" => Some("csrci"),
                    _ => None,
                };
                if let Some(a) = alias {
                    decoded.render_hints.compat_mnemonic = Some(a.to_string());
                    decoded.render_hints.compat_hidden_operands = vec![0];
                }
            }
        }
        "c.addiw" => {
            decoded.render_hints.compat_mnemonic = Some("addiw".to_string());
        }
        "c.lui" => {
            decoded.render_hints.compat_mnemonic = Some("lui".to_string());
        }
        "c.jr" => {
            decoded.render_hints.compat_mnemonic = Some("jr".to_string());
        }
        "c.jal" => {
            decoded.render_hints.compat_mnemonic = Some("jal".to_string());
        }
        "c.subw" => {
            decoded.render_hints.compat_mnemonic = Some("subw".to_string());
        }
        "c.addw" => {
            decoded.render_hints.compat_mnemonic = Some("addw".to_string());
        }
        "fadd.s" | "fsub.s" | "fmul.s" | "fdiv.s" => {
            if let Some(Operand::Text { value }) = decoded.operands.last_mut() {
                *value = match value.as_str() {
                    "0" => "rne".to_string(),
                    "1" => "rtz".to_string(),
                    "2" => "rdn".to_string(),
                    "3" => "rup".to_string(),
                    "4" => "rmm".to_string(),
                    "7" => "dyn".to_string(),
                    _ => value.clone(),
                };
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use robustone_core::ir::{ArchitectureId, RegisterId};

    fn rv_reg(id: u32) -> RegisterId {
        RegisterId::riscv(id)
    }

    fn make_decoded(mnemonic: &str, operands: Vec<Operand>) -> DecodedInstruction {
        let mut registers_read: Vec<RegisterId> = Vec::new();
        let mut registers_written: Vec<RegisterId> = Vec::new();
        for op in &operands {
            match op {
                Operand::Register { register } => {
                    // heuristic: first register is usually rd (write), rest are rs (read)
                    // tests will override registers_written/registers_read explicitly
                }
                _ => {}
            }
        }
        DecodedInstruction {
            architecture: ArchitectureId::Riscv,
            address: 0,
            mode: String::new(),
            mnemonic: mnemonic.to_string(),
            opcode_id: Some(mnemonic.to_string()),
            size: 4,
            raw_bytes: vec![0; 4],
            operands,
            registers_read: Vec::new(),
            registers_written: Vec::new(),
            implicit_registers_read: Vec::new(),
            implicit_registers_written: Vec::new(),
            groups: Vec::new(),
            status: robustone_core::ir::DecodeStatus::Success,
            render_hints: Default::default(),
        }
    }

    #[test]
    fn alias_addi_to_li() {
        let mut d = make_decoded(
            "addi",
            vec![
                Operand::Register {
                    register: rv_reg(5),
                },
                Operand::Register {
                    register: rv_reg(0),
                },
                Operand::Immediate { value: 42 },
            ],
        );
        d.registers_written = vec![rv_reg(5)];
        apply_riscv_aliases(&mut d);
        assert_eq!(d.render_hints.compat_mnemonic, Some("li".to_string()));
        assert_eq!(d.render_hints.compat_hidden_operands, vec![1]);
    }

    #[test]
    fn alias_jal_to_j() {
        let mut d = make_decoded(
            "jal",
            vec![
                Operand::Register {
                    register: rv_reg(0),
                },
                Operand::Immediate { value: 0x1000 },
            ],
        );
        d.registers_written = vec![rv_reg(0)];
        apply_riscv_aliases(&mut d);
        assert_eq!(d.render_hints.compat_mnemonic, Some("j".to_string()));
        assert_eq!(d.render_hints.compat_hidden_operands, vec![0]);
    }

    #[test]
    fn alias_beq_to_beqz() {
        let mut d = make_decoded(
            "beq",
            vec![
                Operand::Register {
                    register: rv_reg(3),
                },
                Operand::Register {
                    register: rv_reg(0),
                },
                Operand::Immediate { value: 8 },
            ],
        );
        d.registers_written = vec![];
        apply_riscv_aliases(&mut d);
        assert_eq!(d.render_hints.compat_mnemonic, Some("beqz".to_string()));
        assert_eq!(d.render_hints.compat_hidden_operands, vec![1]);
    }

    #[test]
    fn alias_bne_to_bnez() {
        let mut d = make_decoded(
            "bne",
            vec![
                Operand::Register {
                    register: rv_reg(3),
                },
                Operand::Register {
                    register: rv_reg(0),
                },
                Operand::Immediate { value: 8 },
            ],
        );
        d.registers_written = vec![];
        apply_riscv_aliases(&mut d);
        assert_eq!(d.render_hints.compat_mnemonic, Some("bnez".to_string()));
        assert_eq!(d.render_hints.compat_hidden_operands, vec![1]);
    }

    #[test]
    fn alias_ori_to_prefetch_i() {
        let mut d = make_decoded(
            "ori",
            vec![
                Operand::Register {
                    register: rv_reg(0),
                },
                Operand::Register {
                    register: rv_reg(2),
                },
                Operand::Immediate { value: 0 },
            ],
        );
        d.registers_written = vec![rv_reg(0)];
        apply_riscv_aliases(&mut d);
        assert_eq!(d.mnemonic, "prefetch.i");
        assert_eq!(d.operands.len(), 1);
    }

    #[test]
    fn alias_csrrs_to_rdcycle() {
        let mut d = make_decoded(
            "csrrs",
            vec![
                Operand::Register {
                    register: rv_reg(1),
                },
                Operand::Immediate { value: 0xC00 },
                Operand::Register {
                    register: rv_reg(0),
                },
            ],
        );
        d.registers_written = vec![rv_reg(1)];
        apply_riscv_aliases(&mut d);
        assert_eq!(d.render_hints.compat_mnemonic, Some("rdcycle".to_string()));
        assert_eq!(d.render_hints.compat_hidden_operands, vec![1, 2]);
    }
}
