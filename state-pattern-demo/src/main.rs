
struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

impl Post {

    pub fn new() -> Post {
        Post {
            state: Some(
                Box::new(
                    Draft {}
                )
            ),
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn add_content(&mut self, content: &str) {
        self.content.push_str(content);
    }

    pub fn request_review(&mut self) {
        self.state = Some(
            self.state.take().unwrap().request_review()
        );
    }

    pub fn approve(&mut self) {
        self.state = Some(
            self.state.take().unwrap().approve()
        );
    }

    pub fn reject(&mut self) {
        self.state = Some(
            self.state.take().unwrap().reject()
        );
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {

}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {

}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {

}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}



fn main() {
    let mut post = Post::new();
    assert_eq!(post.content(), "");
    post.add_content("Some text...");
    assert_eq!(post.content(), "");
    post.request_review();
    assert_eq!(post.content(), "");
    post.approve();
    assert_eq!(post.content(), "Some text...");

    // test reject
    let mut post2 = Post::new();
    assert_eq!(post2.content(), "");
    post2.add_content("Some text...");
    assert_eq!(post2.content(), "");
    post2.request_review();
    assert_eq!(post2.content(), "");
    post2.reject(); // back to Draft
    post2.approve(); // nothing to do, stay Draft
    assert_eq!(post2.content(), "");
}
