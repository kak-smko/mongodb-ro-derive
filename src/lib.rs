mod collection;
mod column;
mod model;

use proc_macro::TokenStream;
use syn::{ parse_macro_input, DeriveInput};
use quote::quote;
use crate::collection::{CollectionAttr, CollectionMeta};
use crate::model::impl_model;

#[proc_macro_derive(Model, attributes(model))]
pub fn derive(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let attrs = CollectionAttr::from_attrs(&ast.attrs);
    let collection = CollectionMeta::new(&ast, &attrs);

    //panic!("{:#?}", collection);
    let model = impl_model(&collection);

    TokenStream::from(quote! {
            #model
        })
}


