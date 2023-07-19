#[macro_export]//examle of macros vec
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
//example of define a processing macros
// use proc_macro;

// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {
// }