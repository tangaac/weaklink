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

    fn write_jmp_binder(&self, text: &mut dyn Write, index: usize, binder: &str) {
        write_lines!(text,
            "    ldr x16, ={index}"
            "    b {binder}",
            index=index,
            binder=binder
        );
    }

    fn write_binder_stub(&self, text: &mut dyn Write, resolver: &str) {
        write_lines!(text,
            "    .cfi_startproc"
            "    stp x0, x1, [sp, #-16]!"
            "    stp x2, x3, [sp, #-16]!"
            "    stp x4, x5, [sp, #-16]!"
            "    stp x6, x7, [sp, #-16]!"
            "    stp x8, x9, [sp, #-16]!"
            "    stp x10, x11, [sp, #-16]!"
            "    stp x12, x13, [sp, #-16]!"
            "    stp x14, x15, [sp, #-16]!"
            "    stp x17, lr, [sp, #-16]!"

            "    mov x0, x16"
            "    bl {pfx}{resolver}"
            "    mov x16, x0"

            "    ldp x17, lr, [sp], #16"
            "    ldp x14, x15, [sp], #16"
            "    ldp x12, x13, [sp], #16"
            "    ldp x10, x11, [sp], #16"
            "    ldp x8, x9, [sp], #16"
            "    ldp x6, x7, [sp], #16"
            "    ldp x4, x5, [sp], #16"
            "    ldp x2, x3, [sp], #16"
            "    ldp x0, x1, [sp], #16"
            "    br x16"
            "    .cfi_endproc",
            pfx=self.asm_symbol_prefix(),
            resolver=resolver
        );
    }

    fn asm_symbol_prefix(&self) -> &str {
        if self.target_os == TargetOs::MacOS  {
            "_"
        } else {
            ""
        }
    }
}
