#[cfg(test)]
mod tests{
    use crate::coffe_machine::coffe_machine::CustomerOrder;
    use crate::coffe_machine::coffe_machine::DrinkType;
    use crate::coffe_machine::coffe_machine::process_order;

    #[test]
    fn test_coffee_no_sugar() {
        assert_eq!("C::", process_order(CustomerOrder::new(DrinkType::Coffee, 0)));
    }

    #[test]
    fn test_tea_one_sugar() {
        assert_eq!("T:1:1", process_order(CustomerOrder::new(DrinkType::Tea, 1)));
    }

    #[test]
    fn test_chocolate_two_sugars() {
        assert_eq!("H:2:1", process_order(CustomerOrder::new(DrinkType::Chocolate, 2)));
    }
}