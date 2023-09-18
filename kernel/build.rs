fn main() {
    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed=linker.ld");

    cc::Build::new()
        .compiler("as")
        .no_default_flags(true)
        .warnings(false)
        .warnings_into_errors(false)
        .extra_warnings(false)
        .files(["src/ivt.s", "src/switch.s"])
        .compile("asm.o");
}
