use std::collections::HashMap;
use rand::Rng;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Card {
    name: String,
    _description: String,
    _rarity: Rarity,
    base_price: u32,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Rarity {
    Common,
    Rare,
    Legendary,
}

struct Inventory {
    cards: HashMap<String, (u32, Card)>,
}

struct Shop {
    name: String,
    inventory: Inventory,
    cash: u32,
    level: u32,
    reputation: u32,
}

impl Shop {
    fn buy_cards(&mut self, distributor: &Distributor) {
        println!("Available cards:");
        for (i, card) in distributor.cards.iter().enumerate() {
            println!("{}. {} - ${}", i + 1, card.name, card.base_price);
        }

        println!("Which card would you like to buy? (Enter the number)");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: usize = choice.trim().parse().expect("Please type a number!");
        if choice > 0 && choice <= distributor.cards.len() {
            let card = &distributor.cards[choice - 1];
            if self.cash >= card.base_price {
                self.inventory
                    .cards
                    .entry(card.name.clone())
                    .or_insert((0, card.clone()));
                let (quantity, _) = self.inventory.cards.get_mut(&card.name).unwrap();
                *quantity += 1;
                self.cash -= card.base_price;
                println!("Bought {} for ${}", card.name, card.base_price);
            } else {
                println!("Not enough cash to buy this card!");
            }
        } else {
            println!("Invalid choice, please try again");
        }
    }

    fn set_card_price(&mut self) {
        println!("Which card would you like to reprice? Enter the card name:");
        let mut card_name = String::new();
        std::io::stdin()
            .read_line(&mut card_name)
            .expect("Failed to read line");
        let card_name = card_name.trim();
        if let Some((quantity, price)) = self.inventory.cards.get_mut(card_name) {
            loop {
                println!(
                    "Enter the new price for {} (current price: ${})",
                    card_name, price.base_price
                );
                let mut new_price = String::new();
                std::io::stdin()
                    .read_line(&mut new_price)
                    .expect("Failed to read line");
                let new_price: u32 = match new_price.trim().parse() {
                    Ok(price) => price,
                    Err(_) => {
                        println!("Please enter a valid number.");
                        continue;
                    }
                };
                if new_price > 1000 {
                    println!("The maximum price for a card is $1000. Please enter a lower price.");
                } else {
                    price.base_price = new_price;
                    println!(
                        "The price of {} is now ${}. Quantity in stock: {}",
                        card_name, price.base_price, quantity
                    );
                    break;
                }
            }
        } else {
            println!("Card not found in inventory: {}", card_name);
        }
    }
    fn sell_cards(&mut self) {
        let customer = Customer::new();
        println!(
            "{} enters the shop with a budget of ${}.",
            customer.name, customer.budget
        );

        // Clone the cards to avoid borrowing issues
        let cards_clone: Vec<(String, (u32, Card))> = self.inventory.cards.clone().into_iter().collect();

        let mut affordable_cards: Vec<(&String, &Card)> = cards_clone
            .iter()
            .filter(|(_, (_, card))| card.base_price <= customer.budget)
            .map(|(name, (_, card))| (name, card))
            .collect();

        if affordable_cards.is_empty() {
            println!(
                "{} can't afford any of the cards in the shop and leaves disappointed.",
                customer.name
            );
            return;
        }

        affordable_cards.sort_by(|(_, price1), (_, price2)| price2.base_price.cmp(&price1.base_price));

        let preferred_cards: Vec<(&String, &Card)> = affordable_cards
            .clone()
            .into_iter()
            // Corrected logic, removed unnecessary .get_key_value()
            .filter(|(name, card)| customer.preferences.contains(&card._rarity))
            .collect();

        let card_to_buy = if !preferred_cards.is_empty() {
            preferred_cards[0]
        } else {
            affordable_cards[0]
        };

        // Now we can safely modify the original HashMap
        let (quantity, _) = self.inventory.cards.get_mut(card_to_buy.0).unwrap();
        *quantity -= 1;
        self.cash += card_to_buy.1.base_price;
        println!(
            "{} bought {} for ${}!",
            customer.name, card_to_buy.0, card_to_buy.1.base_price
        );
        if *quantity == 0 {
            self.inventory.cards.remove(card_to_buy.0);
        }
    }
}

struct Distributor {
    cards: Vec<Card>,
}

impl Distributor {
    fn generate_card() -> Card {
        let rarity = match rand::thread_rng().gen_range(0..100) {
            0..=50 => Rarity::Common,
            51..=80 => Rarity::Rare,
            _ => Rarity::Legendary,
        };
        let base_price = match rarity {
            Rarity::Common => 10,
            Rarity::Rare => 50,
            Rarity::Legendary => 100,
        };
        Card {
            name: format!("Card {}", rand::thread_rng().gen_range(1..=1000)),
            _description: "A random card".to_string(),
            _rarity: rarity,
            base_price,
        }
    }

    fn new() -> Distributor {
        let mut cards = Vec::new();
        for _ in 0..10 {
            cards.push(Distributor::generate_card());
        }
        Distributor { cards }
    }
}

struct Customer {
    name: String,
    budget: u32,
    preferences: Vec<Rarity>,
}

impl Customer {
    fn new() -> Customer {
        let mut rng = rand::thread_rng();
        let name = format!("Customer {}", rng.gen_range(1..1000));

        let budget = rng.gen_range(10..=200);

        let mut preferences = Vec::new();
        for _ in 0..rng.gen_range(1..=3) {
            let rarity = match rng.gen_range(0..100) {
                0..=60 => Rarity::Common,
                61..=90 => Rarity::Rare,
                _ => Rarity::Legendary,
            };
            preferences.push(rarity);
        }

        Customer {
            name,
            budget,
            preferences,
        }
    }
}

fn main() {
     println!(
        r#"
         ________                                 __                      __        ________  ______    ______  
        /        |                               /  |                    /  |      /        |/      \  /      \ 
        $$$$$$$$/______    ______   _____  ____  $$/  _______    ______  $$ |      $$$$$$$$//$$$$$$  |/$$$$$$  |
           $$ | /      \  /      \ /     \/    \ /  |/       \  /      \ $$ |         $$ |  $$ |  $$/ $$ | _$$/ 
           $$ |/$$$$$$  |/$$$$$$  |$$$$$$ $$$$  |$$ |$$$$$$$  | $$$$$$  |$$ |         $$ |  $$ |      $$ |/    |
           $$ |$$    $$ |$$ |  $$/ $$ | $$ | $$ |$$ |$$ |  $$ | /    $$ |$$ |         $$ |  $$ |   __ $$ |$$$$ |
           $$ |$$$$$$$$/ $$ |      $$ | $$ | $$ |$$ |$$ |  $$ |/$$$$$$$ |$$ |         $$ |  $$ \__/  |$$ \__$$ |
           $$ |$$       |$$ |      $$ | $$ | $$ |$$ |$$ |  $$ |$$    $$ |$$ |         $$ |  $$    $$/ $$    $$/ 
           $$/  $$$$$$$/ $$/       $$/  $$/  $$/ $$/ $$/   $$/  $$$$$$$/ $$/          $$/    $$$$$$/   $$$$$$/  
        "#

    );

    let mut shop = Shop {
        name: "My Card Shop".to_string(),
        inventory: Inventory {
            cards: HashMap::new(),
        },
        cash: 1000,
        level: 1,
        reputation: 0,
    };
    loop {
        println!("Welcome to {}!", shop.name);
        println!("Cash: ${}", shop.cash);
        println!("Level: {}", shop.level);
        println!("Reputation: {}", shop.reputation);
        println!("Inventory:");
        for (card, (quantity, price)) in &shop.inventory.cards {
            println!(
                "  - {} (Qty: {}, Price: ${})",
                card, quantity, price.base_price
            );
        }

        println!("\nWhat would you like to do?");
        println!("1. Buy Cards");
        println!("2. Set Prices");
        println!("3. Sell Cards");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number!");

        match choice {
            1 => {
                let distributor = Distributor::new();
                shop.buy_cards(&distributor);
            }
            2 => shop.set_card_price(),
            3 => shop.sell_cards(),
            4 => {
                println!("Thanks for playing!");
                break;
            }
            _ => println!("Invalid choice, please try again"),
        }
    }
}
