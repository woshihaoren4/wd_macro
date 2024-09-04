extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

pub fn global(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let get_function = format!("__get_static_var_{}", input.ident.to_string());
    let struct_name = input.ident.to_string();
    let static_var = generate_static_var(struct_name.as_str(), get_function.as_str());
    let struct_name = proc_macro2::Ident::new(struct_name.as_str(), proc_macro2::Span::call_site());
    let get_function =
        proc_macro2::Ident::new(get_function.as_str(), proc_macro2::Span::call_site());
    let tokens = quote! {
        #static_var
        #input
        wd_tools::share!(#struct_name,#get_function);
    };

    TokenStream::from(tokens)
}

fn generate_static_var(struct_name: &str, get_function: &str) -> proc_macro2::TokenStream {
    let static_global_struct_var = format!("__SGSV_{}", struct_name);
    let static_global_struct_var = proc_macro2::Ident::new(
        static_global_struct_var.as_str(),
        proc_macro2::Span::call_site(),
    );
    let static_global_once_var = format!("__SGOV_{}", struct_name);
    let static_global_once_var = proc_macro2::Ident::new(
        static_global_once_var.as_str(),
        proc_macro2::Span::call_site(),
    );
    let get_function = proc_macro2::Ident::new(get_function, proc_macro2::Span::call_site());
    let panic_info = format!("{struct_name} init failed");
    let struct_name = proc_macro2::Ident::new(struct_name, proc_macro2::Span::call_site());
    let ts = quote! {
    static mut #static_global_struct_var:Option<wd_tools::sync::AsyncMutex<#struct_name>> =  None;
    static mut #static_global_once_var:std::sync::Once = std::sync::Once::new();

    fn #get_function() -> &'static wd_tools::sync::AsyncMutex<#struct_name> {
        unsafe {
            #static_global_once_var.call_once(||{
                #static_global_struct_var = Some(wd_tools::sync::AsyncMutex::new(std::default::Default::default()))
            });
            match #static_global_struct_var {
                Some(ref s) => {
                    s
                }
                None => {
                    panic!(#panic_info)
                }
            }
        }
    }
        };
    ts
}
