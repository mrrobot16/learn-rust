pub struct NewsArticle {
    pub headline: String,
    pub location: String, 
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("NewsArticle: {}, by {}, {}", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    
    fn summarize(&self) -> String {
        format!("Tweet: {}: {}", self.username, self.content)
    }
    

}

pub trait Summary {
    fn summarize_author(&self) -> String {
        format!("Author of article: not found")
    }
    
    fn summarize(&self) -> String {
        // default implentation if any "impl" does add the summarize() method.
        // String::from("Read more...")
        format!("Read more from {}...", self.summarize_author())
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
}