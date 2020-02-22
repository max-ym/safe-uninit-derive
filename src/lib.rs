extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::TokenStreamExt;

#[proc_macro_derive(SafeUninit)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    let mut st = syn::parse_macro_input!(item as syn::ItemStruct);
    let ident = &st.ident;
    let fields = &mut st.fields;

    let mut tok = quote!();
    let mut is_named = true;
    for f in fields {
        if let Some(name) = f.ident.as_ref() {
            tok.append_all(quote!(#name: safe_uninit::SafeUninit::safe_uninit(),));
            is_named = true;
        } else {
            is_named = false;
            tok.append_all(quote!(safe_uninit::SafeUninit::safe_uninit(),));
        }
    }
    let tok = if is_named {
        quote!({#tok})
    } else {
        quote!((#tok))
    };

    quote!(
        unsafe impl safe_uninit::SafeUninit for #ident {
            fn safe_uninit() -> Self {
                #ident #tok
            }
        }
    ).into()
}

#[cfg(test)]
mod test {

    #[test]
    fn derive0() {
        let t = trybuild::TestCases::new();
        t.pass("src/tests/pass.rs");
        t.compile_fail("src/tests/fail.rs");
    }
}
