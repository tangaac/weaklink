
This crate implements weak dynamic linking across platforms (Linux, macOS, and Windows), making it easier to work with
dynamic libraries that may not be installed or may vary in version.

# When is this useful?

This crate is useful when your program depends on a dynamic library that may not be installed on the target system, or
when different versions of the library are in use. Instead of manually managing each API call with `dlopen`/`dlsym` (or
platform-specific equivalents), Weaklink automatically handles loading and symbol resolution at runtime, simplifying the
process.

This is especially for calling into plugins that export mangled symbols (like C++ or Rust), since finding out the
mangled symbol name for a function may be non-trivial.

# How does it work?

Weaklink generates a Rust crate with function stubs that mirror the public APIs of the original dynamic library. These
stubs are compiled into a static library and linked to your main program. When a stubbed API is called, Weaklink
dynamically loads the original library, resolves the symbol, and jumps to the appropriate function.

Conceptually, this is similar to the ELF
[Procedure Linkage Table](https://www.google.com/search?q=Procedure+Linkage+Table) on Linux or
[Delay-loaded DLLs](https://learn.microsoft.com/en-us/cpp/build/reference/linker-support-for-delay-loaded-dlls) on
Windows.

The generated crate also provides a management API that allows:
- Overriding the dynamic library's file name.
- Supplying a dynamic library handle directly.
- Controlling the loading of pre-defined API groups, which are organized at build time. The management API enables you
  to check whether all APIs in a group were successfully resolved at runtime, allowing you to avoid calling APIs that
  are unavailable in the installed version of the library.

# Limitations
Weaklink supports transparent redirection only for code symbols (functions); handling data symbols would require
explicit linker support. However, you can still work with data symbols by manually wrapping them, though this requires
changes in your code. Specifically, you'll need to call a function that returns the address of the data rather than
accessing the data directly.

# Supported OS and architectures:
- Linux: x86_64, arm, aarch64
- MacOS: x86_64, arm64
- Windows: x86_64
