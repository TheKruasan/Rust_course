pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    //public function
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    //public function
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    
    //public function
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    //public function
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    //public function
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}
//declare trait State
trait State {
    //declare functions without default description
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    //declare function with default description
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}
struct Draft {}
//implements State for Draft
impl State for Draft {
    //realization of functions from trait
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}
//implements State for PendingReview
impl State for PendingReview {
    //realization of functions from trait
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}


struct Published {}

impl State for Published {
    //realization of functions from trait
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

//declare new struct
pub struct Post {
    content: String,
}
//declare new struct
pub struct DraftPost {
    content: String,
}

impl Post {
    //function in struct that return another new struct
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    //realization of some functions
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    //return another struct
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}
//declare new struct
struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    //create function for this struct
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}