pub mod coffee_machine {

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

    pub fn process_message(message: String) -> String {
        format!("M:{}", message)
    }

    pub fn process_order_with_money(order: CustomerOrder, money: f32) -> String {
        "".to_owned()
    }

    // Private Methods
    pub fn process_order(order: CustomerOrder) -> String {
        let drink_code = get_drink_code(order.drink_type);
        let sugars = get_sugars(order.sugars);
        let spoon = get_spoon(order.sugars);

        format!("{}:{}:{}", drink_code, sugars, spoon)
    }

    fn get_drink_code(drink_type: DrinkType) -> &'static str {
        match drink_type {
            DrinkType::Tea => "T",
            DrinkType::Coffee => "C",
            DrinkType::Chocolate => "H"
        }
    }

    fn get_sugars(sugars: u8) -> String {
        match sugars {
            0 => "".to_owned(),
            _ => format!("{}", sugars)
        }
    }

    fn get_spoon(sugars: u8) -> &'static str {
        match sugars {
            0 => "",
            _ => "0"
        }
    }

}