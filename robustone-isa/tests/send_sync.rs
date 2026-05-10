//! Verify that `Decoder<B>` and `DecodedInstruction` are `Send + Sync`.

use robustone_core::ir::{ArchitectureId, RegisterId};
use robustone_core::types::error::DisasmError;
use robustone_isa::{
    ArchitectureBackend, DecodeProfile, FeatureSet, FormatSpec, InstructionRead, InstructionSpec,
    RenderPolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DummyMode {
    Base,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DummyFeature: u8 {
        const BASE = 1 << 0;
    }
}

impl FeatureSet for DummyFeature {
    fn empty() -> Self {
        Self::empty()
    }
    fn all_supported_for_tests() -> Self {
        Self::BASE
    }
    fn contains(self, required: Self) -> bool {
        self.bits() & required.bits() == required.bits()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DummyField {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DummyRegClass {}

pub struct DummyBackend;

impl ArchitectureBackend for DummyBackend {
    type Word = u32;
    type Mode = DummyMode;
    type Feature = DummyFeature;
    type Field = DummyField;
    type RegisterClass = DummyRegClass;

    fn architecture_id() -> ArchitectureId {
        ArchitectureId::Riscv
    }

    fn read_instruction(_bytes: &[u8]) -> Result<InstructionRead<Self::Word>, DisasmError> {
        unreachable!()
    }

    fn lookup(
        _word: Self::Word,
        _profile: &DecodeProfile<Self>,
    ) -> Option<&'static InstructionSpec<Self>> {
        unreachable!()
    }

    fn lower_register(
        _class: Self::RegisterClass,
        _raw: u32,
        _profile: &DecodeProfile<Self>,
    ) -> RegisterId {
        unreachable!()
    }

    fn render_policy(_profile: &DecodeProfile<Self>) -> RenderPolicy<Self> {
        unreachable!()
    }

    fn extract_field(
        _word: Self::Word,
        _format: &FormatSpec<Self::Field>,
        _field: Self::Field,
    ) -> Result<u32, DisasmError> {
        unreachable!()
    }
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn decoder_is_send_sync() {
    assert_send_sync::<robustone_isa::Decoder<DummyBackend>>();
}

#[test]
fn decoded_instruction_is_send_sync() {
    assert_send_sync::<robustone_core::ir::DecodedInstruction>();
}
