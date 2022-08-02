// use std::{env, path::PathBuf};
use std::path::Path;

extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed};


//honestly not sure how to implement this...
// pub fn get_exec_dir() -> String
// {
//     let cwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
//     cwd + "/"
// }
// pub fn get_asset_dir() -> String
// {
//     get_exec_dir()+"assets/"
// }
// pub fn get_shaders_dir() -> String
// {
//     get_asset_dir()+"shaders/"
// }
// pub fn get_shader(shader_name:&str) -> String
// {
//     get_shaders_dir()+shader_name
// }

// //TODO this will be string parser...
// #[macro_export]
// macro_rules! random_parser {
//     ($($args:expr),*) => {
// TODO CFG only use when variable itemsafety is true.,?
// Finds and returns a file, will throw error if file is not available
// #[macro_export]
// macro_rules! find_file{
//     // ($($args:expr),*) => {
//     //     let mut result: String = String::from("");
//     //     $(
//     //         let tempstr: String = format!("{}", format_args!("{}", $args));
//     //         result.push_str(&tempstr[..]);
//     //     )*
//     //     println!("{}", result);
//     //     // include_str!(result);
//     //     assert!(Path::new(&result).exists());
//     // };

//     ($($args:expr),*) => {
//         let mut result: PathBuf = PathBuf::new();
//         $(
//             // let tempstr: String = format!("{}", format_args!("{}", $args));
//             result = result.join($args);
//         )*
//         //TODO comment out
//         println!("{}", result.display());
//         let bo = result.exists();
//         assert!(bo);
//         result
//     };
// }

// #[proc_macro]
// pub fn file(path:TokenStream) -> TokenStream {
//     // Parse the input tokens into a syntax tree
//     let a = parse_macro_input!(path);

//     println!("{}", Path::new(&path).exists());

//     //TODO should return filepath if valid.
//     TokenStream::new()
// }

#[proc_macro]
pub fn my_proc_macro(input: TokenStream) -> TokenStream {
    // let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    // //TODO how to ... maybe turn data into string????

    // // let s = input.to_string();
    // let ea:String = data.into();
    
    // let description = match data {
    //     syn::Data::Struct(s) => match s.fields {
    //         syn::Fields::Named(FieldsNamed { named, .. }) => {
    //         let idents = named.iter().map(|f| &f.ident);
    //         format!(
    //             "a struct with these named fields: {}",
    //             quote! {#(#idents), *}
    //         )
    //         }
    //         syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
    //         let num_fields = unnamed.iter().count();
    //         format!("a struct with {} unnamed fields", num_fields)
    //         }
    //         syn::Fields::Unit => format!("a unit struct"),
    //     },
    //     syn::Data::Enum(DataEnum { variants, .. }) => {
    //         let vs = variants.iter().map(|v| &v.ident);
    //         format!("an enum with these variants: {}", quote! {#(#vs),*})
    //     }
    //     syn::Data::Union(DataUnion {
    //         fields: FieldsNamed { named, .. },
    //         ..
    //     }) => {
    //         let idents = named.iter().map(|f| &f.ident);
    //         format!("a union with these named fields: {}", quote! {#(#idents),*})
    //     }
    //     };

    // let output = quote! {
    //     impl #ident {
    //         fn describe() {
    //         println!("{} is {}.", stringify!(#description));
    //         }
    //     }
    // };
    // output.into()

    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    assert!(Path::new("path").exists());

    // assert!(false);




    TokenStream::new()
}

#[proc_macro_derive(Describe)]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let description = match data {
    syn::Data::Struct(s) => match s.fields {
        syn::Fields::Named(FieldsNamed { named, .. }) => {
        let idents = named.iter().map(|f| &f.ident);
        format!(
            "a struct with these named fields: {}",
            quote! {#(#idents), *}
        )
        }
        syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
        let num_fields = unnamed.iter().count();
        format!("a struct with {} unnamed fields", num_fields)
        }
        syn::Fields::Unit => format!("a unit struct"),
    },
    syn::Data::Enum(DataEnum { variants, .. }) => {
        let vs = variants.iter().map(|v| &v.ident);
        format!("an enum with these variants: {}", quote! {#(#vs),*})
    }
    syn::Data::Union(DataUnion {
        fields: FieldsNamed { named, .. },
        ..
    }) => {
        let idents = named.iter().map(|f| &f.ident);
        format!("a union with these named fields: {}", quote! {#(#idents),*})
    }
    };

    let output = quote! {
    impl #ident {
        fn describe() {
        println!("{} is {}.", stringify!(#ident), #description);
        }
    }
    };
    output.into()
}