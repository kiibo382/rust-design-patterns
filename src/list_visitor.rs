use std::fmt;

trait Visitor<T> {
    fn visit(&mut self, element: &T);
}

trait Element {
    fn accept(&self, visitor: &mut dyn Visitor<Self>)
    where
        Self: Sized;
}

trait Entry: Element {
    fn get_name(&self) -> String;
    fn get_size(&self) -> usize;
    fn add(&mut self, entry: Box<dyn Entry>);
}

struct File {
    name: String,
    size: usize,
}

impl File {
    pub fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }
}

impl Entry for File {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn add(&mut self, _entry: Box<dyn Entry>) {
        panic!("File::add is not supported");
    }
}

impl Element for File {
    fn accept(&self, visitor: &mut dyn Visitor<Self>) {
        visitor.visit(self);
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.size)
    }
}

struct Directory {
    name: String,
    directory: Vec<Box<dyn Entry>>,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            directory: vec![],
        }
    }
}

impl Entry for Directory {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_size(&self) -> usize {
        self.directory.iter().map(|entry| entry.get_size()).sum()
    }

    fn add(&mut self, entry: Box<dyn Entry>)
    where
        Self: Sized,
    {
        self.directory.push(entry);
    }
}

impl Element for Directory {
    fn accept(&self, visitor: &mut dyn Visitor<Self>) {
        visitor.visit(self);
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/ ({})", self.name, self.get_size())
    }
}

struct ListVisitor {
    current_dir: String,
}

impl ListVisitor {
    pub fn new() -> Self {
        Self {
            current_dir: String::new(),
        }
    }
}

impl Visitor<File> for ListVisitor {
    fn visit(&mut self, file: &File) {
        println!("{}{}", self.current_dir, file);
    }
}

impl Visitor<Directory> for ListVisitor {
    fn visit(&mut self, directory: &Directory) {
        println!("{}{}", self.current_dir, directory);
        let saved_dir = self.current_dir.clone();
        self.current_dir
            .push_str(&format!("{}/", directory.get_name()));
        for entry in &directory.directory {
            entry.accept(self);
        }
        self.current_dir = saved_dir;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visitor() {
        let mut root_dir = Directory::new(String::from("root"));
        root_dir.add(Box::new(File::new(String::from("diary.html"), 100)));
        root_dir.add(Box::new(File::new(String::from("index.html"), 200)));
        let mut usr_dir = Directory::new(String::from("usr"));
        usr_dir.add(Box::new(File::new(String::from("Composite.java"), 100)));
        usr_dir.add(Box::new(File::new(String::from("Visitor.java"), 200)));
        root_dir.add(Box::new(usr_dir));
        let mut list_visitor = ListVisitor::new();
        root_dir.accept(&mut list_visitor);
    }
}
