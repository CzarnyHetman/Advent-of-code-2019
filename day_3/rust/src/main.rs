use std::collections::{HashMap, HashSet};
use std::fs;
use std::env;
use std::error::Error;
use std::path;

#[derive(Debug)]
enum Dirs {
    R(i32),
    L(i32),
    U(i32),
    D(i32)
}

#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
struct Point {
    x: i32, 
    y: i32,
    dist: i32
}

impl Point {
    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn equals(&self, p2: &Point) -> bool {
        self.x == p2.x && self.y == p2.y
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);

    let path = &args.get(1).expect("Supply path param");
    println!("{}", path);

    let path = path::Path::new(path);
    if !path.exists() {
        println!("Path unreachable");
        return Ok(())
    }


    let file  = fs::read_to_string(path)?;
    let input : Vec<&str> = file.split("\n").collect();

    let wire_1 = input[0];
    let wire_2 = input[1];



    let starting_point = Point { x:0, y:0, dist: 0};
    let directions_1 = parse_path(wire_1);
    let path_1 = generate_path(&starting_point, &directions_1);

    let directions_2 = parse_path(wire_2);
    let path_2 = generate_path(&starting_point, &directions_2);

    let map_1 = generate_hashmap(&path_1);

    let mut crossings = Vec::new();

    for p2 in path_2 {
        if is_point_in_hashmap(&p2, &map_1){
            crossings.push(p2)
        }
    }

    let smallest = smallest_dist(&crossings);

    println!("Smallest Manhattan: {:?}", smallest);

    let mut steps = Vec::new();
    for p2 in crossings {
        for p1 in &path_1 {
            if p1.equals(&p2) {
                steps.push(p1.dist + p2.dist);
            }
        }
    }

    let smallest_steps = smallestfn(&steps);

    println!("smallest steps: {}", smallest_steps);

    Ok(())
}

fn smallestfn(vec: &Vec<i32>) -> i32 {
    let mut smallest = -1;
    for p in vec {
        if smallest == -1 || smallest > *p{
            smallest = *p;
        }
    }
    smallest
}

fn smallest_dist(crossings : &Vec<Point>) -> i32{
    let mut smallest = -1;
    for p in crossings {
        if smallest == -1 || smallest > p.manhattan(){
            smallest = p.manhattan();
        }
    }
    smallest
}

fn is_point_in_hashmap(p : &Point, h: &HashMap<i32, HashSet<i32>>) -> bool {
    if h.contains_key(&p.x) {
        let set = h.get(&p.x).unwrap();
        if set.contains(&p.y) {
            return true;
        }
    }
    
    false
}

fn generate_hashmap(path : &Vec<Point>) -> HashMap<i32, HashSet<i32>>{
    let mut collection = HashMap::<i32, HashSet<i32>>::new();

    for p in path {
        if collection.contains_key(&p.x){
            let set = collection.get_mut(&p.x).unwrap();
            set.insert(p.y);
        } else {
            let mut set = HashSet::<i32>::new();
            set.insert(p.y);
            collection.insert(p.x, set);
        }
    }

    collection
}

fn parse_path(path : &str) -> Vec<Dirs> {
    path.split(",").map(|item| {
        let letter = &item[..1];
        let count: i32 = item[1..].trim().parse().expect("shit happened");
        match letter {
            "R" => Dirs::R(count),
            "L" => Dirs::L(count),
            "U" => Dirs::U(count),
            "D" => Dirs::D(count),
            _ => panic!()
        }
    }).collect()
}

fn generate_path(starting_point : &Point, directions : &Vec<Dirs>) -> Vec<Point> {
    let mut result = Vec::new();
    let mut last_point = starting_point;
    for dir in directions {
        let mut vec = generate_points_vec(last_point, dir);
        result.append(&mut vec);

        last_point = &result.last().unwrap();
    }
    result
}

fn generate_points_vec(starting_point : &Point, dir : &Dirs) -> Vec<Point> {
    let mut count = 0;
    let mut x = starting_point.x;
    let mut y = starting_point.y;
    let dist = starting_point.dist;
    let direction = match dir {
        Dirs::D(number) => {
            count = *number;
            (0, -1)
        },
        Dirs::U(number) => {
            count = *number;
            (0, 1)
        },
        Dirs::L(number) => {
            count = *number;
            (-1, 0)
        },
        Dirs::R(number) => {
            count = *number;
            (1, 0)
        }
    };

    let mut vector = Vec::new();

    for i in 0..count {
        x += direction.0;
        y += direction.1;
        let p = Point{ x, y, dist: dist + i + 1};
        vector.push(p);
    }

    vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smallest_dist_should_find_smalest() {
        let path = vec![
            Point { x: 0, y: -2, dist: 2 },
            Point { x: 0, y: -3, dist: 3 },
            Point { x: 0, y: -1, dist: 1 },
            Point { x: 0, y: -4, dist: 4 },
            Point { x: 0, y: -5, dist: 5 },
        ];

        let result = smallest_dist(&path);

        assert_eq!(result, 1)
    }

    #[test]
    fn is_point_in_hashmap_finds() {
        let p = Point { x: 0, y: -1, dist: 1 };

        let mut set = HashSet::new();
        set.insert(-1);
        set.insert(-2);
        set.insert(-3);
        set.insert(-4);
        set.insert(-5);

        let mut map = HashMap::new();
        map.insert(0, set);

        let result = is_point_in_hashmap(&p, &map);

        assert!(result);
    }

    #[test]
    fn is_point_in_hashmap_not_finds() {
        let p = Point { x: 0, y: -6, dist: 6 };

        let mut set = HashSet::new();
        set.insert(-1);
        set.insert(-2);
        set.insert(-3);
        set.insert(-4);
        set.insert(-5);

        let mut map = HashMap::new();
        map.insert(0, set);

        let result = is_point_in_hashmap(&p, &map);

        assert!(!result);
    }

    #[test]
    fn generate_hashmap_works() {
        let path = vec![
            Point { x: 0, y: -1, dist: 1 },
            Point { x: 0, y: -2, dist: 2 },
            Point { x: 0, y: -3, dist: 3 },
            Point { x: 0, y: -4, dist: 4 },
            Point { x: 0, y: -5, dist: 5 },
        ];

        let hashmap = generate_hashmap(path);


        let mut set = HashSet::new();
        set.insert(-1);
        set.insert(-2);
        set.insert(-3);
        set.insert(-4);
        set.insert(-5);

        let mut map = HashMap::new();
        map.insert(0, set);

        assert_eq!(hashmap, map);

    }

    #[test]
    fn generate_points_vec_works_for_d() {
        let p = Point { x: 0, y: 0, dist: 0};
        let dir = Dirs::D(5);

        let result = generate_points_vec(&p, &dir);
        
        let expected = vec![
            Point { x: 0, y: -1, dist: 1 },
            Point { x: 0, y: -2, dist: 2 },
            Point { x: 0, y: -3, dist: 3 },
            Point { x: 0, y: -4, dist: 4 },
            Point { x: 0, y: -5, dist: 5 },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn generate_path_works() {
        let p = Point { x:0, y:0, dist: 0};
        let dirs = vec![Dirs::D(2), Dirs::L(2)];

        let result = generate_path(&p, &dirs);

        let expected = vec![
            Point { x: 0, y: -1, dist: 1 },
            Point { x: 0, y: -2, dist: 2 },
            Point { x: -1, y: -2, dist: 3 },
            Point { x: -2, y: -2, dist: 4 },
        ];

        assert_eq!(result, expected);
    }
}