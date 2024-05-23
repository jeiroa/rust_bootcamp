fn main() {
    let username = get_username(1);
    // to check Option results we should use a match expression
    match &username { // using & because we want to use username later
        Option::Some(name) => println!("{name}"),
        Option::None => {} // do nothing
    }
    
    // it is also possible to check the result with an if
    // assign the content of username into name and if there was any, true is resolved and name can be used
    if let Some(name) = username { // as said, Option is not necessary
        println!("{name}")
    }

    let username_db = get_username_db(1);
    if let Some(name) = username_db {
        println!("{name}")
    }
}

// use Option to return results that are "empty" (there is no null in Rust)
fn get_username(user_id: u32) -> Option<String> {
    // get username from database
    let db_result = String::from("Ferris");
    if user_id == 1 {
        Option::Some(db_result) // Option is not necessary to be set, Some(db_result) is enough
    } else {
        Option::None
    }
}

fn get_username_db(user_id: u32) -> Option<String> {
    let query = format!("GET username FROM users WHERE id={user_id}");
    let db_result = query_db(query);
    // handy method of Result to return Some if Result::Ok and None if Result::Err
    db_result.ok()
}

// Result enum defines the type for the valid result and the error
fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query string is empty"))
    } else {
        Ok(String::from("Ferris DB"))
    }
}