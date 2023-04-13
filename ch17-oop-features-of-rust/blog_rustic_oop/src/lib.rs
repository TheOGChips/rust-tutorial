pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new () -> DraftPost {
        return DraftPost {
            content: String::new(),
        };
    }

    pub fn content (&self) -> &str {
        return &self.content;
    }
}

impl DraftPost {
    pub fn add_text (&mut self, text: &str) {
        return self.content.push_str(text);
    }

    pub fn request_review (self) -> PendingReviewPost {
        return PendingReviewPost {
            content: self.content,
        };
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve (self) -> Post {
        return Post {
            content: self.content,
        };
    }
}
