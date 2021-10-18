pub mod coffe_machine {
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

    pub fn process_order(order: CustomerOrder) -> String {
        "".to_owned()
    }
}