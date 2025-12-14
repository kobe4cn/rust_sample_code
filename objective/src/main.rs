use objective::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    post.request_review();
    post.approve();
    post.publish();
    assert_eq!(post.content(), "I ate a salad for lunch today");
    println!("content: {}", post.content());
}
