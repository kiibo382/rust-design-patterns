trait Visitor {
    fn visit(&self, element: &dyn Element);
}

trait Element {
    fn accept(&self, visitor: &dyn Visitor);
}

struct ConcreteElementA;
impl Element for ConcreteElementA {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit(self);
    }
}

struct ConcreteElementB;
impl Element for ConcreteElementB {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit(self);
    }
}

struct ConcreteVisitor1;
impl Visitor for ConcreteVisitor1 {
    fn visit(&self, _element: &dyn Element) {
        println!("ConcreteVisitor1");
    }
}

struct ConcreteVisitor2;
impl Visitor for ConcreteVisitor2 {
    fn visit(&self, _element: &dyn Element) {
        println!("ConcreteVisitor2");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visitor() {
        let elements: Vec<Box<dyn Element>> = vec![
            Box::new(ConcreteElementA),
            Box::new(ConcreteElementB),
        ];
        let visitors: Vec<Box<dyn Visitor>> = vec![
            Box::new(ConcreteVisitor1),
            Box::new(ConcreteVisitor2),
        ];

        for element in elements {
            for visitor in &visitors {
                element.accept(visitor);
            }
        }
    }
}
