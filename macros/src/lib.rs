use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{emit_call_site_error, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{parse, Expr, LitStr};

#[proc_macro_error]
#[proc_macro]
pub fn back_reference(stream: TokenStream) -> TokenStream {
    let input: Expr = parse(stream).unwrap();
    if let Expr::Lit(n) = input {
        let n: usize = n.to_token_stream().to_string().parse().unwrap_or_default();

        if n == 0 {
            emit_call_site_error!("`n` must be an integer bigger than 0");
        }

        let back_ref = syn::Lit::Str(LitStr::new(&format!(r"\{}", n), Span::call_site()));
        quote!(#back_ref).into()
    } else {
        panic!(
            "Expected usize, found {}",
            input.to_token_stream().to_string()
        );
    }
}
