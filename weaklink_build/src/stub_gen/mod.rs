use crate::util::iter_fmt;
use crate::SymbolStub;
use std::io::Write;

pub(crate) trait StubGenerator {
    fn generate(&self, text: &mut dyn Write, symbols: &[SymbolStub], symbol_table: &str, lazy_binding: bool) {
        write_lines!(text,
            "global_asm!{{\""
            ".data"
            ".p2align 2, 0x0"
            "{symbol_table}:"
            "_{symbol_table}:"
            "{entries}"
            "\"}}",
            symbol_table = symbol_table,
            entries = iter_fmt(symbols.iter().enumerate(), |f, (idx, sym)| {
                write!(f, "    {} ", self.data_ptr_directive());
                if sym.is_data || !lazy_binding { writeln!(f, "0") } else { writeln!(f, "resolve_{idx}") }
            }
        ));

        if lazy_binding {
            write_lines!(text,
                "global_asm!{{\""
                "binder:"
                "\"}}");
        }

        for (i, symbol) in symbols.iter().enumerate() {
            if !symbol.is_data {
                write_lines!(text,
                    "global_asm!{{\""
                    ".text"
                    ".p2align 2, 0x0"
                    ".global \\\"{symbol}\\\"" // Will be unescaped the 2nd time when compiling the generated module.
                    "\\\"{symbol}\\\":",
                    symbol = symbol.export_name
                );
                self.write_fn_stub(text, symbol_table, i);

                if lazy_binding {
                    writeln!(text, "resolve_{index}:", index = i);
                    self.write_jmp_binder(text, i, "binder");
                }

                writeln!(text, "\"}}");
            } else {
                write_lines!(text,
                    "#[no_mangle]"
                    "pub extern \"C\" fn {symbol}() -> Address {{"
                    "    unsafe {{ {symbol_table}[{index}] as Address }}"
                    "}}",
                    symbol = symbol.export_name,
                    symbol_table = symbol_table,
                    index = i
                );
            }
        }
    }

    fn write_fn_stub(&self, text: &mut dyn Write, symtab_base: &str, index: usize);

    fn write_jmp_binder(&self, text: &mut dyn Write, index: usize, binder: &str);

    fn data_ptr_directive(&self) -> &str {
        ".quad"
    }
}

pub(crate) mod aarch64;
pub(crate) mod arm;
pub(crate) mod x64;
