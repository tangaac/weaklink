use crate::SymbolStub;
use std::io::{Read, Write};

pub struct ArmStubGenerator {}

impl ArmStubGenerator {
    pub fn new() -> Self {
        ArmStubGenerator {}
    }
}

impl super::StubGenerator for ArmStubGenerator {
    fn write_fn_stub(&self, text: &mut dyn Write, symtab_base: &str, index: usize) {
        write_lines!(text,
            "    ldr r12, ={symtab_base} - 1f + {offset}"
            "    add r12, pc, r12"
            "    ldr r12, [r12]"
            "1:"
            "    bx r12"
            "    .ltorg",
            symtab_base = symtab_base,
            offset = index * 4
        );
    }

    fn write_jmp_binder(&self, text: &mut dyn Write, index: usize, binder: &str) {
        write_lines!(text,
            "    ldr r12, ={index}"
            "    b {binder}",
            index=index,
            binder=binder
        );
    }

    fn data_ptr_directive(&self) -> &str {
        ".long"
    }
}
