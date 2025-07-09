
# Chapter 1: 

Rust is an ahead-of-time compiled language, meaning you can
compile a program and give the executable to someone else, and they can
run it even without having Rust installed.

If you’re continually checking your work while writing the code, using cargo
check will speed up the process! As such, many Rustaceans run cargo check
periodically as they write their program to make sure it compiles.

When your project is finally ready for release, you can use cargo build
--release to compile it with optimizations.

If you’re benchmarking your code’s running time, be sure to run cargo build --release
and benchmark with the executable in target/release.

target/debug builds faster due to missing optimizations.

Build:
rustc for simple projects 
cargo for intricate large projects 

# Chapter 2:

Rust defaults to an i32, foo here is a 32 bit integer
so for let foo = bar 

Rust allows us to shadow the previous value of guess with a new one. Usually used for converting values in rust, you can reuse the same variable name to change its type.

the underscore "_" is a catchall value, Err(_) we’re saying we want to match all Err values, no matter what
information they have inside them.

Rust does not infer field types inside a struct.

Rust is statically typed with no runtime type info (RTTI) by default.
