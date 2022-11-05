use std::collections::HashMap;

struct ProductBuilder<T>
where
    T: Clone,
{
    product: T,
    settings: HashMap<String, String>,
}

impl<T> ProductBuilder<T>
where
    T: Clone,
{
    pub fn new(product: T, settings: HashMap<String, String>) -> Self {
        Self { product, settings }
    }

    pub fn product(self) -> T {
        self.product
    }

    pub fn add_setting(mut self, key: String, value: String) -> Self {
        self.settings.insert(key, value);
        ProductBuilder { ..self }
    }
}

impl ProductBuilder<Product1> {
    pub fn build(self) -> Product {
        Product {
            product_type: ProductType::Product1,
            settings: self.settings,
        }
    }
}

impl ProductBuilder<Product2> {
    pub fn build(self) -> Product {
        Product {
            product_type: ProductType::Product2,
            settings: self.settings,
        }
    }
}

#[derive(Debug)]
struct Product {
    product_type: ProductType,
    settings: HashMap<String, String>,
}

impl Product {
    pub fn new(product_type: ProductType, settings: HashMap<String, String>) -> Self {
        Self {
            product_type,
            settings,
        }
    }
}

#[derive(Debug)]
enum ProductType {
    Product1,
    Product2,
}

#[derive(Clone)]
struct Product1 {
    parts: Vec<String>,
}
impl Product1 {
    pub fn new(parts: Vec<String>) -> Self {
        Self { parts }
    }

    pub fn builder() -> ProductBuilder<Self> {
        ProductBuilder::new(Self::new(vec![]), HashMap::new())
    }

    pub fn add(&mut self, part: String) {
        self.parts.push(part);
    }
}

#[derive(Clone)]
struct Product2 {
    parts: Vec<String>,
}
impl Product2 {
    pub fn new(parts: Vec<String>) -> Self {
        Self { parts }
    }

    pub fn builder() -> ProductBuilder<Self> {
        ProductBuilder::new(Self::new(vec![]), HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let _product_a = Product1::builder()
            .add_setting("key".to_string(), "value".to_string())
            .add_setting("key2".to_string(), "value2".to_string())
            .build();

        let _product_b = Product2::builder()
            .add_setting("key".to_string(), "value".to_string())
            .add_setting("key2".to_string(), "value2".to_string())
            .build();
    }
}
