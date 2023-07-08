// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar` for the type `String`.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a hint.

use std::fmt::format;

pub trait Summary {
    // "Each type implementing this trait must provide its
    // own custom behavior for the body of the method"
    fn summarize(&self) -> String;

    fn i_aunno(&self) {
        println!(
            "this is what's known as a default implementation. \
        it does the same thing for every type associated with this trait"
        )
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// NewsArticle is a type that implements the Summary trait.
// NewsArticle needs to provide a `summary` method that matches
// the method signature contained in the Summary trait.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// impl TraitName for Type
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(mut self) -> Self {
        self.push_str("Bar");
        self
    }
}

fn main() {
    let tweet = Tweet {
        username: "alexan_dev".to_string(),
        content: "cool tweet bruh".to_string(),
        reply: true,
        retweet: true,
    };
    let article = NewsArticle {
        headline: "headline".to_string(),
        location: "chicago".to_string(),
        author: "x".to_string(),
        content: "a bunch of content here... ".to_string(),
    };

    format!("{:?}", tweet.i_aunno());
    format!("{:?}", article.i_aunno());
    println!();
    println!("🐣 new tweet: {}", tweet.summarize());
    println!();
    println!("new article: {}", article.summarize());
    println!();

    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }

    #[test]
    fn negative_batman() {
        main();
        assert_eq!(12, 12)
    }
}
