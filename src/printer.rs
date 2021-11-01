pub mod printer {
    use crate::coffe_machine::coffee_machine::{CoffeeMachine, DrinkType};
    use enum_iterator::IntoEnumIterator;

    pub fn generate_sales_report(coffeemachine: &CoffeeMachine) -> String {
        let mut report = "SALES REPORT".to_string();

        for drink_type in DrinkType::into_enum_iter() {
            report = format!("{}\n{}: {}", report, drink_type, coffeemachine.get_total_amount_sold(drink_type));
        }

        format!("{}\nTotal Money: {}", report, coffeemachine.get_total_money_sold())
    }
}