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

    fn write_binder_stub(&self, text: &mut dyn Write, resolver: &str) {
        let first_param = if self.itanium_abi {
            "rdi" // SystemV ABI
        } else {
            "rcx" // Windows ABI
        };

        write_lines!(text,
            "    push rbp"
            "    mov rbp, rsp"
            // Re-align stack to 16 bytes
            "    and rsp, ~0xF"

            // Save volatile registers
            //   for SystemV ABI: rax, rcx, rdx, rdi, rsi, r8, r9, r10, r11
            //   for Windows ABI: rax, rcx, rdx, r8, r9, r10, r11
            // For simplicity we'll save the union of them.
            "    push rax"
            "    push rcx"
            "    push rdx"
            "    push rdi"
            "    push rsi"
            "    push r8"
            "    push r9"
            "    push r10"
            "    push r11"

            // Windows ABI requires us to allocate 4 "home" slots for register parameters,
            // plus, we need one slot to keep stack 16-aligned after pushing 9 registers above.
            "    sub rsp, 8*5" 
            "    mov {first_param}, [rbp + 8]"
            "    call {resolver}"
            "    add rsp, 8*5"
            "    mov [rbp + 8], rax" // Replace symbol table index with the returned symbol address

            "    pop r11"
            "    pop r10"
            "    pop r9"
            "    pop r8"
            "    pop rsi"
            "    pop rdi"
            "    pop rdx"
            "    pop rcx"
            "    pop rax"

            "    mov rsp, rbp"
            "    pop rbp"
            "    ret",
            first_param=first_param,
            resolver=resolver
        );
    }
}
