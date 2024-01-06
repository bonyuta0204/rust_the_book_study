use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad for lunch");

    let mut pending_post = post.request_review();

    pending_post.approve();
    pending_post.approve();

    println!("Content: {}", pending_post.to_post().unwrap().content());
}
