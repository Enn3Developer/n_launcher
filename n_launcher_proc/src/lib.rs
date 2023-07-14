use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitStr,
};

extern crate proc_macro;

struct Args {
    lit: LitStr,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit: LitStr = input.parse()?;
        Ok(Self { lit })
    }
}

#[proc_macro]
pub fn cross_path(input: TokenStream) -> TokenStream {
    let Args { lit } = parse_macro_input!(input as Args);
    let value = if cfg!(target_os = "windows") {
        lit.value().replace("/", "\\").replace(":", ";")
    } else {
        lit.value()
    };

    value.into_token_stream().into()
}
