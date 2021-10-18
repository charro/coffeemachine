#[cfg(test)]
mod tests{
    use crate::coffe_machine::coffee_machine::{CustomerOrder, DrinkType, process_message, process_order};

    #[test]
    fn test_coffee_no_sugar() {
        assert_eq!("C::", process_order(CustomerOrder::new(DrinkType::Coffee, 0)));
    }

    #[test]
    fn test_tea_one_sugar() {
        assert_eq!("T:1:0", process_order(CustomerOrder::new(DrinkType::Tea, 1)));
    }

    #[test]
    fn test_chocolate_two_sugars() {
        assert_eq!("H:2:0", process_order(CustomerOrder::new(DrinkType::Chocolate, 2)));
    }

    #[test]
    fn test_user_message() {
        assert_eq!("M:my own shiny message", process_message("my own shiny message".to_owned()));
    }
}