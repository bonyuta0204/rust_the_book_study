use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad for lunch");

    post.request_review();

    post.approve();

    println!("Content: {}", post.content());
}
