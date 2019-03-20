use crate::SymbolStub;
use std::io::{Read, Write};

pub struct Aarch64StubGenerator {
    is_macos: bool,
}

impl Aarch64StubGenerator {
    pub fn new_linux() -> Self {
        Aarch64StubGenerator { is_macos: false }
    }
    pub fn new_macos() -> Self {
        Aarch64StubGenerator { is_macos: true }
    }
}

impl super::StubGenerator for Aarch64StubGenerator {
    fn write_fn_stub(&self, text: &mut dyn Write, symtab_base: &str, index: usize) {
        if self.is_macos {
            write_lines!(text,
                "    adrp x16, {symtab_base} + {offset} @PAGE"
                "    ldr x16, [x16, {symtab_base} + {offset} @PAGEOFF]"
                "    br x16",
                symtab_base = symtab_base,
                offset = index * 8
            );
        } else {
            write_lines!(text,
                "    adrp x16, {symtab_base} + {offset}"
                "    ldr x16, [x16, :lo12:{symtab_base} + {offset}]"
                "    br x16",
                symtab_base = symtab_base,
                offset = index * 8
            );
        }
    }

    fn write_jmp_binder(&self, text: &mut dyn Write, index: usize, binder: &str) {
        write_lines!(text,
            "    ldr x16, ={index}"
            "    b {binder}",
            index=index,
            binder=binder
        );
    }
}
