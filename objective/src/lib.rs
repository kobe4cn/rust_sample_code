trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn publish(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&'a self, post: &'a Post) -> &'a str;
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

pub struct Draft;
pub struct PendingReview;
pub struct Published;
pub struct Rejected;
pub struct Approved;

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Rejected {})
    }
    // PendingReview 不能直接 publish，必须先 approve
    fn publish(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Approved {})
    }
    // PendingReview 状态不能显示内容
    fn content<'a>(&'a self, _post: &'a Post) -> &'a str {
        ""
    }
}

impl State for Approved {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Approved 可以 publish 到 Published
    fn publish(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Rejected {})
    }
    // Approved 状态不能显示内容，只有 Published 可以
    fn content<'a>(&'a self, _post: &'a Post) -> &'a str {
        ""
    }
}
impl State for Rejected {
    // Rejected 可以重新 request_review 回到 PendingReview
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    // Rejected 不能直接 approve，必须先 request_review
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Rejected 不能直接 publish，必须先 request_review 然后 approve
    fn publish(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Rejected 状态不能显示内容
    fn content<'a>(&'a self, _post: &'a Post) -> &'a str {
        ""
    }
}
impl State for Published {
    // Published 是最终状态，不能再转换
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Published 不能再 approve（不能跨节点流转）
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn publish(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // 只有 Published 状态可以显示内容
    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        &post.content
    }
}

impl State for Draft {
    // Draft 可以 request_review 到 PendingReview
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    // Draft 不能直接 reject，必须先 request_review
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Draft 不能直接 publish，必须先 request_review 然后 approve
    fn publish(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Draft 不能直接 approve，必须先 request_review
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Draft 状态不能显示内容
    fn content<'a>(&'a self, _post: &'a Post) -> &'a str {
        ""
    }
}

impl Post {
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        if let Some(state) = self.state.as_ref() {
            state.content(self)
        } else {
            ""
        }
    }
    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
    pub fn reject(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.reject());
        }
    }
    pub fn publish(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.publish());
        }
    }
}
impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        // Draft 状态不能显示内容
        assert_eq!(post.content(), "");
        // Draft -> PendingReview
        post.request_review();
        // PendingReview 状态不能显示内容
        assert_eq!(post.content(), "");
        // PendingReview -> Rejected
        post.reject();
        // Rejected 状态不能显示内容
        assert_eq!(post.content(), "");
        // Rejected -> PendingReview
        post.request_review();
        assert_eq!(post.content(), "");
        // PendingReview -> Approved
        post.approve();
        // Approved 状态不能显示内容
        assert_eq!(post.content(), "");
        // Approved -> Published
        post.publish();
        // 只有 Published 状态可以显示内容
        assert_eq!(post.content(), "I ate a salad for lunch today");
        println!("content: {}", post.content());
    }
}
