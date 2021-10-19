pub mod coffee_machine {
    use crate::price_manager::price_manager::check_missing_money_for_product;

    #[derive(Copy, Clone, Eq, Hash, PartialEq)]
    pub enum DrinkType {
        Tea,
        Coffee,
        Chocolate
    }

    pub struct CustomerOrder {
        drink_type: DrinkType,
        sugars: u8
    }

    impl CustomerOrder {
        pub fn new(drink_type: DrinkType, sugars: u8) -> CustomerOrder {
            CustomerOrder { drink_type, sugars }
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
        let drink_code = get_drink_code(order.drink_type);
        let sugars = get_sugars_code(order.sugars);
        let spoon = get_spoon_code(order.sugars);

        format!("{}:{}:{}", drink_code, sugars, spoon)
    }

    pub fn process_message(message: String) -> String {
        format!("M:{}", message)
    }

    // Private Methods
    fn get_drink_code(drink_type: DrinkType) -> &'static str {
        match drink_type {
            DrinkType::Tea => "T",
            DrinkType::Coffee => "C",
            DrinkType::Chocolate => "H"
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