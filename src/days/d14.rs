/// Advent of Code 2019, day 14
/// https://adventofcode.com/2019/day/14
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;
use std::str::FromStr;

fn div_round_up(n: u64, d: u64) -> u64 {
    if n % d != 0 {
        n / d + 1
    } else {
        n / d
    }
}

#[derive(Debug, Clone)]
struct Material {
    material: String,
    quantity: u64,
}

impl Material {
    fn new(material: String, quantity: u64) -> Self {
        Self { material, quantity }
    }
}

impl FromStr for Material {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        if split.len() != 2 {
            return Err(format!("{:?} must have only two parts.", s));
        }
        let quantity = split[0]
            .parse::<u64>()
            .map_err(|e| format!("Error parsing {:?}: {}", s, e))?;
        let material = split[1].to_string();
        Ok(Self { material, quantity })
    }
}

#[derive(Debug, Clone)]
struct Recipe {
    input: Vec<Material>,
    output: Material,
}

impl FromStr for Recipe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" => ").collect();
        if parts.len() != 2 {
            return Err(format!(
                "Recipe must be in form '0 AAA, ... => 0 BBB', but got {:?}",
                s
            ));
        }
        let input = parts[0]
            .split(", ")
            .map(|i| i.parse::<Material>())
            .collect::<Result<Vec<Material>, String>>()?;
        let output = parts[1].parse::<Material>()?;
        Ok(Self { input, output })
    }
}

#[derive(Debug, Clone)]
struct Collection {
    recipes: Vec<Recipe>,
}

impl Collection {
    fn new(recipes: Vec<Recipe>) -> Self {
        Self { recipes }
    }

    fn find<'a>(&self, material: &String) -> Option<usize> {
        self.recipes
            .iter()
            .enumerate()
            .filter_map(|(i, r)| {
                if &r.output.material == material {
                    Some(i)
                } else {
                    None
                }
            })
            .nth(0)
    }

    fn base(&self) -> Vec<String> {
        let mut base: Vec<String> = Vec::new();
        for r in self.recipes.iter() {
            for m in r.input.iter() {
                if self.find(&m.material).is_none() {
                    base.push(m.material.clone());
                }
            }
        }
        base
    }

    fn sorted<'a>(&self) -> Vec<String> {
        let mut sorted = Vec::new();
        let mut base: VecDeque<String> = VecDeque::from_iter(self.base().iter().cloned());
        let mut g = self.clone();
        while let Some(s) = base.pop_back() {
            if sorted.contains(&s) {
                continue;
            }
            sorted.push(s.clone());
            let size = g.recipes.len();
            for i in (0..size).rev() {
                let r = &mut g.recipes[i];
                for j in (0..r.input.len()).rev() {
                    if r.input[j].material == s {
                        r.input.remove(j);
                    }
                }
                if r.input.len() == 0 {
                    base.push_front(r.output.material.clone());
                    g.recipes.remove(i);
                }
            }
        }
        if g.recipes.len() > 0 {
            panic!("DAG not consumed while sorting; may be a cycle somewhere.");
        }
        sorted
    }

    fn find_total_raw_from_sort(&self, sorted: &Vec<String>, material: &Material) -> Vec<Material> {
        let base = self.base();
        let mut materials: HashMap<&String, u64> = HashMap::new();
        materials.insert(&material.material, material.quantity);

        for m in sorted.iter().rev() {
            if base.contains(m) {
                continue;
            }
            if let Some(i) = self.find(m) {
                let r = &self.recipes[i];
                let multi = div_round_up(
                    materials.get(&r.output.material).unwrap().clone(),
                    r.output.quantity,
                );
                for i in r.input.iter() {
                    *materials.entry(&i.material).or_insert(0) += i.quantity * multi;
                }
            }
        }

        materials
            .iter()
            .filter_map(|(&m, q)| {
                if base.contains(m) {
                    Some(Material::new(m.clone(), q.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    fn total_raw(&self, material: &Material) -> Vec<Material> {
        let sorted = self.sorted();
        self.find_total_raw_from_sort(&sorted, material)
    }
}

fn parse_input<'a>() -> Collection {
    let data = include_bytes!("input/d14.txt");
    let recipes = String::from_utf8_lossy(data)
        .split("\n")
        .map(|l| l.parse::<Recipe>())
        .collect::<Result<Vec<Recipe>, String>>()
        .unwrap();
    Collection::new(recipes)
}

pub fn d14a() -> String {
    let result = parse_input().total_raw(&Material::new("FUEL".to_string(), 1));
    result[0].quantity.to_string()
}

pub fn d14b() -> String {
    let collection = &parse_input();
    let sorted = collection.sorted();
    let fuel = "FUEL".to_string();
    let result = collection.find_total_raw_from_sort(&sorted, &Material::new(fuel.clone(), 1));

    let total: u64 = 1000000000000;
    let mut min = total / result[0].quantity;
    let mut max = min * 2;

    while max - min > 1 {
        let mid = (min + max) / 2;
        let result =
            collection.find_total_raw_from_sort(&sorted, &Material::new(fuel.clone(), mid));
        let required = result[0].quantity;
        if required > total {
            max = mid;
        } else if required < total {
            min = mid;
        } else {
            min = mid;
            max = mid;
        }
    }

    min.to_string()
}
