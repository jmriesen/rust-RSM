use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, Span};
use quote::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct NodeRef {
    r#type: String,
    named: bool,
}

#[derive(Deserialize)]
struct TypeConfig {
    multiple: bool,
    required: bool,
    types: Vec<NodeRef>,
}

#[derive(Deserialize)]
struct Node {
    r#type: String,
    named: bool,
    children: Option<TypeConfig>,
    #[serde(default)]
    fields: HashMap<String, TypeConfig>,
}

#[proc_macro]
pub fn models(_: TokenStream) -> TokenStream {
    //TODO remove this hard coding at some point.
    let data = include_str!("../../tree-sitter-M/src/node-types.json");
    let mut nodes: Vec<Node> = serde_json::from_str(data).expect("Unable to parse");

    nodes.retain(|x| x.named);
    for node in &mut nodes {
        if let Some(config) = &mut node.children {
            config.r#types.retain(|x| x.named);
        }
        for config in node.fields.values_mut() {
            config.types.retain(|x| x.named);
        }
    }
    let nodes = nodes.iter().map(|node| {
        let type_name = Ident::new(&node.r#type, Span::call_site());
        let life_time = syn::Lifetime::new("'a", Span::call_site());
        /*
        if there are children we should have the method.
        If there is one child then then the return type should that node type.
        If it is not require and not multiple then it should be Optionl.
        If if there could be multiple it should be a vec.
         */
        // TODO refactor this code.
        // TODO does are fields considered children for Node::named_children.
        // TODO I need fields for UnaryExpresstion.
        let children_code = node
            .children
            .as_ref()
            .map(|config| impl_getter(&type_name, None, config));

        let field_code = node
            .fields
            .iter()
            .map(|(name, config)| impl_getter(&type_name, Some(name), config));

        quote! {
            #[derive(Clone)]
            pub struct #type_name <#life_time> {
                node:Node <#life_time>
            }
            impl <#life_time>#type_name<#life_time> {
                fn create(node: Node<#life_time>) -> Self{
                    Self{
                        node
                    }
                }
                pub fn node(&self)->&Node<#life_time>{
                    &self.node
                }
            }

            #children_code

            #(#field_code)*
        }
    });
    let tokens = quote! {
        #(#nodes)*
        //-------
    };

    tokens.into()
}

fn def_enum(name: &Ident, config: &TypeConfig) -> proc_macro2::TokenStream {
    let life_time = syn::Lifetime::new("'a", Span::call_site());
    let options = config
        .types
        .iter()
        .filter(|x| x.named)
        .map(|x| x.r#type.as_str())
        .map(|x| (Literal::string(x), Ident::new(x, Span::call_site())));

    let enum_varients = options.clone().map(|(_, i)| quote! {#i(#i<#life_time>)});
    let match_arms = options.map(|(s, i)| quote! {#s =>Self::#i(#i::create(node))});

    let declaration = quote! {
        #[derive(Clone)]
        pub enum #name<#life_time>{
            #(#enum_varients),*
        }
    };
    let impl_block = quote! {

        impl <#life_time>#name<#life_time> {
            fn create(node: Node<#life_time>) -> Self{
                match node.kind(){
                    #(#match_arms),*,
                    _ => unreachable!()
                }
            }
        }
    };
    quote! {
        #declaration
        #impl_block
    }
}

fn get_return_type(
    parent: &Ident,
    subtype: &str,
    config: &TypeConfig,
) -> (Ident, proc_macro2::TokenStream) {
    if config.types.len() > 1 {
        let subtype = format!("{}{}", subtype[0..1].to_uppercase(), &subtype[1..]);
        let type_name = format_ident!("{}{}", parent, subtype);
        let enum_def = def_enum(&type_name, config);
        (type_name, enum_def)
    } else {
        (format_ident!("{}", config.types[0].r#type), quote! {})
    }
}

fn impl_getter(
    parent: &Ident,
    field_name: Option<&str>,
    config: &TypeConfig,
) -> proc_macro2::TokenStream {
    let subtype = field_name.unwrap_or("children");

    let life_time = syn::Lifetime::new("'a", Span::call_site());
    let (type_name, type_def) = get_return_type(parent, subtype, config);

    let (impl_line, return_type) = if config.multiple {
        (
            quote! {children.collect()},
            quote! {Vec<#type_name<#life_time>>},
        )
    } else if !config.required {
        (
            quote! {children.next()},
            quote! {Option<#type_name<#life_time>>},
        )
    } else {
        (
            quote! {children.next().unwrap()},
            quote! {#type_name<#life_time>},
        )
    };

    let selector = if let Some(field) = field_name {
        let field = Literal::string(field);
        quote! {children_by_field_name(#field,&mut cursur).filter(|x|x.is_named())}
    } else {
        quote! {named_children(&mut cursur)}
    };

    let method = Ident::new(subtype, Span::call_site());
    quote! {
        #type_def
        impl <#life_time>#parent<#life_time> {
            pub fn #method(&self) -> #return_type{
                let mut cursur = self.node.walk();
                let mut children = self.node.#selector
                    .map(#type_name::create);
                #impl_line
            }
        }
    }
}
