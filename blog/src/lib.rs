pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
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
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approval_count: 0,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    approval_count: u32,
}

impl PendingReviewPost {
    const APPROVAL_NEEDED: u32 = 2;
    pub fn approve(&mut self) {
        self.approval_count += 1;
    }

    pub fn to_post(&self) -> Option<Post> {
        if self.approval_count >= Self::APPROVAL_NEEDED {
            Some(Post {
                content: self.content.clone(),
            })
        } else {
            None
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_published() {
        let mut post = Post::new();
        post.add_text("I ate salad for lunch");
        let mut post = post.request_review();
        post.approve();
        assert!(post.to_post().is_none());
        post.approve();

        let approved_post = post.to_post();
        assert!(approved_post.is_some());
        assert_eq!(approved_post.unwrap().content(), "I ate salad for lunch");
    }

    #[test]
    fn rejection() {
        let mut post = Post::new();
        post.add_text("I ate salad for lunch");
        let post = post.request_review();
        let post = post.reject();
        assert_eq!("I ate salad for lunch", post.content);
    }
}
