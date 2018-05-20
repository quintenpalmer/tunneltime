#![recursion_limit = "128"]

extern crate proc_macro;

extern crate syn;

#[macro_use]
extern crate quote;

#[proc_macro_derive(FromRow)]
pub fn from_row_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the output, possibly using quasi-quotation
    let expanded = derive_from_row(&input);

    // Hand the output tokens back to the compiler
    expanded.into()
}

fn derive_from_row(ast: &syn::DeriveInput) -> quote::Tokens {
    let field_assignments = match ast.data {
        syn::Data::Struct(ref data) => {
            if data.fields.iter().len() == 0 {
                panic!("#[derive(FromRow)] must have at least one field");
            }

            let mut field_assignments = Vec::new();
            for field_syn in data.fields.iter() {
                let field = field_syn.ident.as_ref().unwrap();
                let field_str = format!("{}", field);
                field_assignments.push(quote! { #field: row.get_opt(#field_str).ok_or(
                    ::postgres_extra::Error::Extra(
                        ::postgres_extra::ExtraError::ParseNonexistentField(#field_str.to_owned())
                    ))?
                .map_err(::postgres_extra::Error::Postgres)? });
            }
            field_assignments
        }
        syn::Data::Enum(_) => panic!("#[derive(FromRow)] can only be used with structs"),
        syn::Data::Union(_) => panic!("#[derive(FromRow)] can only be used with structs"),
    };

    let name = &ast.ident;

    // Helper is provided for handling complex generic types correctly and effortlessly
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics ::postgres_extra::FromRow for #name #ty_generics #where_clause {
            fn parse_row(row: ::postgres::rows::Row) -> Result<Self, ::postgres_extra::Error> {
                return Ok(#name { #(#field_assignments),* });
            }
        }
    }
}
