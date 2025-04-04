use syn::{Attribute, Meta};

const DERIVE_TYPE: &str = "Type";

pub fn has_derive_type(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if let Meta::List(meta_list) = &attr.meta {
            if meta_list.path.is_ident("derive") {
                for nested_meta in meta_list
                    .parse_args_with(
                        syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                    )
                    .unwrap_or_default()
                {
                    if let Meta::Path(path) = nested_meta {
                        if path.is_ident(DERIVE_TYPE) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn type_to_typescript(type_name: &str) -> &str {
    match type_name {
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64"
        | "isize" | "usize" => "number",
        "str" | "String" | "char" => "string",
        "bool" => "boolean",
        _ => type_name,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use syn::{ItemEnum, ItemStruct, ItemType, parse_quote};

    #[test]
    fn it_correctly_checks_if_derive_type_is_present() {
        let test_struct: ItemStruct = parse_quote! {
            #[derive(Type)]
            struct MyStruct;
        };

        assert!(has_derive_type(&test_struct.attrs));

        let test_type: ItemType = parse_quote! {
            #[derive(Type)]
            type MyType = i32;
        };

        assert!(has_derive_type(&test_type.attrs));

        let test_enum: ItemEnum = parse_quote! {
            #[derive(Type)]
            enum MyEnunType {
                Variant1,
                Variant2,
            }
        };

        assert!(has_derive_type(&test_enum.attrs));

        let test_struct_without_type: ItemStruct = parse_quote! {
            struct MyStruct;
        };

        assert!(!has_derive_type(&test_struct_without_type.attrs));

        let multi_derived_trait: ItemStruct = parse_quote! {
            #[derive(Type, Serialize, Debug)]
            struct MyStruct;
        };

        assert!(has_derive_type(&multi_derived_trait.attrs));
    }
}
