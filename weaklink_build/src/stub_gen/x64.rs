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

    fn write_jmp_binder(&self, text: &mut dyn Write, index: usize, binder: &str) {
        write_lines!(text,
            "    push {index}"
            "    jmp {binder}",
            index=index,
            binder=binder
        );
    }

    fn write_binder_stub(&self, text: &mut dyn Write, resolver: &str) {
        let first_param = if self.target_os == TargetOs::Windows {
            "rcx" // Windows ABI
        } else {
            "rdi" // SystemV ABI
        };

        write_lines!(text,
            "   .cfi_startproc"
            "   .cfi_def_cfa rsp, 16"
            "    push rbx"
            "    .cfi_adjust_cfa_offset 8"
            "    mov rbx, rsp"
            "   .cfi_def_cfa_register rbx"
            // We don't know whether the stub was call'ed or jmp'ed to,
            // better make sure stack is 16-aligned.
            "    and rsp, ~0xF"

            // Save volatile registers:
            // - for SystemV ABI: rax, rcx, rdx, rdi, rsi, r8, r9, r10, r11
            // - for Windows ABI: rax, rcx, rdx, r8, r9, r10, r11
            // For simplicity, we are saving a superset of them.
            "    push rax"
            "    push rcx"
            "    push rdx"
            "    push rdi"
            "    push rsi"
            "    push r8"
            "    push r9"
            "    push r10"
            "    push r11"

            // Windows ABI requires us to allocate 4 "home slots" for register parameters,
            // we also need one extra slot to keep stack 16-aligned after having pushed 9 registers above.
            "    sub rsp, 8*5"
            "    mov {first_param}, [rbx + 8]"
            "    call {pfx}{resolver}"
            "    add rsp, 8*5"
            "    mov [rbx + 8], rax" // Replace symbol table index with the returned symbol address

            "    pop r11"
            "    pop r10"
            "    pop r9"
            "    pop r8"
            "    pop rsi"
            "    pop rdi"
            "    pop rdx"
            "    pop rcx"
            "    pop rax"

            "    mov rsp, rbx"
            "    pop rbx"
            "    ret"
            "    .cfi_endproc"
            ,
            pfx=self.asm_symbol_prefix(),
            first_param=first_param,
            resolver=resolver
        );
    }

    fn asm_symbol_prefix(&self) -> &str {
        if self.target_os == TargetOs::MacOS {
            "_"
        } else {
            ""
        }
    }
}
