use proc_macro::TokenStream;

mod global;

///```rust
///[dependencies]
/// wd_tools = {features = ["sync"]} # version >= 0.13.14
/// wd_macro = "0.4" # version >= 0.4
///```
///
/// ```rust
/// mod static_mod {
///    #[derive(Default,Debug)]
///    #[wd_macro::global]
///    pub struct Config{
///        pub name:String,
///    }
///}
///
///
///fn main() {
///    use static_mod::Config;
///    let _:() = Config::lock_ref_mut(|x|{
///        x.name = "teshin".into();
///    });
///    let name:String = Config::unsafe_mut_ptr(|x|{
///        x.name.clone()
///    });
///    println!("name = {name}");
///}
#[proc_macro_attribute]
pub fn global(attr: TokenStream, item: TokenStream) -> TokenStream {
    global::global(attr, item)
}
