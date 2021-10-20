#[cfg(test)]
mod tests{
    use approx::assert_relative_eq;

    use crate::coffe_machine::coffee_machine::{CustomerOrder, DrinkType, process_message, process_order, process_order_with_money};
    use crate::price_manager::price_manager::check_missing_money_for_product;

    //Coffee Machine: Basic Orders
    #[test]
    fn test_coffee_no_sugar() {
        assert_eq!("C::", process_order(CustomerOrder::new(DrinkType::Coffee).with_sugar(0)));
    }

    #[test]
    fn test_tea_one_sugar() {
        assert_eq!("T:1:0", process_order(CustomerOrder::new(DrinkType::Tea).with_sugar(1)));
    }

    #[test]
    fn test_chocolate_two_sugars() {
        assert_eq!("H:2:0", process_order(CustomerOrder::new(DrinkType::Chocolate).with_sugar(2)));
    }

    #[test]
    fn test_user_message() {
        assert_eq!("M:my own shiny message", process_message("my own shiny message".to_owned()));
    }

    #[test]
    fn test_orange_juice() {
        assert_eq!("O::", process_order(CustomerOrder::new(DrinkType::OrangeJuice).with_sugar(0)));
    }

    // Coffee Machine: Orders with money
    #[test]
    fn test_accept_order_with_enough_money() {
        assert_eq!("H:2:0", process_order_with_money(CustomerOrder::new(DrinkType::Chocolate).with_sugar(2), 0.7));
    }

    #[test]
    fn test_accept_order_with_exact_money() {
        assert_eq!("H:2:0", process_order_with_money(CustomerOrder::new(DrinkType::Chocolate).with_sugar(2), 0.5));
    }

    #[test]
    fn test_reject_order_with_not_enough_money() {
        assert_eq!("M:Missing 0.4 euros", process_order_with_money(CustomerOrder::new(DrinkType::Chocolate).with_sugar(2), 0.1));
    }

    // Price Manager
    #[test]
    fn test_exact_price_for_chocolate() {
        assert_eq!(0.0, check_missing_money_for_product(DrinkType::Chocolate, 0.5));
    }

    #[test]
    fn test_missing_money_for_chocolate() {
        assert_relative_eq!(0.2, check_missing_money_for_product(DrinkType::Chocolate, 0.3));
    }

    #[test]
    fn test_excess_of_money_for_chocolate() {
        assert_eq!(0.0, check_missing_money_for_product(DrinkType::Chocolate, 0.70));
    }



}