use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields, parse_macro_input};

#[proc_macro_derive(Enemy, attributes(enemy_data))]
pub fn derive_enemy(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let name = parsed_input.ident;

    let mut output = quote! {};
    let mut data_field = None;

    match parsed_input.data {
        Data::Struct(ref data_struct) => {
            match data_struct.fields {
                Fields::Named(ref named_fields) => {
                    for field in named_fields.named.iter() {
                        for attribute in field.attrs.iter() {
                            if attribute.path().is_ident("enemy_data") {
                                data_field = Some(field.ident.as_ref().unwrap());
                            }
                        }
                    }
                },
                _ => (),
            }
        },
        _ => {},
    }

    if let Some(data_field) = data_field {
        output = quote! {
            impl Enemy for #name {
                fn damage(&mut self, amount: real) {
                    self.#data_field.health -= amount;
                    if self.#data_field.health <= 0.0 {
                        self.die();
                    }
                }

                fn die(&mut self) {
                    self.i_die();
                }
            }
        };
    }

    output.into()
}
