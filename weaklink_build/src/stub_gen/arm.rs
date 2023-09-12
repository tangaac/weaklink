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

    fn write_binder_stub(&self, text: &mut dyn Write, resolver: &str) {
        write_lines!(text,
            "    push {{{{ r0, r1, r2, r3, lr }}}}"
            "    mov r7, sp"
            // Re-align stack to 8 bytes
            "    bic sp, sp, #7"
            "    mov r0, r12"
            "    bl {resolver}"
            "    mov r12, r0"
            "    mov sp, r7"
            "    pop {{{{ r0, r1, r2, r3, lr }}}}"
            "    bx r12",
            resolver=resolver
        );
    }

    fn data_ptr_directive(&self) -> &str {
        ".long"
    }
}
