// lib example of structure of program with gui 
// make ui component drawable 
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // components shoul be drawable
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    // run screen
    pub fn run(&self) {
        // draws every component
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// define a button component
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
// make button drawable
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
// define a select box component
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}
// make button drawable
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}