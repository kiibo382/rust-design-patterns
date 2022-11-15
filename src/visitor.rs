trait Visitor<T> {
    fn visit(&mut self, element: &T);
}

trait Element {
    fn accept(&self, visitor: &mut dyn Visitor<Self>);
}

#[derive(Debug)]
struct ConcreteElementA;
impl Element for ConcreteElementA {
    fn accept(&self, visitor: &mut dyn Visitor<Self>) {
        visitor.visit(self);
    }
}

#[derive(Debug)]
struct ConcreteElementB;
impl Element for ConcreteElementB {
    fn accept(&self, visitor: &mut dyn Visitor<Self>) {
        visitor.visit(self);
    }
}

struct ConcreteVisitor1;
impl Visitor<ConcreteElementA> for ConcreteVisitor1 {
    fn visit(&mut self, a: &ConcreteElementA) {
        println!("Visited {:?}.", a);
    }
}

impl Visitor<ConcreteElementB> for ConcreteVisitor1 {
    fn visit(&mut self, b: &ConcreteElementB) {
        println!("Visited {:?}.", b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visitor() {
        let mut visitor = ConcreteVisitor1;
        let a = ConcreteElementA;
        a.accept(&mut visitor);
        
        let mut visitor = ConcreteVisitor1;
        let b = ConcreteElementB;
        b.accept(&mut visitor);
    }
}
