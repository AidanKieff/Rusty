struct NameStack <'a>{
    names: Vec<&'a str>,
}

impl<'a> NameStack<'a> {
    fn new() -> Self {
        NameStack { names: Vec::new() }
    }

    fn add_name(&mut self, name: &'a str) {
        self.names.push(name);
    }

    fn remove_name_with_substr(&mut self, sub_str: &str) -> Vec<&str> {
        let mut removed: Vec<&str> = Vec::new();
        let mut i = 0;
        while i < self.names.len() {

            if self.names[i].contains(sub_str) {
                let remove = self.names.remove(i);
                removed.push(remove);
            } else {
                i += 1;
            }
        }

        if removed.is_empty() {
            panic!("Name with substring not found");
        } else {
            removed
        }
        
    }
}


fn main() {
    let mut my_names = NameStack::new();
    
    my_names.add_name("Alice");
    my_names.add_name("Bob");
    my_names.add_name("Cindyice");
    my_names.add_name("Emily");
    
    let removed = my_names.remove_name_with_substr("ice");
    println!("Removed: {:?}", removed);
    //assert_eq!(my_names.names.len(), 3);

}


// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime
//    is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is
//    &self or &mut self, the lifetime of self is assigned to all output
//    lifetime parameters. 

/*
struct NameStack<'a> {
    names: Vec<&'a str>,
}

impl<'a> NameStack<'a> {
    fn new() -> Self {
        NameStack { names: Vec::new() }
    }
    fn add_name(&mut self, name: &'a str) {
        self.names.push(name);
    }
    fn remove_name_with_substr(&mut self, sub_str: &str) -> &'a str {
        for i in 0..self.names.len() {
            if self.names[i].contains(sub_str) {
                let removed = self.names.remove(i);
                return removed;
            }
        }
        panic!("Name with substring not found");
    }
}

fn main() {
    let mut my_names = NameStack::new();
    my_names.add_name("Al");
    my_names.add_name("Jan");
    my_names.add_name("Cindy");
    my_names.add_name("Emily");
    let removed = my_names.remove_name_with_substr("ice");
    println!("Removed: {removed}");
    assert_eq!(my_names.names.len(), 3);
}

*/