use proc_macro::TokenStream;
use syn::{LitByteStr, LitStr, Token, parse::Parse, parse_macro_input};

mod consts;
mod impls;

struct MacroInput {
    flag: LitStr,
    key: LitByteStr,
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let flag = input.parse()?;

        input.parse::<Token![,]>()?;

        let key = input.parse()?;

        Ok(Self { flag, key })
    }
}

#[proc_macro]
pub fn obfuscate_flag(input: TokenStream) -> TokenStream {
    // Expand so that we
    let input = parse_macro_input!(input as MacroInput);
    impls::obfuscate_flag_impl(input.flag.value(), input.key.value()).into()
}
