use crate::util::iter_fmt;
use crate::SymbolStub;
use std::io::Write;

#[derive(PartialEq, Eq)]
pub(crate) enum TargetOs {
    Linux,
    MacOS,
    Windows,
}

pub(crate) trait StubGenerator {
    fn generate(&self, text: &mut dyn Write, symbols: &[SymbolStub], symbol_table: &str) {
        write_lines!(text,
            "global_asm!{{\""
            ".data"
            ".p2align 2, 0x0"
            "{pfx}{symbol_table}:"
            "{entries}"
            "\"}}",
            pfx = self.asm_symbol_prefix(),
            symbol_table = symbol_table,
            entries = iter_fmt(symbols.iter().enumerate(), |f, (idx, sym)| {
                let dir = self.data_ptr_directive();
                writeln!(f, "    {dir} 0")
            }
        ));

        for (i, symbol) in symbols.iter().enumerate() {
            if !symbol.is_data {
                write_lines!(text,
                    "global_asm!{{\""
                    ".text"
                    ".p2align 2, 0x0"
                    ".global \\\"{symbol}\\\"" // Will be unescaped the 2nd time when compiling the generated module.
                    //".type   \\\"{symbol}\\\", function"
                    "\\\"{symbol}\\\":",
                    symbol = symbol.export_name
                );
                self.write_fn_stub(text, symbol_table, i);
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

    /// Emit code that loads index'th entry from the symbol table and jumps to that address.
    fn write_fn_stub(&self, text: &mut dyn Write, symtab_base: &str, index: usize);

    /// Declaration directive for pointer-sized data.
    fn data_ptr_directive(&self) -> &str {
        ".quad"
    }

    /// A prefix, if any, that needs to be prepended to Rust symbols in order to reference them in assembly code.
    fn asm_symbol_prefix(&self) -> &str {
        ""
    }
}

pub(crate) mod aarch64;
pub(crate) mod arm;
pub(crate) mod x64;
pub(crate) mod loongarch;
