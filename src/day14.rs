//! <https://adventofcode.com/2021/day/14>

use std::collections::HashMap;
use std::str::FromStr;

const INPUT_TEMPLATE: &str = "OOVSKSPKPPPNNFFBCNOV";

const INPUT_RULES: [&str; 100] = [
    "BC -> C", "PP -> O", "SK -> K", "KH -> N", "OK -> S", "PC -> O", "VP -> K", "CF -> K",
    "HC -> H", "FV -> V", "PB -> P", "NK -> H", "CK -> F", "FH -> H", "SV -> B", "NH -> C",
    "CP -> S", "HP -> O", "HS -> O", "BK -> B", "KC -> P", "VV -> B", "OF -> O", "KP -> V",
    "FO -> V", "FK -> V", "VH -> K", "KB -> P", "KF -> H", "SH -> S", "HF -> O", "BB -> F",
    "FC -> O", "SO -> S", "BS -> O", "HH -> C", "BO -> S", "CO -> F", "VC -> V", "KS -> N",
    "OC -> N", "FP -> P", "HN -> B", "HV -> V", "HO -> P", "KO -> C", "SF -> H", "NO -> N",
    "PS -> C", "BP -> K", "SC -> C", "NP -> C", "CH -> V", "KV -> B", "HK -> V", "OP -> V",
    "SP -> V", "NC -> V", "FF -> B", "CC -> V", "CS -> F", "SB -> C", "OS -> C", "FN -> O",
    "CV -> P", "OH -> H", "OO -> P", "PO -> F", "NS -> H", "VB -> K", "OV -> K", "PH -> H",
    "BH -> V", "SS -> B", "PK -> F", "VK -> O", "BN -> V", "VF -> O", "PF -> H", "VS -> K",
    "ON -> V", "BF -> F", "CN -> F", "VO -> B", "FS -> K", "OB -> B", "PN -> H", "NF -> O",
    "VN -> P", "BV -> S", "NV -> V", "FB -> V", "NB -> P", "CB -> B", "KK -> S", "NN -> F",
    "SN -> B", "HB -> P", "PV -> S", "KN -> S",
];

struct InsertionRule {
    start_pair: (char, char),
    element_to_insert: char,
}

struct Polymer {
    element_pairs_quantities: HashMap<(char, char), u128>,
    /// First working solution did not have this field. Element quantities can *almost* be
    /// calculated from [Polymer::element_pairs_quantities] - almost, but not quite, because all
    /// elements are represented in 2 pairs, except for the first and last element. So extra data
    /// are needed. And instead of storing the first/last element, it's more elegant to just store
    /// all the counts - as seen on Reddit.
    elements_quantities: HashMap<char, u128>,
}

impl FromStr for InsertionRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_split: Vec<&str> = s.split_whitespace().collect();
        let start_pair = (
            s_split[0].chars().collect::<Vec<_>>()[0],
            s_split[0].chars().collect::<Vec<_>>()[1],
        );
        let element_to_insert = s_split[2].chars().next().unwrap();
        Ok(InsertionRule {
            start_pair,
            element_to_insert,
        })
    }
}

impl FromStr for Polymer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut element_pairs_quantities = HashMap::new();
        for element_pair in s.chars().zip(s.chars().skip(1)) {
            *element_pairs_quantities.entry(element_pair).or_insert(0) += 1;
        }
        let mut elements_quantities = HashMap::new();
        for element in s.chars() {
            *elements_quantities.entry(element).or_insert(0) += 1;
        }
        Ok(Polymer {
            element_pairs_quantities,
            elements_quantities,
        })
    }
}

impl Polymer {
    fn do_pair_insertion(&mut self, rules: &[InsertionRule]) {
        let mut new_element_pairs_quantities = HashMap::new();
        let mut new_elements_quantities = self.elements_quantities.clone();

        for (&pair, &quantity) in &self.element_pairs_quantities {
            if let Some(rule) = rules.iter().find(|r| r.start_pair == pair) {
                *new_element_pairs_quantities
                    .entry((pair.0, rule.element_to_insert))
                    .or_insert(0) += quantity;
                *new_element_pairs_quantities
                    .entry((rule.element_to_insert, pair.1))
                    .or_insert(0) += quantity;

                *new_elements_quantities
                    .entry(rule.element_to_insert)
                    .or_insert(0) += quantity;
            }
        }

        self.element_pairs_quantities = new_element_pairs_quantities;
        self.elements_quantities = new_elements_quantities;
    }
}

pub fn solution_1() -> String {
    let rules: Vec<InsertionRule> = INPUT_RULES
        .iter()
        .map(|&r| InsertionRule::from_str(r).unwrap())
        .collect();
    let mut polymer = Polymer::from_str(INPUT_TEMPLATE).unwrap();

    for _ in 0..10 {
        polymer.do_pair_insertion(&rules);
    }

    let most_common_element_quantity = polymer.elements_quantities.values().max().unwrap();
    let least_common_element_quantity = polymer.elements_quantities.values().min().unwrap();
    (most_common_element_quantity - least_common_element_quantity).to_string()
}

pub fn solution_2() -> String {
    let rules: Vec<InsertionRule> = INPUT_RULES
        .iter()
        .map(|&r| InsertionRule::from_str(r).unwrap())
        .collect();
    let mut polymer = Polymer::from_str(INPUT_TEMPLATE).unwrap();

    for _ in 0..40 {
        polymer.do_pair_insertion(&rules);
    }

    let most_common_element_quantity = polymer.elements_quantities.values().max().unwrap();
    let least_common_element_quantity = polymer.elements_quantities.values().min().unwrap();
    (most_common_element_quantity - least_common_element_quantity).to_string()
}
