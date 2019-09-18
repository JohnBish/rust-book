pub trait Summary { // Traits are similar to interfaces in other languages
    // fn summarize(&self) -> String;

    fn summarize(&self) -> String { // Here, we use a default summarize method to be overridden if desired
        String::from("Read more")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Stub {}

impl Summary for Stub {}

pub fn notify(item: impl Summary) { // Short for pub fn notify<T: Summary>(item: T)
    println!("Breaking news! {}", item.summarize());
}

// If we wanted notify to accept types which implement Summary and Display:
// pub fn notify(item: impl Summary + Display)
// OR
// pub fn notify<T: Summary + Display>(item: T)

// where clause: instead of
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
// We could write:
// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// NOT ALLOWED to return multiple types that implement the same trait:
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from("The Pittsburgh Penguins once again are the best
//             hockey team in the NHL."),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
