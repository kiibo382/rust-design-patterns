trait Builder {
    fn produce_part_a(&mut self);
    fn produce_part_b(&mut self);
    fn produce_part_c(&mut self);
    fn get_result(&self) -> Box<dyn Product>;
}

trait Product {
    fn list_parts(&self) -> String;
}

struct Director {
    builder: Box<dyn Builder>,
}

impl Director {
    pub fn new(builder: Box<dyn Builder>) -> Self {
        Self { builder }
    }

    pub fn build_minimal_viable_product(&mut self) {
        self.builder.produce_part_a();
    }

    pub fn build_full_featured_product(&mut self) {
        self.builder.produce_part_a();
        self.builder.produce_part_b();
        self.builder.produce_part_c();
    }
}

#[derive(Clone)]
struct ConcreteBuilder1 {
    product: Product1,
}

impl ConcreteBuilder1 {
    pub fn new() -> Self {
        Self {
            product: Product1::new(),
        }
    }

    pub fn product(self) -> Product1 {
        self.product
    }
}

impl Builder for ConcreteBuilder1 {
    fn produce_part_a(&mut self) {
        self.product.add(String::from("PartA1"));
    }

    fn produce_part_b(&mut self) {
        self.product.add(String::from("PartB1"));
    }

    fn produce_part_c(&mut self) {
        self.product.add(String::from("PartC1"));
    }

    fn get_result(&self) -> Box<dyn Product> {
        Box::new(self.product.clone())
    }
}

#[derive(Clone)]
struct Product1 {
    parts: Vec<String>,
}

impl Product1 {
    pub fn new() -> Self {
        Self { parts: vec![] }
    }

    pub fn add(&mut self, part: String) {
        self.parts.push(part);
    }
}

impl Product for Product1 {
    fn list_parts(&self) -> String {
        self.parts.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let mut director = Director::new(Box::new(ConcreteBuilder1::new()));
        director.build_minimal_viable_product();
        assert_eq!(director.builder.get_result().list_parts(), "PartA1");
    }
}
