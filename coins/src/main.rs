use std::str::FromStr;

#[allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
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
            capacity: 10u8,
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
}
