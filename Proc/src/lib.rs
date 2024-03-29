use proc_macro::TokenStream;

mod ui_element;

#[proc_macro_attribute]
pub fn ui_element(_: TokenStream, input: TokenStream) -> TokenStream {
    ui_element::ui_element(input)
}

#[proc_macro]
pub fn ui_element_trait(_: TokenStream) -> TokenStream {
    ui_element::ui_element_trait()
}
