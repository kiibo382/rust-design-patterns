trait Visitor {
    fn visit_a(&self, a: &ConcreteElementA);
    fn visit_b(&self, b: &ConcreteElementB);
}

trait Element {
    fn accept(&self, visitor: &dyn Visitor);
}

#[derive(Debug)]
struct ConcreteElementA;
impl Element for ConcreteElementA {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_a(self);
    }
}

#[derive(Debug)]
struct ConcreteElementB;
impl Element for ConcreteElementB {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_b(self);
    }
}

struct ConcreteVisitor1;
impl Visitor for ConcreteVisitor1 {
    fn visit_a(&self, a: &ConcreteElementA) {
        println!("Visited {:?}.", a);
    }

    fn visit_b(&self, b: &ConcreteElementB) {
        println!("Visited {:?}.", b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visitor() {
        let visitor = ConcreteVisitor1;
        let a = ConcreteElementA;
        let b = ConcreteElementB;
        a.accept(&visitor);
        b.accept(&visitor);
    }
}
