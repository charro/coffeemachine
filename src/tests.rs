#[cfg(test)]
mod tests{
    use crate::coffe_machine::coffe_machine::CustomerOrder;
    use crate::coffe_machine::coffe_machine::DrinkType;
    use crate::coffe_machine::coffe_machine::process_order;

    #[test]
    fn test_coffee_no_sugar() {
        assert_eq!("C:0:0", process_order(CustomerOrder::new(DrinkType::Coffee, 0)));
    }
}