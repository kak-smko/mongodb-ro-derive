use darling::FromMeta;
use crate::collection::CollectionMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{ Path};
const PROXY_MODEL_STRUCT_PATH: &str = "mongodb_ro::model::Model";
pub fn impl_model(collection: &CollectionMeta) -> TokenStream {
    let ident = &collection.ident;
    let col_name = &collection.name;
    let add_times = &collection.add_times;
    let model = Path::from_string(PROXY_MODEL_STRUCT_PATH).unwrap();
    let mut columns = "{".to_string();

    for item in &collection.columns {
        let name = match &item.attr.name{
            None => {"null".to_string()}
            Some(a) => {format!("{:?}",a.value())}
        };
        let text = match &item.attr.text{
            None => {"null".to_string()}
            Some(a) => {format!("{:?}",a.value())}
        };
        let item_name=item.name.clone();

        columns=format!("{columns}{:?}:{{ \"asc\":{},\"desc\":{},\"unique\":{},\"sphere2d\":{},\"text\":{},\"hidden\":{},\"name\":{} }},",item_name,
                                item.attr.asc,item.attr.desc,item.attr.unique,item.attr.sphere2d,text,item.attr.hidden,name);
    }
    let columns=columns.strip_suffix(",").unwrap();
    let columns=format!("{columns} }}");


    quote! {
        impl #ident {
            pub fn new_model<'a>(db: &mongodb::Database) -> #model<'a , Self>{
                let model = #model::<Self>::new(db , #col_name , #columns, #add_times);
                model
            }
        }
    }
}
