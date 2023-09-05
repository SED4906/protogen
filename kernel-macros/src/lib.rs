extern crate proc_macro;

#[proc_macro]
pub fn import_isrs(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut isrs = vec![];
    for i in 0..256 {
        isrs.push(["extern \"x86-interrupt\" {fn isr","(stack: InterruptStackFrame, error_code: u64);}"].join(&i.to_string()));
    }
    isrs.join("\n").parse().unwrap()
}