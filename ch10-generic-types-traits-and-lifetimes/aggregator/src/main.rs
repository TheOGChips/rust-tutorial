pub trait Summary {
    fn summarize_author (&self) -> String;
    //fn summarize (&self) -> String;
    fn summarize (&self) -> String {
        return format!("Read more from {}...", self.summarize_author());
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author (&self) -> String {
        return format!("Placeholder author summary...");
    }
    /*fn summarize (&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
    }*/
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    /*fn summarize (&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }*/
    fn summarize_author (&self) -> String {
        return format!("@{}", self.username);
    }
}

//pub fn notify (item: &impl Summary) {
pub fn notify<T: Summary> (item: &T) {
//pub fn notify (item1: &impl Summary, item2: &impl Summary) {  //NOTE: Types can be mixed
//pub fn notify<T: Summary> (item1: &T, item2: &T) {    //NOTE: Types must be the same
//pub fn notify (item: &(impl Summary + Display)) {     //NOTE: Double trait bound
//pub fn notify<T: Summary + Display> (item: &T) {      //NOTE: Double trait bound
//pub fn notify<T> (item: &T) where T: Summary + Display {  //NOTE: Using a where clause for traits
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable () -> impl Summary {
    return Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
}

/* NOTE: Example blanket implementation along with trait generics
 * impl<T: Display> ToString for T {}
 */

/* NOTE: Would bring into scope like this:
 *       use aggregator::{Summary, NewsArticle, Tweet};
 */
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the \
                               NHL."),
    };
    println!("New article available! {}", article.summarize());
}
