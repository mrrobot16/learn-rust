mod lib;
use crate::lib::{Summary, Tweet, NewsArticle};

fn main() {
    println!("Hello, world of generics!");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Of course, as you probable already know, people"),
        reply: false,
        retweet: false,
    };
    
    let news_article = NewsArticle {
        headline: String::from("NewsArticle Headline"),
        location: String::from("NewsArticle Location"),
        author: String::from("NewsArticle Author"),
        content: String::from("NewsArticle Content"),
    };
    
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", news_article.summarize());
}