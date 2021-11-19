use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
	parse_macro_input,
	Data,
	DeriveInput,
	Fields,
	Generics,
	Ident,
};

#[proc_macro_derive(Resource)]
pub fn derive_editable(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	expand_derive_resource(&input).into()
}

fn expand_derive_resource(input: &DeriveInput) -> TokenStream2 {
	match &input.data {
		Data::Struct(data) => {
			implement_struct_resource(&input.ident, &input.generics, &data.fields)
		}
		_ => unimplemented!("Enum not supported"),
	}
}

fn implement_struct_resource(ident: &Ident, generics: &Generics, fields: &Fields) -> TokenStream2 {
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	// let mut tokens = Vec::new();
	// match fields {
	// 	Fields::Named(fields) => {
	// 		for field in fields.named.iter() {
	// 			match &field.ident {
	// 				Some(ident) => {
	// 					tokens.push(quote! {
	// 						// newport_editor::Editable::edit(&mut self.#ident, stringify!(#ident), ui);
	// 					});
	// 				}
	// 				None => {}
	// 			}
	// 		}
	// 	}
	// 	_ => panic!("Named are only supported"),
	// }

	quote! {
		impl #impl_generics resources::Resource for #ident #ty_generics #where_clause {
			// fn edit(&mut self, name: &str, ui: &mut newport_editor::Ui) {
			// 	newport_editor::CollapsingHeader::new(name)
			// 		.default_open(true)
			// 		.show(ui, |ui|{
			// 			#(#tokens)*
			// 		});
			// }
		}
	}
}