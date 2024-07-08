

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn can_add_text<'a>(&self)  -> bool { false }
    fn content<'a>(&self, post: &'a Post) -> &'a str {""}
}



struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn can_add_text<'a>(&self) -> bool {
        true
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> { self }
}


struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(ConfirmReview {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}


struct ConfirmReview {}

impl State for ConfirmReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}


struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> { self }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn reject(self: Box<Self>) -> Box<dyn State> { self }
}


pub struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, s: &str) {
        if self.state.as_ref().unwrap().can_add_text() {
            self.content.push_str(s);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_state_pattern_blog() {
        let mut post = Post::new();

        post.add_text("I ate salad for a lunch today");
        assert_eq!("", post.content());
    
        post.request_review();
        assert_eq!("", post.content());
    
        post.reject();
        post.request_review();
        post.add_text("shit");
        assert_eq!("", post.content());
    
        post.approve();
        assert_eq!("", post.content());
    
        post.approve();
        assert_eq!("I ate salad for a lunch today", post.content());
    
        println!("{}", post.content());
    }
}