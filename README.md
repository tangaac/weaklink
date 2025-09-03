This crate implements weak dynamic linking across platforms (Linux, macOS, and Windows), making it easier to work with
dynamic libraries that may not be installed or may vary in version.

# When is this useful?

This crate is useful when your program depends on a dynamic library that may not be installed on the target system, or
when different versions of the library are in use. Instead of manually managing each API call with `dlopen`/`dlsym` (or
platform-specific equivalents), Weaklink automatically handles loading and symbol resolution at runtime, simplifying the
process.

This is especially helpful when calling into plugins that export mangled symbols (such as C++ or Rust), since determining
the mangled symbol name for a function can be non-trivial.

# How does it work?

Weaklink generates a Rust crate with function stubs that mirror the public APIs of the original dynamic library. These
stubs are compiled into a static library and linked to your main program. When a stubbed API is called, Weaklink
dynamically loads the original library, resolves the symbol, and jumps to the appropriate function.

Conceptually, this is similar to the ELF
[Procedure Linkage Table](https://www.google.com/search?q=Procedure+Linkage+Table) on Linux or
[Delay-loaded DLLs](https://learn.microsoft.com/en-us/cpp/build/reference/linker-support-for-delay-loaded-dlls) on
Windows.

The generated crate also provides a management API that allows you to:
- Override the dynamic library's file name.
- Supply a dynamic library handle directly.
- Control the loading of predefined API groups, which are organized at build time. The management API lets you check
  whether all APIs in a group were successfully resolved at runtime, so you can avoid calling APIs that are unavailable
  in the installed version of the library.

# Limitations

Weaklink can only handle function symbols (code). It does not provide transparent support for data symbols (such as
global variables), because that would require explicit linker support.

If you need to work with data symbols, you must handle them manually in your code. This typically means replacing direct
data accesses with a function call that returns the address of the data, then dereferencing this address.

# Supported OS and architectures:

* Linux: x86_64, arm, aarch64
* MacOS: x86_64, arm64
* Windows: x86_64
