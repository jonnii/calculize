mod core {
    use std::iter::FromIterator;

    #[derive(Debug)]
    pub struct Column<'a> {
        name: &'a str,
        data: Vec<f64>, 
    }

    pub struct Table<'a> {
        columns: Vec<Column<'a>>
    }

    pub enum Allocation {
        Value(f64)
    }

    pub enum CalculationResult {
        Value(Vec<Allocation>),
        Conditional
    }

    pub trait Calculator {
        fn calculate(&self, table: &Table) -> CalculationResult;
    }

    pub trait Rule {
        fn create_calculator(&self) -> Box<Calculator>;
    }

    impl<'a> Table<'a> {
        pub fn new() -> Table<'a> {
            let quantities = Vec::from_iter((0..10000).map(|_| 100.0));
            let prices = Vec::from_iter((0..10000).map(|_| 1.0));
            
            let c1 = Column { name: "quantity", data: quantities };
            let c2 = Column { name: "price", data: prices };
            let columns = vec![c1, c2];
            return Table { columns: columns };
        }

        fn column(&self, name: &str) -> &Column<'a> {
            return self.columns.iter().find(|x| x.name == name).unwrap();
        }

        pub fn zip_columns(&self, first: &str, second: &str) -> Vec<Allocation> {
            let c1 = self.column(first);
            let c2 = self.column(second);

            let market_values = c1.data.iter().zip(c2.data.iter())
                .map(|x| x.0 * x.1);
            
            let base = market_values.map(|x| x * 0.07);
            let allocations = base.map(|x| Allocation::Value(x));

            return Vec::from_iter(allocations);
        }
    }

    pub fn calculate_total(result: CalculationResult) -> f64 {
        match result {
            CalculationResult::Value(allocations) =>
                allocations.iter().map(|x| 
                    match x {
                        &Allocation::Value(x) => x
                    }
                ).sum(),
            _ => 0.0
        } 
     }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use core::{Rule, Table, Calculator, CalculationResult, calculate_total};

    struct SimpleCalculator {}

    impl SimpleCalculator{
        fn new() -> SimpleCalculator {
            SimpleCalculator{}
        }
    }

    impl Calculator for SimpleCalculator {
        fn calculate(&self, table: &Table) -> CalculationResult {
            let results = table.zip_columns("quantity", "price");
            return CalculationResult::Value(results);
        }
    }

    struct SampleRule {}

    impl Rule for SampleRule {
        fn create_calculator(&self) -> Box<Calculator> {
            return Box::new(SimpleCalculator::new());
        }
    }

    #[test]
    fn distance_test() {
        let table = Table::new();

        let rule = SampleRule{};

        let calculator = rule.create_calculator();
        let result = calculator.calculate(&table);

        let total = calculate_total(result);
        println!("{:?}", total);
    }
}