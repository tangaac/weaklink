use crate::SymbolStub;
use std::io::{Read, Write};

pub struct X64StubGenerator {
    itanium_abi: bool, // else Windows ABI
}

impl X64StubGenerator {
    pub fn new_itanium() -> Self {
        X64StubGenerator { itanium_abi: true }
    }

    pub fn new_windows() -> Self {
        X64StubGenerator { itanium_abi: false }
    }
}

impl super::StubGenerator for X64StubGenerator {
    fn write_fn_stub(&self, text: &mut dyn Write, symtab_base: &str, index: usize) {
        if self.itanium_abi {
            write_lines!(text,
                "    mov r11, [rip + {symtab_base}@GOTPCREL]"
                "    jmp [r11 + {offset}]",
                symtab_base = symtab_base,
                offset = index * 8
            );
        } else {
            write_lines!(
                text,
                "   jmp qword ptr [rip + {symtab_base} + {offset}]",
                symtab_base = symtab_base,
                offset = index * 8
            );
        }
    }

    fn write_jmp_binder(&self, text: &mut dyn Write, index: usize, binder: &str) {
        write_lines!(text,
            "    push {index}"
            "    jmp {binder}",
            index=index,
            binder=binder
        );
    }
}
