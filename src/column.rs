use structmeta::{ StructMeta};
use syn::{Attribute, Field, LitStr, Meta, Token};
use syn::punctuated::Punctuated;

#[derive(Clone, Debug)]
pub struct ColumnMeta {
    /// Name of the column in the database
    pub name: String,
    pub attr:ColumnAttr
}

impl ColumnMeta {


    pub fn from_fields<'a>(fields: impl Iterator<Item = &'a Field>) -> Vec<Self> {
        fields.map(|f| ColumnMeta::from_field(f)).collect()
    }

    pub fn from_syn(ident: &syn::Ident, ty: &syn::Type,attr:ColumnAttr) -> Self {
        let syn::Type::Path(_ty) = &ty else {
            panic!("No type on field {}", ident);
        };

        Self {
            name: ident.to_string(),
            attr
        }
    }
    pub fn from_field(f: &Field) -> Self {
        let ident = f.ident.as_ref().expect("No ident on field");
        let attr = ColumnAttr::from_attrs(&f.attrs);
        let column = ColumnMeta::from_syn(ident, &f.ty,attr);

        column
    }
}


/// Available attributes on a column (struct field)
#[derive(StructMeta,Debug,Clone)]
pub struct ColumnAttr {
    pub asc: bool,
    pub desc: bool,
    pub unique: bool,
    pub sphere2d: bool,
    pub text: Option<LitStr>,
    pub hidden: bool,
    pub name: Option<LitStr>,
}

impl ColumnAttr {
    pub fn from_attrs(ast: &[Attribute]) -> Self {
        let mut attr =ColumnAttr{
            asc: false,
            desc: false,
            sphere2d: false,
            text: None,
            unique: false,
            hidden: false,
            name: None,
        };
        for item in ast.iter(){
            if item.path().is_ident("model") {
                let nested = item.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated).unwrap();
                for meta in nested {
                    match meta {
                        // #[repr(C)]
                        Meta::Path(path) if path.is_ident("asc") => {
                            attr.asc = true;
                        }
                        Meta::Path(path) if path.is_ident("desc") => {
                            attr.desc = true;
                        }
                        Meta::Path(path) if path.is_ident("sphere2d") => {
                            attr.sphere2d = true;
                        }
                        Meta::Path(path) if path.is_ident("unique") => {
                            attr.unique = true;
                        }
                        Meta::Path(path) if path.is_ident("hidden") => {
                            attr.hidden = true;
                        }

                        // #[repr(align(N))]
                        Meta::List(meta) if meta.path.is_ident("name") => {
                            let lit: LitStr = meta.parse_args().unwrap();
                            attr.name = Some(lit);
                        }
                        Meta::List(meta) if meta.path.is_ident("text") => {
                            let lit: LitStr = meta.parse_args().unwrap();
                            attr.text = Some(lit);
                        }

                        _ => {}
                    }
                }

            }

        }
        attr
    }
}

