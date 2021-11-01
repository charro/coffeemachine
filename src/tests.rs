#[cfg(test)]
mod tests{
    use approx::assert_relative_eq;

    use crate::coffe_machine::coffee_machine::{CoffeeMachine, CustomerOrder, DrinkType, process_message};
    use crate::price_manager::price_manager::check_missing_money_for_product;
    use std::collections::HashMap;
    use crate::printer::printer::generate_sales_report;

    //Coffee Machine: Basic Orders
    #[test]
    fn test_coffee_no_sugar() {
        let mut coffee_machine = CoffeeMachine::new();
        assert_eq!("C::", coffee_machine.process_order(CustomerOrder::new(DrinkType::Coffee).with_sugar(0)));
    }

    #[test]
    fn test_tea_one_sugar() {
        let mut coffee_machine = CoffeeMachine::new();
        assert_eq!("T:1:0", coffee_machine.process_order(CustomerOrder::new(DrinkType::Tea).with_sugar(1)));
    }

    #[test]
    fn test_chocolate_two_sugars() {
        let mut coffee_machine = CoffeeMachine::new();
        assert_eq!("H:2:0", coffee_machine.process_order(CustomerOrder::new(DrinkType::Chocolate).with_sugar(2)));
    }

    #[test]
    fn test_user_message() {
        assert_eq!("M:my own shiny message", process_message("my own shiny message".to_owned()));
    }

    #[test]
    fn test_orange_juice() {
        let mut coffee_machine = CoffeeMachine::new();
        assert_eq!("O::", coffee_machine.process_order(CustomerOrder::new(DrinkType::OrangeJuice).with_sugar(0)));
    }

    #[test]
    fn test_tea_two_sugars_extra_hot() {
        let mut coffee_machine = CoffeeMachine::new();
        assert_eq!("Th:2:0", coffee_machine.process_order(CustomerOrder::new(DrinkType::Tea).with_sugar(2).extra_hot()));
    }

    #[test]
    fn test_chocolate_no_sugar_extra_hot() {
        let mut coffee_machine = CoffeeMachine::new();
        assert_eq!("Hh::", coffee_machine.process_order(CustomerOrder::new(DrinkType::Chocolate).extra_hot()));
    }

    // Coffee Machine: Orders with money
    #[test]
    fn test_accept_order_with_more_than_enough_money() {
        let mut coffee_machine = CoffeeMachine::new();
        assert_eq!("H:2:0", coffee_machine.process_order_with_money(CustomerOrder::new(DrinkType::Chocolate).with_sugar(2), 0.7));
    }

    #[test]
    fn test_accept_order_with_exact_money() {
        let mut coffee_machine = CoffeeMachine::new();
        assert_eq!("H:2:0", coffee_machine.process_order_with_money(CustomerOrder::new(DrinkType::Chocolate).with_sugar(2), 0.5));
    }

    #[test]
    fn test_reject_order_with_not_enough_money() {
        let mut coffee_machine = CoffeeMachine::new();
        assert_eq!("M:Missing 0.4 euros", coffee_machine.process_order_with_money(CustomerOrder::new(DrinkType::Chocolate).with_sugar(2), 0.1));
    }

    // Coffee Machine: Orders that change state
    #[test]
    fn test_make_some_orders_and_check_resulting_money_earned() {
        let mut coffee_machine = CoffeeMachine::new();
        coffee_machine.process_order(CustomerOrder::new(DrinkType::Chocolate));
        coffee_machine.process_order(CustomerOrder::new(DrinkType::OrangeJuice));
        assert_relative_eq!(1.1, coffee_machine.get_total_money_sold());
    }

    #[test]
    fn test_make_some_orders_and_check_how_many_sold_drinks() {
        let mut coffee_machine = CoffeeMachine::new();
        coffee_machine.process_order(CustomerOrder::new(DrinkType::Tea));
        assert_eq!(1, coffee_machine.get_total_amount_sold(DrinkType::Tea));
        assert_eq!(0, coffee_machine.get_total_amount_sold(DrinkType::Chocolate));
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

    #[test]
    fn test_exact_money_for_orange_juice() {
        assert_eq!(0.0, check_missing_money_for_product(DrinkType::OrangeJuice, 0.60));
    }

    // Printer tests
    #[test]
    fn test_print_a_sales_report() {
        let sold_drinks: HashMap<DrinkType, u32> = [
            (DrinkType::Tea, 2),
            (DrinkType::Coffee, 4)
        ].iter().copied().collect();

        let coffee_machine = CoffeeMachine { sold_drinks, money_earned: 33.60 };

        assert_eq!("SALES REPORT\nTea: 2\nCoffee: 4\nChocolate: 0\nOrangeJuice: 0\nTotal Money: 33.6", generate_sales_report(&coffee_machine))
    }

}