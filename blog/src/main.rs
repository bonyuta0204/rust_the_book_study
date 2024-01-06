use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad for lunch");

    let post = post.request_review();

    let post = post.approve();

    println!("Content: {}", post.content());
}
