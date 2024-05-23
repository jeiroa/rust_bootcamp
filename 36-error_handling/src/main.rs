#![allow(dead_code)]
use std::{collections::HashMap, error::Error, fmt::{Debug, Display}, io, num::ParseIntError};

struct ParsePaymentInfoError {
    source: Option<Box<dyn Error>>,
    msg: String,
}

impl Debug for ParsePaymentInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // print the error
        write!(f, "{self} \n\t{}", self.msg)?;

        // and also the cause if there is one
        if let Some(e) = self.source.as_ref() {
            write!(f, "\n\nCaused by:\n\t{e:?}")?;
        }

        Ok(())
    }
}

// trait to provide automatic parsing from ParseIntError to ParsePaymentInfoError when using ?
// this method is not actually used in this program
impl From<ParseIntError> for ParsePaymentInfoError {
    fn from(e: ParseIntError) -> Self {
        ParsePaymentInfoError {
            source: Some(Box::new(e)),
            msg: String::from("") // there is not enough context to set this message
        }
    }
}

impl Display for ParsePaymentInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Parsing payment error: invalid payment info")
    }
}

impl Error for ParsePaymentInfoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split(" ")
        .map(|s| {
            s.parse() // remember this method returns ParseIntError in case of Error
                .map_err(|_| { // map the error to provide a custom message
                    ParsePaymentInfoError {
                        source: None,
                        msg: format!("{s:?} could not be parsed as u32")
                    }
                })
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| {
            ParsePaymentInfoError {
                source: Some(Box::new(e)),
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

// User Input Error
enum CreditCardError {
    InvalidInput(String),
    Other(Box<dyn Error>, String)
}

impl Debug for CreditCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // {self} displays the message from Display implementation below
            Self::InvalidInput(msg) => write!(f, "{self}\n{msg}"),
            Self::Other(e, msg) => write!(f, "{self}\n{msg}\n\nCaused by:\n\t{e:?}"),
        }
    }
}

impl Display for CreditCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Credit card error: Could not retrieve credit card.")
    }
}

impl Error for CreditCardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CreditCardError::InvalidInput(_) => None,
            CreditCardError::Other(e, _) => {
                Some(e.as_ref()) // because of the size error if as_ref is not used (return dyn Error instead of the actual Box<dyn Error>)
            }
        }
    }
}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    // this call should return a user error
    let card_string = credit_cards.get(name)
        .ok_or(CreditCardError::InvalidInput(format!("No credit card was found for {name}.")))?;

    let card = parse_card(card_string)
        .map_err(|e| {
            CreditCardError::Other(Box::new(e), format!("{name}'s card could not be parsed."))
    })?;

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
                CreditCardError::Other(_, _) => println!("Something went wrong! Try again!") // hide the actual error to the user
            }

            // log the actual error for debugging purposes
            log::error!("{err:?}");
        },
    }
}
