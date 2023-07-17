use blog::Post;

fn main() {
    let mut post = Post::new();//new struct entity 
    //use method from Post
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
