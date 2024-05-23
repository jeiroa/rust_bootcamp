#![allow(dead_code)]
use std::{collections::HashMap, fmt::Debug, io};

use anyhow::Context;
use thiserror::Error;

#[derive(Debug, Error)] // needed for thiserror
#[error("{msg}")] // provides a Display implementation providing the error message (content of field msg in this case)
struct ParsePaymentInfoError {
    source: Option<anyhow::Error>, // errors are now of this type (no Box pointer is necessary anymore)
    msg: String,
}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split(" ")
        .map(|s| {
            s.parse() // remember this method returns ParseIntError in case of Error
                .with_context(|| format!("{s:?} could not be parsed as u32")) // convert an Error into an anyhow::Error with that message
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| {
            ParsePaymentInfoError {
                source: Some(e),
                msg: format!("Failed to parse input as numbers. Input: {card}")
            }
        })?; // if map_err were not used, this ? would call the From<ParseIntError> trait to return a ParsePaymentInfoError
        //.unwrap(); // get rid of unwrap and switch it for ?

    Ok(numbers)
}

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(ParsePaymentInfoError {
            source: None,
            msg: format!("Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}")
        })
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap(); 

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv
    })
}

#[derive(Debug, Error)] // needed for thiserror
enum CreditCardError {
    #[error("{0}")] // message will be the first param of annotated enum below
    InvalidInput(String),
    #[error(transparent)] // message will be the source error provided by "from" macro below
    Other(#[from] anyhow::Error),
}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    // this call should return a user error
    let card_string = credit_cards.get(name)
        .ok_or(CreditCardError::InvalidInput(format!("No credit card was found for {name}.")))?;

    let card = parse_card(card_string)
        .with_context(|| format!("{name}'s card could not be parsed."))
        .map_err(|e| {
            CreditCardError::Other(e)
        }
    )?;

    Ok(card)
}

fn main() {
    env_logger::init(); // initialize logger
    // to set logger use RUST_LOG=<level> cargo run
    // level can be error, warn, info, debug, trace, off

    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27"),
        ("Bob", "1234567 Dec 27 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(err) => {
            match &err {
                CreditCardError::InvalidInput(msg) => println!("{msg}"),
                CreditCardError::Other(_) => println!("\nSomething went wrong! Try again!") // hide the actual error to the user
            }

            // log the actual error for debugging purposes
            log::error!("\n{err:?}");
        }
    }
}
