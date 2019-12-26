use std::fs;
use std::collections::HashMap;

fn from_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        if !line.contains(")") {
            continue;
        }
        let mut parts = line.split(")").take(2);

        let parent = parts.next().unwrap();
        let child = parts.next().unwrap();

        orbits.entry(String::from(parent))
              .or_default()
              .push(String::from(child));
    }

    return orbits;
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
    let map = from_input(input);
    assert_eq!(orbit_count(&map, "COM", 0), 42);
}

fn orbit_count(map: &HashMap<String, Vec<String>>, target: &str, depth: u32) -> u32 {
    match map.get(target) {
        Some(children) => children.iter()
                                  .map(|child| orbit_count(map, child, depth+1))
                                  .fold(depth, |acc, e| acc + e),
        None => depth,
    }
}

fn main() {
    if let Ok(content) = fs::read_to_string("input.txt") {
        let map = from_input(&content);
        println!("Orbits: {}", orbit_count(&map, "COM", 0));
    } else {
        println!("Unable to read");
    }
}
