use crate::SymbolStub;
use std::io::{Read, Write};
use super::TargetOs;

pub struct LoongArchStubGenerator {}

impl super::StubGenerator for LoongArchStubGenerator {
    fn write_fn_stub(&self, text: &mut dyn Write, symtab_base: &str, index: usize) {
        write_lines!(text,
            "    pcalau12i $r12, %pc_hi20({symtab_base} + {offset})"
            "    ld.d $r12, $r12, %pc_lo12({pfx}{symtab_base} + {offset})"
            "    jr $r12",
            pfx=self.asm_symbol_prefix(),
            symtab_base = symtab_base,
            offset = index * 8
        );
    }

    fn asm_symbol_prefix(&self) -> &str {
            ""
    }
}
