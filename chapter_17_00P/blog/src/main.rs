use blog::{Post, PendingReviewPost};

fn main() {
    println!("Hello, world! of OOP");
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    
    let post = post.request_review();
    
    let post = post.approve();
    // assert_eq!("", post.content());

    // post.request_review();
    // assert_eq!("", post.content());

    // post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

// TODO: Add the below suggestions.

// - Add a reject method that changes the post’s state from PendingReview back to Draft.
// - Require two calls to approve before the state can be changed to Published.
// - Allow users to add text content only when a post is in the Draft state. Hint: have the state object responsible 
// for what might change about the content but not responsible for modifying the Post.