pub mod coffee_machine {
    use crate::price_manager::price_manager::{check_missing_money_for_product, DRINK_PRICES};
    use enum_iterator::IntoEnumIterator;
    use std::collections::HashMap;
    use std::fmt;

    #[derive(Copy, Clone, Eq, Hash, IntoEnumIterator, PartialEq)]
    pub enum DrinkType {
        Tea,
        Coffee,
        Chocolate,
        OrangeJuice
    }

    impl fmt::Display for DrinkType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                DrinkType::Tea => write!(f, "Tea"),
                DrinkType::Coffee => write!(f, "Coffee"),
                DrinkType::Chocolate => write!(f, "Chocolate"),
                DrinkType::OrangeJuice => write!(f, "OrangeJuice"),
            }
        }
    }

    pub struct CustomerOrder {
        drink_type: DrinkType,
        sugars: u8,
        extra_hot: bool
    }

    pub struct CoffeeMachine {
        pub sold_drinks: HashMap<DrinkType, u32>,
        pub money_earned: f32
    }

    impl CustomerOrder {
        pub fn new(drink_type: DrinkType) -> CustomerOrder {
            CustomerOrder { drink_type, sugars: 0, extra_hot: false }
        }

        pub fn with_sugar(mut self, sugar: u8) -> CustomerOrder {
            self.sugars = sugar;
            self
        }

        pub fn extra_hot(mut self) -> CustomerOrder {
            self.extra_hot = true;
            self
        }
    }

    impl CoffeeMachine {

        pub fn new() -> CoffeeMachine {
            CoffeeMachine { sold_drinks: HashMap::new(), money_earned: 0.0 }
        }

        pub fn process_order_with_money(&mut self, order: CustomerOrder, money: f32) -> String {
            let missing_money = check_missing_money_for_product(order.drink_type, money);

            if missing_money > 0.0 {
                process_message(format!("Missing {} euros", missing_money))
            } else {
                self.process_order(order)
            }
        }

        pub fn process_order(&mut self, order: CustomerOrder) -> String {
            let drink_code = get_drink_code(&order);
            let sugars = get_sugars_code(order.sugars);
            let spoon = get_spoon_code(order.sugars);

            self.register_order(order);
            format!("{}:{}:{}", drink_code, sugars, spoon)
        }

        pub fn get_total_money_sold(&self) -> f32 {
            self.money_earned
        }

        pub fn get_total_amount_sold(&self, drink_type:DrinkType) -> u32 {
            self.sold_drinks.get(&drink_type).unwrap_or(&0).clone()
        }

        fn register_order(&mut self, order: CustomerOrder) {
            let price = DRINK_PRICES.get(&order.drink_type).unwrap_or(&0.0);
            self.money_earned = self.money_earned + price;

            let amount_sold = self.sold_drinks.get(&order.drink_type).unwrap_or(&0);
            self.sold_drinks.insert(order.drink_type, amount_sold + 1);
        }

    }

    pub fn process_message(message: String) -> String {
        format!("M:{}", message)
    }

    // Private Methods
    fn get_drink_code(order: &CustomerOrder) -> String {
        let drink_code = match order.drink_type {
            DrinkType::Tea => "T",
            DrinkType::Coffee => "C",
            DrinkType::Chocolate => "H",
            DrinkType::OrangeJuice => "O"
        };

        format!("{}{}", drink_code, get_extra_hot_code(order.extra_hot))
    }

    fn get_extra_hot_code(extra_hot: bool) -> &'static str {
        match extra_hot {
            true => "h",
            false => ""
        }
    }

    fn get_sugars_code(sugars: u8) -> String {
        match sugars {
            0 => "".to_owned(),
            _ => format!("{}", sugars)
        }
    }

    fn get_spoon_code(sugars: u8) -> &'static str {
        match sugars {
            0 => "",
            _ => "0"
        }
    }

}