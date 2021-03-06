#[cfg(test)]
mod tests{
    use approx::assert_relative_eq;

    use crate::coffe_machine::coffee_machine::*;
    use crate::price_manager::price_manager::check_missing_money_for_product;
    use std::collections::HashMap;
    use crate::printer::printer::generate_sales_report;

    #[cfg(test)]
    use mockall::{predicate::eq};

    pub struct DefaultQuantityChecker;

    impl BeverageQuantityChecker for DefaultQuantityChecker {
        fn is_empty(&self, drink_type: DrinkType) -> bool {
            match drink_type {
                _ => false
            }
        }
    }

    pub struct DefaultEmailNotifier;

    impl EmailNotifier for DefaultEmailNotifier {
        fn notify_missing_drink(&self, drink_type: DrinkType) {
            match drink_type {
                _ => { /* Notify */ }
            }
        }
    }


    //Coffee Machine: Basic Orders
    #[test]
    fn test_coffee_no_sugar() {
        let mut coffee_machine = CoffeeMachine::new(&DefaultQuantityChecker {}, &DefaultEmailNotifier {});
        assert_eq!("C::", coffee_machine.process_order(CustomerOrder::new(DrinkType::Coffee).with_sugar(0)));
    }

    #[test]
    fn test_tea_one_sugar() {
        let mut coffee_machine = CoffeeMachine::new(&DefaultQuantityChecker {}, &DefaultEmailNotifier {});
        assert_eq!("T:1:0", coffee_machine.process_order(CustomerOrder::new(DrinkType::Tea).with_sugar(1)));
    }

    #[test]
    fn test_chocolate_two_sugars() {
        let mut coffee_machine = CoffeeMachine::new(&DefaultQuantityChecker {}, &DefaultEmailNotifier {});
        assert_eq!("H:2:0", coffee_machine.process_order(CustomerOrder::new(DrinkType::Chocolate).with_sugar(2)));
    }

    #[test]
    fn test_user_message() {
        assert_eq!("M:my own shiny message", process_message("my own shiny message".to_owned()));
    }

    #[test]
    fn test_orange_juice() {
        let mut coffee_machine = CoffeeMachine::new(&DefaultQuantityChecker {}, &DefaultEmailNotifier {});
        assert_eq!("O::", coffee_machine.process_order(CustomerOrder::new(DrinkType::OrangeJuice).with_sugar(0)));
    }

    #[test]
    fn test_tea_two_sugars_extra_hot() {
        let mut coffee_machine = CoffeeMachine::new(&DefaultQuantityChecker {}, &DefaultEmailNotifier {});
        assert_eq!("Th:2:0", coffee_machine.process_order(CustomerOrder::new(DrinkType::Tea).with_sugar(2).extra_hot()));
    }

    #[test]
    fn test_chocolate_no_sugar_extra_hot() {
        let mut coffee_machine = CoffeeMachine::new(&DefaultQuantityChecker {}, &DefaultEmailNotifier {});
        assert_eq!("Hh::", coffee_machine.process_order(CustomerOrder::new(DrinkType::Chocolate).extra_hot()));
    }

    // Coffee Machine: Orders with money
    #[test]
    fn test_accept_order_with_more_than_enough_money() {
        let mut coffee_machine = CoffeeMachine::new(&DefaultQuantityChecker {}, &DefaultEmailNotifier {});
        assert_eq!("H:2:0", coffee_machine.process_order_with_money(CustomerOrder::new(DrinkType::Chocolate).with_sugar(2), 0.7));
    }

    #[test]
    fn test_accept_order_with_exact_money() {
        let mut coffee_machine = CoffeeMachine::new(&DefaultQuantityChecker {}, &DefaultEmailNotifier {});
        assert_eq!("H:2:0", coffee_machine.process_order_with_money(CustomerOrder::new(DrinkType::Chocolate).with_sugar(2), 0.5));
    }

    #[test]
    fn test_reject_order_with_not_enough_money() {
        let mut coffee_machine = CoffeeMachine::new(&DefaultQuantityChecker {}, &DefaultEmailNotifier {});
        assert_eq!("M:Missing 0.4 euros", coffee_machine.process_order_with_money(CustomerOrder::new(DrinkType::Chocolate).with_sugar(2), 0.1));
    }

    // Coffee Machine: Orders that change state
    #[test]
    fn test_make_some_orders_and_check_resulting_money_earned() {
        let mut coffee_machine = CoffeeMachine::new(&DefaultQuantityChecker {}, &DefaultEmailNotifier {});
        coffee_machine.process_order(CustomerOrder::new(DrinkType::Chocolate));
        coffee_machine.process_order(CustomerOrder::new(DrinkType::OrangeJuice));
        assert_relative_eq!(1.1, coffee_machine.get_total_money_sold());
    }

    #[test]
    fn test_make_some_orders_and_check_how_many_sold_drinks() {
        let mut coffee_machine = CoffeeMachine::new(&DefaultQuantityChecker {}, &DefaultEmailNotifier {});
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

        let coffee_machine = CoffeeMachine { sold_drinks, money_earned: 33.60, quantity_checker: &DefaultQuantityChecker {}, email_notifier: &DefaultEmailNotifier {} };

        assert_eq!("SALES REPORT\nTea: 2\nCoffee: 4\nChocolate: 0\nOrangeJuice: 0\nTotal Money: 33.6", generate_sales_report(&coffee_machine))
    }

    // Shortage Checking tests
    #[test]
    fn test_beverage_shortage_generates_error_output() {
        let mut quantity_checker = MockBeverageQuantityChecker::new();
        quantity_checker.expect_is_empty()
            .with(eq(DrinkType::Chocolate))
            .return_const(true);

        let email_notifier = DefaultEmailNotifier;

        let mut coffee_machine = CoffeeMachine::new(&quantity_checker, &email_notifier);

        let order_with_shortage = CustomerOrder::new(DrinkType::Chocolate);

        assert_eq!("ERROR: Shortage of Chocolate", coffee_machine.process_order(order_with_shortage));
    }

    #[test]
    fn test_beverage_shortage_sends_email() {
        let mut email_notifier = MockEmailNotifier::new();

        // Will make the test fail in case notify_missing_drink(DrinkType::Chocolate) isn't called once
        email_notifier.expect_notify_missing_drink()
            .with(eq(DrinkType::OrangeJuice))
            .times(1)
            .return_const(());

        let mut quantity_checker = MockBeverageQuantityChecker::new();
        quantity_checker.expect_is_empty()
            .with(eq(DrinkType::OrangeJuice))
            .return_const(true);

        let mut coffee_machine = CoffeeMachine::new(&quantity_checker, &email_notifier);

        let order_with_shortage = CustomerOrder::new(DrinkType::OrangeJuice);

        coffee_machine.process_order(order_with_shortage);
    }
}
