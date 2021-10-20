pub mod coffee_machine {
    use crate::price_manager::price_manager::check_missing_money_for_product;

    #[derive(Copy, Clone, Eq, Hash, PartialEq)]
    pub enum DrinkType {
        Tea,
        Coffee,
        Chocolate,
        OrangeJuice
    }

    pub struct CustomerOrder {
        drink_type: DrinkType,
        sugars: u8,
        extra_hot: bool
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

    pub fn process_order_with_money(order: CustomerOrder, money: f32) -> String {
        let missing_money = check_missing_money_for_product(order.drink_type, money);

        if missing_money > 0.0 {
            process_message(format!("Missing {} euros", missing_money))
        }
        else {
            process_order(order)
        }
    }

    pub fn process_order(order: CustomerOrder) -> String {
        let drink_code = get_drink_code(&order);
        let sugars = get_sugars_code(order.sugars);
        let spoon = get_spoon_code(order.sugars);

        format!("{}:{}:{}", drink_code, sugars, spoon)
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