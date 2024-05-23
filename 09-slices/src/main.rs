#![allow(dead_code, unused_variables)]

fn main() {
    // slices are references to a contiguous sequence of elements in a collection
    let tweet = String::from("This is my tweet and it's ver very long");

    // slice to the first 20 position of string tweet
    let trimmed_tweet = &tweet[..20];
    println!("trimmed_tweet (first 20 chars): {}", trimmed_tweet);

    let trimmed_tweet = &tweet[5..];
    println!("trimmed_tweet (from char 5): {}", trimmed_tweet);

    let trimmed_tweet = &tweet[..];

    // String is growable, head allocated and UTF-8 encoded
    // str (/ster/) is immutable, UTF-8 encoded allocatef in stack, heap, or static memory, and
    // handled behind a reference (&str) because length is unknown at compile time
    // &str is an immutable view (pointer) to a string (or a subset of it)

    let s = "my string"; // literals are slices and are always stored in the binary file

    let tweet2 = "This is my tweet and it's ver very long";

    let trimmed_tweet = trim_tweet(&tweet); // call function with a String
    //let trimmed_tweet2 = trim_tweet(tweet2); // call function with a &str <-- it does not compile

    println!("trimmed_tweet: {}", trimmed_tweet);

    // trim_tweet_all will be able to handle both types
    let trimmed_tweet = trim_tweet_all(tweet2); // call function with a &str
    let trimmed_tweet2 = trim_tweet_all(&tweet); // call function with a String <-- it works because of deref coercion (&String is converted to &str)

    println!("trimmed_tweet: {}", trimmed_tweet);
    println!("trimmed_tweet2: {}", trimmed_tweet2);

    // slices also works with any other type of collection
    let a = [1, 2, 3, 4, 5, 6];
    let a_slice = &a[1..3]; // slice to positions 1 and 2
    println!("a_slice: {:?}", a_slice); // :? format is to print the var with debug format

    fn trim_tweet(tweet: &String) -> &str {
        &tweet[..20]
    }

    // &str params are able to handle &str and also &String types
    fn trim_tweet_all(tweet: &str) -> &str {
        &tweet[..20]
    }
}
