pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new () -> Post {
        return Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text (&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content (&self) -> &str {
        /* NOTE:
         * as_ref() returns an Option<&Box<dyn State>>
         * unwrap() is okay here because a None value is impossible
         * Deref coercion happens upon calling content(self)
         */
        return self.state.as_ref().unwrap().content(self);
    }

    pub fn request_review (&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve (&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    /* NOTE: These methods are only valid when called on a Box
     * holding a State trait object
     */
    fn request_review (self: Box<Self>) -> Box<dyn State>;
    fn approve (self: Box<Self>) -> Box<dyn State>;
    fn content<'a> (&self, post: &'a Post) -> &'a str {
        return "";
    }
}

struct Draft {}

impl State for Draft {
    fn request_review (self: Box<Self>) -> Box<dyn State> {
        return Box::new(PendingReview {});
    }

    fn approve (self: Box<Self>) -> Box<dyn State> {
        return self;
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review (self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn approve (self: Box<Self>) -> Box<dyn State> {
        return Box::new(Published {});
    }
}

struct Published {}

impl State for Published {
    fn request_review (self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn approve (self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn content<'a> (&self, post: &'a Post) -> &'a str {
        return &post.content;
    }
}
