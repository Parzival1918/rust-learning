use std::str::FromStr;
use rand::{thread_rng, Rng};
use dialoguer::{Select, Input, Confirm};
use dialoguer::theme::ColorfulTheme;
use std::fmt;

#[allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        match self {
            Self::Penny => write!(f, "Penny"),
            Self::Nickel => write!(f, "Nickel"),
            Self::Dime => write!(f, "Dime"),
            Self::Quarter => write!(f, "Quarter"),
        }
    }
}

#[allow(unused)]
impl Coin {
    fn value(&self) -> u32 {
        match self {
            Self::Penny => 1,
            Self::Nickel => 5,
            Self::Dime => 10,
            Self::Quarter => 25
        }
    }
}

#[derive(Debug)]
struct OverCapacity;

#[allow(unused)]
struct Purse {
    coins: Vec<Coin>,
    capacity: u8,
    owner: String
}

impl Purse {
    fn new(owner: &str) -> Self {
        Self {
            coins: vec![],
            capacity: thread_rng().gen_range(5..=15u8),
            owner: String::from_str(owner).unwrap()
        }
    }

    fn add_coin(&mut self, coin: Coin) -> Result<(), OverCapacity> {
        if self.coin_count() == self.capacity.into() {
            return Err(OverCapacity)
        }

        self.coins.push(coin);
        Ok(())
    } 

    /// The number of coins in the purse 
    fn coin_count(&self) -> usize {
        self.coins.len()
    }

    fn count_each(&self) -> Vec<u8> {
        let mut counter = vec![0u8; 4];
        for coin in &self.coins {
            match coin {
                Coin::Penny => counter[0] += 1,
                Coin::Nickel => counter[1] += 1,
                Coin::Dime => counter[2] += 1,
                Coin::Quarter => counter[3] += 1
            }
        }

        counter
    }

    fn total_value(&self) -> u32 {
        let counter = self.coins.iter().map(|c| c.value()).sum();

        counter
    }
}

fn main() {
    let mut my_purse = Purse::new("Pedro");

    let coins = vec![Coin::Dime, Coin::Nickel, Coin::Quarter, 
                                Coin::Quarter, Coin::Penny, Coin::Dime, 
                                Coin::Nickel, Coin::Quarter, Coin::Quarter, 
                                Coin::Quarter, Coin::Penny];

    for coin in coins {
        match my_purse.add_coin(coin) {
            Ok(_) => continue,
            Err(err) => println!("Error adding coin: {:?}", err)
        }
    }

    println!("Number of coins in my purse: {}", my_purse.coin_count());
    println!("Amount of coins of each type: {:?}", my_purse.count_each());
    println!("Total value of the coins is: {}", my_purse.total_value());

    let opts = vec!["Contents", "Add coins", "Exit"];

    let owner: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("Your name?").interact_text().unwrap();
    let mut purse = Purse::new(&owner);

    loop {
        let choice = Select::with_theme(&ColorfulTheme::default()).with_prompt("Choose one").default(0)
            .items(&opts).interact().unwrap();

        match opts[choice] {
            "Contents" => show_contents(&purse),
            "Add coins" => add_coins(&mut purse),
            "Exit" => {
                let confirm = Confirm::with_theme(&ColorfulTheme::default()).with_prompt("Are you sure?").default(true)
                    .interact().unwrap();

                if confirm {
                    break
                }
            }
            _ => break
        }
    }
}

fn show_contents(purse: &Purse) {
    println!(" - There are {} coins in the purse", purse.coin_count());
    println!(" - The total amounts to {} cents", purse.total_value());
    println!(" - The maximum amount of coins you can hold is {}", purse.capacity);
}

fn add_coins(purse: &mut Purse) {
    let coins = vec![Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    let idx = Select::with_theme(&ColorfulTheme::default()).with_prompt("Which coin are you adding?").default(0)
            .items(&coins).interact().unwrap();

    let coin: Coin;
    match idx {
        0 => coin = Coin::Penny,
        1 => coin = Coin::Nickel,
        2 => coin = Coin::Dime,
        3 => coin = Coin::Quarter,
        _ => coin = Coin::Penny,
    }
    
    match purse.add_coin(coin) {
        Ok(_) => println!("Coin added!"),
        Err(_) => println!("Limit of purse reached! Coin not added")
    }
}