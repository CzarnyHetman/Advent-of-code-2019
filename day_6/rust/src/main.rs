use std::{env, path, fs, collections::{HashMap, HashSet}};

#[derive(Debug)]
#[derive(PartialEq)]
struct Node<'a> {
    index: usize,
    parent_index: Option<usize>,
    label: &'a str,
    children: Vec<usize>
}

impl<'a> Node<'a> {
    fn new<'b> (index: usize, label: &'a str) -> Self {
        Node::<'a> {
            index,
            parent_index: None,
            label,
            children: vec![]
        }
    }

    fn new_satelite<'b> (index: usize, label: &'a str, parent_index: usize) -> Self {
        Node::<'a> {
            index,
            parent_index: Some(parent_index),
            label,
            children: vec![]
        }
    }

    fn init_tree (data: &'a Vec<OrbitDefinition>) -> Vec<Node<'a>> {
        let mut tree: Vec<Node> = Vec::new();
        let mut map: HashMap<&str, usize> = HashMap::new();
        for ele in data {
            if map.contains_key(ele.center) {
                let len = tree.len();
                let index = *map.get(ele.center).unwrap();
                let center = tree.get_mut(index).unwrap();

                if map.contains_key(ele.satelite) {
                    let index = *map.get(ele.satelite).unwrap();
                    center.children.push(index);
                    let center_index = center.index;
                    let mut satelite = tree.get_mut(index).unwrap();
                    satelite.parent_index = Some(center_index);
                } else {
                    let satelite = Node::new_satelite(len, ele.satelite, center.index);
                    center.children.push(satelite.index);
                    map.insert(ele.satelite, len);
                    tree.push(satelite);
                }
            } else {
                let len = tree.len();
                let center = Node::new(len, ele.center);

                map.insert(ele.center, len);
                tree.push(center);

                let len = tree.len();
                let index = *map.get(ele.center).unwrap();
                let center = tree.get_mut(index).unwrap();

                if map.contains_key(ele.satelite) {
                    let index = *map.get(ele.satelite).unwrap();
                    center.children.push(index);
                    let center_index = center.index;
                    let mut satelite = tree.get_mut(index).unwrap();
                    satelite.parent_index = Some(center_index);
                } else {
                    let satelite = Node::new_satelite(len, ele.satelite, center.index);
                    center.children.push(satelite.index);
                    map.insert(ele.satelite, len);
                    tree.push(satelite);
                }
            }

            
        }
        tree
    }

    fn count_orbits_between(tree: &Vec<Node>, start_node: &str, end_node: &str) -> usize{
        let first_path = Node::path_of(tree, start_node);
        let second_path = Node::path_of(tree, end_node);
        
        let mut second_set: HashMap<&str, usize> = HashMap::new();
        for ele in second_path.iter().enumerate() {
            second_set.insert(ele.1, ele.0);
        }

        let mut path_count = 0;
        for ele in first_path {
            path_count += 1;
            if second_set.contains_key(ele) {
                path_count += second_set.get(ele).unwrap();
                break;
            }
        }

        path_count
    }

    fn path_of(tree: &'a Vec<Node>, node: &str) -> Vec<&'a str> {
        let node = Node::find_node(tree, node);
        match node {
            Some(node) => {
                let mut result = vec![];
                let mut node = node;
                loop {
                    match node.parent_index {
                        Some(index) => {
                            node = tree.get(index).unwrap();
                            result.push(node.label);
                        },
                        None => break
                    }
                }
                result
            },
            None => vec![] 
        }
    }

    fn find_node<'b, 'c>(tree: &'b Vec<Node>, label: &str) -> Option<&'b Node<'b>> {
        for ele in tree.iter(){
            if ele.label == label {
                return Some(&ele)
            }
        }

        None
    }
    
    fn count_orbits(tree: &Vec<Node>) -> i32 {
        let mut count = 0;
        for ele in tree.iter().rev() {
            if ele.parent_index.is_some(){
                count += Node::count_suborbits(&tree, &ele)
            }
        }

        count
    }

    fn count_suborbits(tree: &Vec<Node>, node: &Node) -> i32 {
        if node.parent_index.is_none() {
            return 0;
        }
        let mut count = 0;
        let mut parent_index = node.parent_index;

        while parent_index.is_some() {
            count += 1;
            parent_index = tree.get(parent_index.unwrap()).unwrap().parent_index;
        }

        count
    }
}

#[derive(Debug)]
struct OrbitDefinition<'a> {
    center: &'a str,
    satelite: &'a str
}

impl<'a> OrbitDefinition<'a> {
    fn new(line: &'a str) -> Option<Self> {

        let split_index = line.find(")")?;


        let center = &line[..split_index];
        let satelite = &line[split_index+1..];
        Some(OrbitDefinition {center, satelite})
    }
}

fn main() {
    let file = get_file().unwrap();
    let lines = file.lines();
    let orbit_definitions: Vec<OrbitDefinition> = lines.filter_map(|line| OrbitDefinition::new(line)).collect();
    let tree = Node::init_tree(&orbit_definitions);
    let unmatched: Vec<&str> = tree.iter().filter_map(|item|  match item.parent_index { Some(_) => None, None => Some(item.label)}).collect();
    let count = Node::count_orbits(&tree);
    
    println!("Unmatched: {}", unmatched.len());
    println!("Unmatched: {:?}", unmatched);
    println!("Count: {}", count);

    let count_orbits = Node::count_orbits_between(&tree, "YOU", "SAN");

    println!("Orbits between: {}", count_orbits);
}

fn get_file() -> Option<String> {
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);

    let path = &args.get(1).expect("Supply path param");
    println!("{}", path);

    let path = path::Path::new(path);
    if !path.exists() {
        println!("Path unreachable");
        return None;
    }

    let file = fs::read_to_string(path).ok()?;
    
    Some(file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_tree_creates_two_nodes() {
        let data = vec![
            OrbitDefinition{center: "A", satelite: "B"}
        ];

        let tree = Node::init_tree(&data);

        let expected_tree = vec![
            Node {index: 0 ,parent_index: None   , label: "A", children: vec![1]}, 
            Node {index: 1 ,parent_index: Some(0), label: "B", children: vec![]}
        ];

        assert_eq!(&tree, &expected_tree);
    }

    #[test]
    fn path_of_gives_correct_path() {
        let tree = vec![
            Node {index: 0 ,parent_index: None   , label: "A", children: vec![1]}, 
            Node {index: 1 ,parent_index: Some(0), label: "B", children: vec![2, 4]},
            Node {index: 2 ,parent_index: Some(1), label: "C", children: vec![3]},
            Node {index: 3 ,parent_index: Some(2), label: "D", children: vec![]},
            Node {index: 4 ,parent_index: Some(1), label: "E", children: vec![]},
            ];

        let node = "D";

        let path = Node::path_of(&tree, node);

        let expected_path = vec![
            "C", "B", "A"
        ];

        assert_eq!(path, expected_path);
    }

    #[test]
    fn count_orbits_between_gives_3() {
        let tree = vec![
            Node {index: 0 ,parent_index: None   , label: "A", children: vec![1]}, 
            Node {index: 1 ,parent_index: Some(0), label: "B", children: vec![2, 4]},
            Node {index: 2 ,parent_index: Some(1), label: "C", children: vec![3]},
            Node {index: 3 ,parent_index: Some(2), label: "D", children: vec![]},
            Node {index: 4 ,parent_index: Some(1), label: "E", children: vec![]},
            ];

        let path = Node::count_orbits_between(&tree, "E", "D");

        

        assert_eq!(path, 2);
    }

    #[test]
    fn fill_should_create_chain() {
        let data = vec![
            OrbitDefinition{center: "A", satelite: "B"},    //1B
            OrbitDefinition{center: "B", satelite: "C"},    //1B + 1P
            OrbitDefinition{center: "C", satelite: "D"},    //1B + 2P
            OrbitDefinition{center: "B", satelite: "E"},    //1B + 1P
            ];

        let tree = Node::init_tree(&data);

        let expected_tree = vec![
            Node {index: 0 ,parent_index: None   , label: "A", children: vec![1]}, 
            Node {index: 1 ,parent_index: Some(0), label: "B", children: vec![2, 4]},
            Node {index: 2 ,parent_index: Some(1), label: "C", children: vec![3]},
            Node {index: 3 ,parent_index: Some(2), label: "D", children: vec![]},
            Node {index: 4 ,parent_index: Some(1), label: "E", children: vec![]},
            ];

        assert_eq!(&tree, &expected_tree);
    }

    #[test]
    fn fill_should_create_chain_when_root_not_first() {
        let data = vec![
            OrbitDefinition{center: "B", satelite: "C"},    //1B + 1P
            OrbitDefinition{center: "C", satelite: "D"},    //1B + 2P
            OrbitDefinition{center: "B", satelite: "E"},    //1B + 1P
            OrbitDefinition{center: "A", satelite: "B"},    //1B
            ];

        let tree = Node::init_tree(&data);

        let expected_tree = vec![
            Node {index: 0 ,parent_index: Some(4), label: "B", children: vec![1, 3]},
            Node {index: 1 ,parent_index: Some(0), label: "C", children: vec![2]},
            Node {index: 2 ,parent_index: Some(1), label: "D", children: vec![]},
            Node {index: 3 ,parent_index: Some(0), label: "E", children: vec![]},
            Node {index: 4 ,parent_index: None   , label: "A", children: vec![0]}, 
            ];

        assert_eq!(&tree, &expected_tree);
    }

    #[test]
    fn fill_should_create_chain_when_reversed() {
        let data = vec![
            OrbitDefinition{center: "B", satelite: "E"},    //1B + 1P
            OrbitDefinition{center: "C", satelite: "D"},    //1B + 2P
            OrbitDefinition{center: "B", satelite: "C"},    //1B + 1P
            OrbitDefinition{center: "A", satelite: "B"},    //1B
            ];

        let tree = Node::init_tree(&data);

        let expected_tree = vec![
            Node {index: 0 ,parent_index: Some(4), label: "B", children: vec![1, 2]},
            Node {index: 1 ,parent_index: Some(0), label: "E", children: vec![]},
            Node {index: 2 ,parent_index: Some(0), label: "C", children: vec![3]},
            Node {index: 3 ,parent_index: Some(2), label: "D", children: vec![]},
            Node {index: 4 ,parent_index: None   , label: "A", children: vec![0]}, 
            ];

        assert_eq!(&tree, &expected_tree);
    }

    #[test]
    fn fill_when_scrumbled_should_create_chain() {
        let data = vec![
            OrbitDefinition{center: "A", satelite: "B"},    //1B
            OrbitDefinition{center: "C", satelite: "D"},    //1B + 2P
            OrbitDefinition{center: "B", satelite: "C"},    //1B + 1P
            OrbitDefinition{center: "B", satelite: "E"},    //1B + 1P
            ];

        let tree = Node::init_tree(&data);

        let expected_tree = vec![
            Node { index: 0, parent_index: None   , label: "A", children: vec![1] },
            Node { index: 1, parent_index: Some(0), label: "B", children: vec![2, 4] },
            Node { index: 2, parent_index: Some(1), label: "C", children: vec![3] },
            Node { index: 3, parent_index: Some(2), label: "D", children: vec![] },
            Node { index: 4, parent_index: Some(1), label: "E", children: vec![] }
            ];

        assert_eq!(&tree, &expected_tree);
    }


    #[test]
    fn count_orbits_should_give_8() {
        let tree = vec![
            Node {index: 0 ,parent_index: None   , label: "A", children: vec![1]}, 
            Node {index: 1 ,parent_index: Some(0), label: "B", children: vec![2, 4]},
            Node {index: 2 ,parent_index: Some(1), label: "C", children: vec![3]},
            Node {index: 3 ,parent_index: Some(2), label: "D", children: vec![]},
            Node {index: 4 ,parent_index: Some(1), label: "E", children: vec![]},
            ];

        let count = Node::count_orbits(&tree);

        assert_eq!(count, 8)
    }

    #[test]
    fn new_orbit_definition_created() {
        let line = &String::from("A)B")[..];
        let od = OrbitDefinition::new(line).unwrap();

        assert_eq!(od.center, "A");
        assert_eq!(od.satelite, "B");

    }
}
