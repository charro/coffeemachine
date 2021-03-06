pub mod price_manager {
    use std::collections::HashMap;
    use lazy_static::lazy_static;
    use crate::coffe_machine::coffee_machine::{DrinkType};

    lazy_static!{
        pub static ref DRINK_PRICES: HashMap<DrinkType, f32> = [
            (DrinkType::Tea, 0.4),
            (DrinkType::Coffee, 0.6),
            (DrinkType::Chocolate, 0.5),
            (DrinkType::OrangeJuice, 0.6),
        ].iter().copied().collect();
    }

    pub fn check_missing_money_for_product(drink_type: DrinkType, money: f32) -> f32 {
        let price = DRINK_PRICES[&drink_type];
        let missing_money = price - money;

        if missing_money > 0.0 {
            return missing_money;
        }
        else {
            return 0.0;
        }
    }

}