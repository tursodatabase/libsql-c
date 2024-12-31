use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use std::iter::repeat;

#[proc_macro_attribute]
pub fn signature(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = parse_macro_input!(attr as syn::Path);
    let input = parse_macro_input!(item as syn::ItemFn);
    
    let fn_name = &input.sig.ident;
    let arg_count = input.sig.inputs.len();

    let args = repeat(quote!(_)).take(arg_count);
    
    quote! {
        const _:() = {
            let _: [unsafe extern "C" fn(#(#args),*) -> _; 2] = [
                #path::#fn_name,
                #fn_name,
            ];
        };

        #input
    }.into()
}
