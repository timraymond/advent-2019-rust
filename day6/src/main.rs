/// Node represents a member in a graph of orbits
struct Node {
    name: String,
    children: Vec<Node>,
}

impl Node {
    fn new(name: &str) -> Node {
        Node{
            name: String::from(name),
            children: Vec::new(),
        }
    }

    fn from_input(input: &str) -> Node {
        let mut com = Node::new("COM");

        for line in input.lines() {
            if !line.contains(")") {
                continue;
            }
            let mut parts = line.split(")").take(2);

            let parent = parts.next().unwrap();
            let child = Node::new(parts.next().unwrap());

            com.add(parent, child);
        }

        return com;
    }

    fn add(&mut self, target_name: &str, child: Node) {
        if let Some(target) = self.locate(target_name) {
            target.children.push(child);
        }
    }

    fn locate(&mut self, name: &str) -> Option<&mut Node> {
        if self.name == name {
            return Some(self);
        }

        for child in self.children.iter() {
            if child.name == name {
                return Some(self);
            }
        }

        return None;
    }

    fn orbits(&self) -> u32 {
        let mut sum = 1;
        for child in self.children.iter() {
            sum += child.orbits();
        }
        return sum;
    }
}

#[test]
fn test_add() {
    let mut com = Node::new("COM");
    com.add("COM", Node::new("A"));
    assert_eq!(com.orbits(), 2);
    com.add("A", Node::new("B"));
    assert_eq!(com.orbits(), 3);
}

#[test]
fn test_counts_direct_orbits() {
    let map = Node{
        name: String::from("A"),
        children: vec!(Node{name: String::from("B"), children: Vec::new()}),
    };

    assert_eq!(map.orbits(), 2);
}

#[test]
fn test_construction() {
    let input = "
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";
    let map = Node::from_input(input);
    assert_eq!(map.orbits(), 42);
}

#[test]
fn test_counts_indirect_orbits() {
    let map = Node{
        name: String::from("A"),
        children: vec!(Node{
            name: String::from("B"),
            children: vec!(Node{
                name: String::from("C"),
                children: Vec::new(),
            }),
        }),
    };

    assert_eq!(map.orbits(), 3);
}

fn main() {
    println!("Hello, world!");
}
