use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ImplItemFn, ItemImpl};

#[derive(Default, FromMeta)]
struct EnemyFnAttributes {
    damage: bool,
}

impl EnemyFnAttributes {
    fn get_function_tokens(&self) -> Vec<TokenStream> {
        let mut function_tokens = Vec::new();

        if self.damage {
            function_tokens.push(quote! {
                #[func]
                pub fn p_damage(&mut self, amount: real) {
                    self.damage(amount);
                }
            }.into());
        }

        function_tokens
    }
}

#[proc_macro_attribute]
pub fn export_enemy_functions(arguments: TokenStream, input: TokenStream) -> TokenStream {
    let mut parsed_input = parse_macro_input!(input as ItemImpl);

    let nested_meta = match NestedMeta::parse_meta_list(arguments.into()) {
        Ok(value) => value,
        Err(_) => panic!("error reading arguments."),
    };

    let parsed_arguments = match EnemyFnAttributes::from_list(&nested_meta) {
        Ok(value) => value,
        Err(_) => panic!("error reading arguments."),
    };

    for function in parsed_arguments.get_function_tokens() {
        parsed_input.items.insert(0, syn::parse::<ImplItemFn>(function).expect("internal attribute error.").into());
    }


    parsed_input.to_token_stream().into()
}
