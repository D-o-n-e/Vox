use proc_macro2;
use proc_macro;
use quote::quote;
use VoxAPI::*;
use serde_json::*;
use std::collections::HashMap;
//use syn::*;



#[proc_macro_attribute]
pub fn extends(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let api = parse_api();
    let properties: proc_macro2::TokenStream = parse_properties(attr.to_string(), &api).parse().unwrap();
    let methods: proc_macro2::TokenStream = parse_methods(attr.to_string(), &api).parse().unwrap();
    let attr = proc_macro2::TokenStream::from(attr);
    let item = proc_macro2::TokenStream::from(item);
    let mut is_struct: bool = false;
    let mut struct_name: proc_macro2::Ident;
    let mut result = proc_macro2::TokenStream::new();
    for i in item.into_iter(){
        if is_struct{
            struct_name = proc_macro2::Ident::new(&(i.to_string())[..], i.span());
            is_struct = false;
            result = proc_macro2::TokenStream::from(quote!{
                #[derive(Debug)]
                pub struct #struct_name{
                    #properties
                }

                impl #struct_name{
                    #methods
                }
            })
        }
        if i.to_string() == "struct".to_string(){
            is_struct = true;
        }
        ;
            
        }
    proc_macro::TokenStream::from(result)
}

fn parse_properties(class_name: String, data: &HashMap<String, class>) -> String{
    let mut prop: String = "".to_owned();
    for (name, node) in data{
        if *name == class_name{
            for p in &node.properties{
                prop.push_str(p);
            }
        }
    }
    prop
}

fn parse_methods(class_name: String, data: &HashMap<String, class>) -> String{
    let mut meth: String = "".to_owned();
    for (name, node) in data{
        if *name == class_name{
            for m in &node.methods{
                meth.push_str(m);
            }
        }
    }
    meth
}