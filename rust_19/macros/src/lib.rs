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


#[route(GET, "/")]//example of using attribute-macros 
fn index() {}

#[proc_macro_attribute]//signature of atribue-macros
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}//Here, we have two parameters of type TokenStream. ...
                                                                    //...The first is for the contents of the attribute: the GET,...
                                                                    //... "/" part. The second is the body of the item the...
                                                                    //... attribute is attached to: in this case, fn index()...
                                                                    // {} and the rest of the function’s body.