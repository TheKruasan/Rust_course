mod gui;
use gui::*;
// example of structure of program with gui 
fn main() {
    // create screen
    let screen = Screen {
        // add components
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    // run screen
    screen.run();
}