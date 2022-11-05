pub trait AbstractFactory {
    fn create_product_a(&self) -> Box<dyn ProductA>;
    fn create_product_b(&self) -> Box<dyn ProductB>;
}

pub trait ProductA {
    fn useful_function_a(&self) -> &str;
}

pub trait ProductB {
    fn useful_function_b(&self) -> &str;
}

pub struct ConcreteFactory1;
impl AbstractFactory for ConcreteFactory1 {
    fn create_product_a(&self) -> Box<dyn ProductA> {
        Box::new(ConcreteProductA1::new())
    }

    fn create_product_b(&self) -> Box<dyn ProductB> {
        Box::new(ConcreteProductB1::new())
    }
}

impl ConcreteFactory1 {
    pub fn new() -> Self {
        Self
    }
}

pub struct ConcreteProductA1 {
    pub useful_function_a: String,
}

impl ProductA for ConcreteProductA1 {
    fn useful_function_a(&self) -> &str {
        &self.useful_function_a
    }
}

impl ConcreteProductA1 {
    pub fn new() -> Self {
        Self {
            useful_function_a: String::from("The result of the product A1."),
        }
    }
}

pub struct ConcreteProductB1 {
    pub useful_function_b: String,
}

impl ProductB for ConcreteProductB1 {
    fn useful_function_b(&self) -> &str {
        &self.useful_function_b
    }
}

impl ConcreteProductB1 {
    pub fn new() -> Self {
        Self {
            useful_function_b: String::from("The result of the product B1."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abstract_factory() {
        let factory1 = ConcreteFactory1::new();
        let product_a1 = factory1.create_product_a();
        let product_b1 = factory1.create_product_b();

        assert_eq!(
            product_a1.useful_function_a(),
            "The result of the product A1."
        );
        assert_eq!(
            product_b1.useful_function_b(),
            "The result of the product B1."
        );
    }
}
