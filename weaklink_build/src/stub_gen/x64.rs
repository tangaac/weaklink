use super::TargetOs;
use crate::SymbolStub;
use std::io::{Read, Write};

pub struct X64StubGenerator {
    pub(crate) target_os: TargetOs,
}

impl super::StubGenerator for X64StubGenerator {
    fn write_fn_stub(&self, text: &mut dyn Write, symtab_base: &str, index: usize) {
        if self.target_os == TargetOs::Windows {
            write_lines!(
                text,
                "   jmp qword ptr [rip + {symtab_base} + {offset}]",
                symtab_base = symtab_base,
                offset = index * 8
            );
        } else {
            write_lines!(text,
                "    mov r11, [rip + {pfx}{symtab_base}@GOTPCREL]"
                "    jmp [r11 + {offset}]",
                pfx=self.asm_symbol_prefix(),
                symtab_base = symtab_base,
                offset = index * 8
            );
        }
    }

    fn asm_symbol_prefix(&self) -> &str {
        if self.target_os == TargetOs::MacOS {
            "_"
        } else {
            ""
        }
    }
}
