use proc_macro2::Ident;
use syn::{Attribute, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, LitBool, LitStr};

use crate::column::ColumnMeta;
use structmeta::StructMeta;

#[derive(Debug, Clone)]
pub struct CollectionMeta {
    pub name: String,
    pub req: String,
    pub add_times: bool,
    pub ident: Ident,
    pub columns: Vec<ColumnMeta>,
}

impl CollectionMeta {
    pub fn new(ast: &DeriveInput, attrs: &[CollectionAttr]) -> Self {
        let ident = &ast.ident;
        let name = if let Some(value) = attrs.iter().find_map(|a| a.collection.as_ref()) {
            value.value()
        } else {
            let text = ident.to_string();
            let mut first = true;
            let result = text
                .chars()
                .map(|c| {
                    let r = if c.is_uppercase() {
                        if !first {
                            format!("_{}", c.to_lowercase())
                        } else {
                            format!("{}", c.to_lowercase())
                        }
                    } else {
                        c.to_string()
                    };
                    first = false;
                    r
                })
                .collect::<String>();
            result
        };

        let add_times = if let Some(value) = attrs.iter().find_map(|a| a.add_times.as_ref()) {
            value.value()
        } else {
            true
        };
        let req = if let Some(value) = attrs.iter().find_map(|a| a.req.as_ref()) {
            value.value()
        } else {
            "bool".to_string()
        };

        let columns = ColumnMeta::from_fields(ast.fields());

        Self {
            name,
            req,
            add_times,
            ident: ident.clone(),
            columns,
        }
    }
}

#[derive(StructMeta, Debug)]
pub struct CollectionAttr {
    pub collection: Option<LitStr>,
    pub add_times: Option<LitBool>,
    pub req: Option<LitStr>,
}

impl CollectionAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Vec<Self> {
        attrs
            .iter()
            .filter(|&a| a.path().is_ident("model"))
            .map(|a| a.parse_args().unwrap())
            .collect()
    }
}

pub trait DeriveInputExt {
    fn fields(&self) -> syn::punctuated::Iter<Field>;
}

impl DeriveInputExt for DeriveInput {
    fn fields(&self) -> syn::punctuated::Iter<Field> {
        let fields = match &self.data {
            Data::Struct(DataStruct { ref fields, .. }) => fields,
            _ => panic!("#[resql] can only be used on structs"),
        };
        let fields = match fields {
            Fields::Named(FieldsNamed { named, .. }) => named,
            _ => panic!("#[resql] can only be used on structs with named fields"),
        };
        fields.iter()
    }
}
