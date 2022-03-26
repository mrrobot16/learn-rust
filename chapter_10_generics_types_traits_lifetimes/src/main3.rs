mod lib;
use crate::lib::{Summary, Tweet, NewsArticle, notify};

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
    
    let news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", news_article.summarize());
    notify(&news_article, &news_article);
}