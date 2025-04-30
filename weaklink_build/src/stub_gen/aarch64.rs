use crate::SymbolStub;
use std::io::{Read, Write};
use super::TargetOs;

pub struct Aarch64StubGenerator {
    pub(crate) target_os: TargetOs
}

impl super::StubGenerator for Aarch64StubGenerator {
    fn write_fn_stub(&self, text: &mut dyn Write, symtab_base: &str, index: usize) {
        if self.target_os == TargetOs::MacOS {
            write_lines!(text,
                "    adrp x16, {pfx}{symtab_base} + {offset} @PAGE"
                "    ldr x16, [x16, {pfx}{symtab_base} + {offset} @PAGEOFF]"
                "    br x16",
                pfx=self.asm_symbol_prefix(),
                symtab_base = symtab_base,
                offset = index * 8
            );
        } else {
            write_lines!(text,
                "    adrp x16, {symtab_base} + {offset}"
                "    ldr x16, [x16, :lo12:{pfx}{symtab_base} + {offset}]"
                "    br x16",
                pfx=self.asm_symbol_prefix(),
                symtab_base = symtab_base,
                offset = index * 8
            );
        }
    }

    fn asm_symbol_prefix(&self) -> &str {
        if self.target_os == TargetOs::MacOS  {
            "_"
        } else {
            ""
        }
    }
}
