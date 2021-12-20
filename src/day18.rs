//! <https://adventofcode.com/2021/day/18>
//!
//! Should have stuck with the tree-based approach, would be more elegant and robust (although
//! parent references are really annoying with all the [std::rc::Rc]/[std::rc::Weak]/
//! [std::cell::RefCell] stuff).

use std::cmp;
use std::ops::{Range, RangeInclusive};

const INPUT: [&str; 100] = [
    "[1,[[3,6],[0,[6,3]]]]",
    "[[[5,2],[[5,0],6]],[6,[5,1]]]",
    "[[5,[[2,3],[7,1]]],[4,[9,[8,3]]]]",
    "[[8,[[3,4],[8,7]]],[[[4,0],[3,5]],[[0,1],6]]]",
    "[[1,[6,[9,0]]],[[7,[5,7]],[[8,9],3]]]",
    "[[[[6,7],[4,9]],7],9]",
    "[[7,3],[[8,9],[7,[4,2]]]]",
    "[[[4,[2,9]],[0,3]],[[4,[0,8]],[[4,4],3]]]",
    "[[[[6,9],9],8],[[[4,0],[1,6]],[4,[3,6]]]]",
    "[[4,[4,[3,3]]],[[2,1],[[6,1],[9,4]]]]",
    "[[5,[6,7]],[[[5,8],[4,3]],[4,[0,8]]]]",
    "[[6,[[9,6],5]],[0,[6,6]]]",
    "[[[[1,5],9],[[5,3],5]],[[[2,0],3],9]]",
    "[4,3]",
    "[[1,8],[[[1,0],[3,8]],3]]",
    "[[[[2,0],[6,5]],4],[[[9,8],[0,1]],3]]",
    "[[8,[7,8]],[[6,[3,2]],[[8,1],[7,5]]]]",
    "[[[[1,4],2],[[4,8],[3,2]]],[[[2,2],6],6]]",
    "[[[4,[0,5]],[[8,8],[7,2]]],[[4,[4,1]],2]]",
    "[[1,[4,[4,0]]],[2,[[2,3],1]]]",
    "[[[[2,1],0],[[3,4],1]],[[2,4],3]]",
    "[[9,[8,7]],[7,[0,[8,0]]]]",
    "[[[9,9],7],[[0,[2,1]],[[7,1],4]]]",
    "[[6,[[3,2],[0,0]]],[[[4,1],9],[7,3]]]",
    "[[[5,[5,6]],[[7,7],[7,8]]],4]",
    "[[8,[[4,1],4]],[[[6,4],[0,3]],[5,[6,4]]]]",
    "[[[9,0],[2,8]],[[6,5],5]]",
    "[[[3,[1,6]],[[5,3],6]],[[[7,4],[4,9]],[[2,3],[6,5]]]]",
    "[[8,[6,7]],6]",
    "[[[[6,0],[1,3]],[0,0]],[[[4,7],[7,8]],[[7,2],2]]]",
    "[[6,[[9,6],[1,1]]],7]",
    "[[[2,3],[6,0]],[3,[[9,3],9]]]",
    "[[[3,0],0],[[[6,0],3],[[1,5],4]]]",
    "[[8,[[0,3],8]],[[[0,8],[4,3]],[8,[3,4]]]]",
    "[[[4,4],0],[[1,[8,0]],[[9,6],3]]]",
    "[8,[[6,[6,7]],[8,7]]]",
    "[[8,[0,[1,4]]],3]",
    "[[[[9,5],0],[[5,3],[1,9]]],[[[7,3],5],[[4,3],9]]]",
    "[[[[9,0],[4,2]],[0,[3,2]]],1]",
    "[[[6,[4,2]],[[5,5],9]],[[[6,1],9],[[3,8],[8,1]]]]",
    "[[[3,[5,0]],[[5,2],[2,2]]],[[0,2],[7,4]]]",
    "[[3,[[5,7],[2,8]]],4]",
    "[[[4,8],5],0]",
    "[[[6,9],[[7,0],7]],[8,7]]",
    "[[7,[[1,3],[0,2]]],[[[4,8],0],[[7,0],6]]]",
    "[[[1,7],[6,6]],[[6,[4,0]],[0,4]]]",
    "[[[[2,2],[3,9]],9],3]",
    "[0,[[4,9],[[5,5],[5,9]]]]",
    "[[[[4,4],2],[6,4]],[[[4,1],[2,0]],[[9,4],0]]]",
    "[[[0,[3,4]],[2,3]],[[7,[2,3]],[3,3]]]",
    "[[[[0,3],9],7],[7,[4,[9,6]]]]",
    "[[9,[[0,8],4]],[5,[2,[4,9]]]]",
    "[[6,2],[[1,7],0]]",
    "[[[[1,6],[8,3]],1],[[[6,7],2],[[4,4],8]]]",
    "[[[[7,1],[0,3]],0],[5,[4,[8,4]]]]",
    "[[[[4,2],[6,2]],[[5,7],8]],[[7,9],4]]",
    "[[[0,[0,4]],5],2]",
    "[[2,[[0,6],6]],[[[3,4],3],[4,5]]]",
    "[[[[6,4],9],[[7,1],0]],[[[8,2],[3,2]],[[1,9],7]]]",
    "[7,[[7,8],[[5,5],0]]]",
    "[[[1,2],[8,5]],[[5,4],[8,0]]]",
    "[[4,[1,3]],[[[4,5],[1,2]],[5,1]]]",
    "[[[[0,7],[4,5]],[9,[2,2]]],[4,[[1,8],[7,5]]]]",
    "[[4,[[0,4],[8,8]]],[[[9,2],[7,1]],[8,[9,5]]]]",
    "[1,3]",
    "[[[[8,9],5],0],[1,6]]",
    "[[[[6,6],[3,5]],[[2,8],[3,3]]],[[[5,3],[5,9]],[0,[1,4]]]]",
    "[[7,[7,[5,5]]],[4,[4,[9,9]]]]",
    "[[[7,[6,7]],[4,2]],[0,[[7,8],1]]]",
    "[[[[6,0],4],[3,[6,9]]],5]",
    "[[[[9,8],6],[[7,4],[3,4]]],[[[8,8],[2,1]],0]]",
    "[9,[[1,7],[7,1]]]",
    "[6,[[3,[3,6]],[2,9]]]",
    "[[[6,9],[[1,4],2]],[7,3]]",
    "[[1,[6,[8,5]]],[[[0,0],0],[7,2]]]",
    "[[[4,[2,7]],[[0,0],8]],[[4,[4,5]],[[4,8],[3,3]]]]",
    "[[[[7,4],[7,5]],[[5,8],3]],[[[6,9],[0,9]],[[7,2],[4,0]]]]",
    "[4,[4,[9,[5,7]]]]",
    "[[[[8,7],3],[6,[0,5]]],[[[7,8],[5,1]],[[0,4],2]]]",
    "[6,[0,[4,3]]]",
    "[[[[6,5],3],7],[[[0,1],5],[6,[2,6]]]]",
    "[[[9,1],[[8,8],[8,2]]],0]",
    "[[[[3,4],1],3],[8,[[1,5],[5,6]]]]",
    "[[[[6,8],2],4],[[5,8],[[1,5],[7,0]]]]",
    "[[3,8],[[[9,0],2],[7,[5,8]]]]",
    "[[[[7,5],6],[[4,4],[5,0]]],[4,[3,[3,0]]]]",
    "[[[7,9],[1,[8,8]]],[[[6,8],4],4]]",
    "[[4,[[6,7],[5,7]]],[6,7]]",
    "[[[[8,8],[0,4]],[[5,5],1]],6]",
    "[[[7,7],[[5,8],[3,4]]],[[0,[7,4]],5]]",
    "[8,[[1,2],[6,9]]]",
    "[[[[9,5],[0,6]],[2,[8,7]]],[[[9,2],4],6]]",
    "[[[1,[5,2]],5],[[1,0],3]]",
    "[[7,[7,[3,7]]],[[2,6],3]]",
    "[1,[[8,[7,1]],[3,8]]]",
    "[[[[3,2],[5,6]],2],[7,[0,0]]]",
    "[[[7,[4,6]],[9,[7,8]]],9]",
    "[[[[4,3],9],8],[[8,5],6]]",
    "[3,[[3,1],[[8,4],8]]]",
    "[[[9,[3,5]],[[0,9],[8,5]]],5]",
];

struct SnailfishNumber(String);

impl SnailfishNumber {
    fn first_number_left_of(&self, i: usize) -> Option<RangeInclusive<usize>> {
        let value_end_i = self.0[..i].rfind(|c: char| c.is_ascii_digit())?;
        let value_start_i = self.0[..=value_end_i]
            .rfind(|c: char| !c.is_ascii_digit())
            .unwrap()
            + 1;
        Some(value_start_i..=value_end_i)
    }

    fn first_number_right_of(&self, i: usize) -> Option<RangeInclusive<usize>> {
        let value_start_i = i + 1 + self.0[i + 1..].find(|c: char| c.is_ascii_digit())?;
        let value_end_i = value_start_i
            + self.0[value_start_i..]
                .find(|c: char| !c.is_ascii_digit())
                .unwrap()
            - 1;
        Some(value_start_i..=value_end_i)
    }

    fn explode(&mut self, pair_to_explode: RangeInclusive<usize>) {
        let comma_i =
            pair_to_explode.start() + self.0[*pair_to_explode.start()..].find(',').unwrap();
        let left_value: u32 = self.0[pair_to_explode.start() + 1..comma_i]
            .parse()
            .unwrap();
        let right_value: u32 = self.0[comma_i + 1..*pair_to_explode.end()].parse().unwrap();

        self.0.replace_range(pair_to_explode.clone(), "0");

        if let Some(number_left_of_pair) = self.first_number_left_of(*pair_to_explode.start()) {
            let old_value = self.0[number_left_of_pair.clone()].parse::<u32>().unwrap();
            let new_value = old_value + left_value;
            self.0
                .replace_range(number_left_of_pair, &new_value.to_string());
        }

        // XXX This is ugly and fragile: indices in the string changed after the previous
        //     [String::replace_range] calls (the main problem is that adding to
        //     `number_left_of_pair` may have made it two-digit); so the `+ 1` in
        //     `*pair_to_explode.start() + 1` is required for this to work, and it only works
        //     when there are no three-or-more-digit numbers.
        if let Some(number_right_of_pair) = self.first_number_right_of(*pair_to_explode.start() + 1)
        {
            let old_value = self.0[number_right_of_pair.clone()].parse::<u32>().unwrap();
            let new_value = old_value + right_value;
            self.0
                .replace_range(number_right_of_pair, &new_value.to_string());
        }
    }

    fn split(&mut self, number_to_split: Range<usize>) {
        let old_value = self.0[number_to_split.clone()].parse::<u32>().unwrap();
        let new_pair = format!(
            "[{},{}]",
            (old_value as f64 / 2.0).floor(),
            (old_value as f64 / 2.0).ceil()
        );
        self.0.replace_range(number_to_split, &new_pair);
    }

    /// Explodes the leftmost pair that needs exploding and returns `true`, or if there is no such
    /// pair, doesn't do anything and returns `false`.
    fn do_explode_action(&mut self) -> bool {
        let mut depth = 0;
        for (i, c) in self.0.char_indices() {
            match c {
                '[' if depth == 4 => {
                    let pair_to_explode_end_i = i + self.0[i..].find(']').unwrap();
                    self.explode(i..=pair_to_explode_end_i);
                    return true;
                }
                '[' => {
                    depth += 1;
                }
                ']' => {
                    depth -= 1;
                }
                _ => (),
            }
        }
        false
    }

    /// Splits the leftmost number that needs splitting and returns `true`, or if there is no such
    /// number, doesn't do anything and returns `false`.
    fn do_split_action(&mut self) -> bool {
        for (i, c) in self.0.char_indices() {
            match c {
                c if c.is_ascii_digit() => {
                    let value_end_i = i + self.0[i..].find(|c: char| !c.is_ascii_digit()).unwrap();
                    let value: u32 = self.0[i..value_end_i].parse().unwrap();
                    if value > 9 {
                        self.split(i..value_end_i);
                        return true;
                    }
                }
                _ => (),
            }
        }
        false
    }

    fn reduce(&mut self) {
        loop {
            if self.do_explode_action() {
                continue;
            }
            if self.do_split_action() {
                continue;
            }
            break;
        }
    }

    fn magnitude(&self) -> u32 {
        /// Given an index to start of a pair or a number, returns a tuple where:
        /// * First element is the magnitude of the pair or number starting at the given index.
        /// * Second element is the index to first character after the processed pair or number.
        fn dfs_traversal(i: usize, sfn: &SnailfishNumber) -> (usize, u32) {
            match &sfn.0[i..=i] {
                "[" => {
                    let (i_after_left_element, left_magnitude) = dfs_traversal(i + 1, sfn);
                    let (i_after_right_element, right_magnitude) =
                        dfs_traversal(i_after_left_element + 1, sfn);
                    let magnitude = 3 * left_magnitude + 2 * right_magnitude;
                    (i_after_right_element + 1, magnitude)
                }
                _ => {
                    let number_end_i = i + sfn.0[i..].find(|c: char| !c.is_ascii_digit()).unwrap();
                    let magnitude = sfn.0[i..number_end_i].parse().unwrap();
                    (number_end_i, magnitude)
                }
            }
        }

        dfs_traversal(0, self).1
    }
}

fn sum(sfn1: &SnailfishNumber, sfn2: &SnailfishNumber) -> SnailfishNumber {
    let mut sum = SnailfishNumber(format!("[{},{}]", sfn1.0, sfn2.0));
    sum.reduce();
    sum
}

pub fn solution_1() -> String {
    let sum = INPUT
        .iter()
        .map(|&s| SnailfishNumber(String::from(s)))
        .reduce(|acc, sfn| sum(&acc, &sfn))
        .unwrap();

    sum.magnitude().to_string()
}

pub fn solution_2() -> String {
    let snailfish_numbers: Vec<SnailfishNumber> = INPUT
        .iter()
        .map(|&s| SnailfishNumber(String::from(s)))
        .collect();

    let mut max_magnitude = 0;
    for i1 in 0..snailfish_numbers.len() {
        for i2 in 0..snailfish_numbers.len() {
            if i1 == i2 {
                continue;
            }

            let sum = sum(&snailfish_numbers[i1], &snailfish_numbers[i2]);
            max_magnitude = cmp::max(max_magnitude, sum.magnitude());
        }
    }
    max_magnitude.to_string()
}

// Failed second version: tree without parent references
//struct SnailfishNumber {
//    left_child: Box<Child>,
//    right_child: Box<Child>,
//}
//
//enum Child {
//    Leaf(u32),
//    InnerNode(SnailfishNumber),
//}
//
//impl FromStr for SnailfishNumber {
//    type Err = String;
//
//    fn from_str(s: &str) -> Result<Self, Self::Err> {
//        let mut top_level_comma_index = None;
//        let mut num_open_parens = 0;
//        for (i, c) in s.char_indices().skip(1) {
//            match c {
//                '[' => num_open_parens += 1,
//                ']' => num_open_parens -= 1,
//                ',' if num_open_parens == 0 => {
//                    top_level_comma_index = Some(i);
//                    break;
//                }
//                _ => (),
//            }
//        }
//
//        let s_left_child = &s[1..top_level_comma_index.unwrap()];
//        let s_right_child = &s[top_level_comma_index.unwrap() + 1..s.len() - 1];
//
//        let left_child = match s_left_child.len() {
//            1 => Child::Leaf(s_left_child.parse().unwrap()),
//            _ => Child::InnerNode(SnailfishNumber::from_str(s_left_child).unwrap()),
//        };
//        let right_child = match s_right_child.len() {
//            1 => Child::Leaf(s_right_child.parse().unwrap()),
//            _ => Child::InnerNode(SnailfishNumber::from_str(s_right_child).unwrap()),
//        };
//
//        Ok(SnailfishNumber {
//            left_child: Box::new(left_child),
//            right_child: Box::new(right_child),
//        })
//    }
//}
//
//impl SnailfishNumber {
//    // tries 1 reduce action, returns true if reduced, false if nothing to be done TODO words
//    fn try_reduce_step(&mut self) -> bool {
//        // TODO explain this is a sliding window over leaves from left to right
//        let mut last_seen_leaves = [None, None, None];
//        let mut to_be_visited = Vec::from([&self.right_child, &self.left_child]);
//        loop {
//            match to_be_visited.pop() {
//                None => break false,
//                Some(&child) => match *child {
//                    Child::InnerNode(SnailfishNumber{left_child, right_child}) => {
//                        to_be_visited.push(&right_child);
//                        to_be_visited.push(&left_child);
//                    },
//                    Child::Leaf(value) => {
//                        last_seen_leaves.rotate_left(1);
//                        last_seen_leaves[2] = Some(child);
//
//                        //TODO check for split here... BUT FORGOT ABOUT EXPLOSIONS!
//                    },
//                }
//            }
//        }
//    }
//}

// Failed first version: tree with parent references (and top-level enum)
//enum SnailfishNumber {
//    Leaf {
//    value: u32,
//        parent: RefCell<Weak<SnailfishNumber>>,
//    },
//    Node {
//        left_child: RefCell<Rc<SnailfishNumber>>,
//        right_child: RefCell<Rc<SnailfishNumber>>,
//        parent: RefCell<Weak<SnailfishNumber>>,
//    },
//}
//
//impl SnailfishNumber {
//    fn parent(&self) -> &RefCell<Weak<SnailfishNumber>> {
//        match self {
//            SnailfishNumber::Leaf { parent, .. } => parent,
//            SnailfishNumber::Node { parent, .. } => parent,
//        }
//    }
//
//    fn left_child(&self) -> &RefCell<Rc<SnailfishNumber>> {
//        match self {
//            SnailfishNumber::Leaf { .. } => panic!("TODO msg"),
//            SnailfishNumber::Node { left_child, .. } => left_child,
//        }
//    }
//
//    fn right_child(&self) -> &RefCell<Rc<SnailfishNumber>> {
//        match self {
//            SnailfishNumber::Leaf { .. } => panic!("TODO msg"),
//            SnailfishNumber::Node { right_child, .. } => right_child,
//        }
//    }
//
//    fn from_str(s: &str) -> Rc<SnailfishNumber> {
//        if s.len() == 1 {
//            Rc::new(SnailfishNumber::Leaf {
//                value: s.parse().unwrap(),
//                parent: RefCell::new(Weak::new()),
//            })
//        } else {
//            let s_trimmed = s.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
//
//            let mut left_child = Option::None;
//            let mut right_child = Option::None;
//            let mut num_open_parens = 0;
//            for (i, c) in s_trimmed.char_indices() {
//                match c {
//                    '[' => num_open_parens += 1,
//                    ']' => num_open_parens -= 1,
//                    ',' if num_open_parens == 0 => {
//                        left_child = Some(SnailfishNumber::from_str(&s_trimmed[0..i]));
//                        right_child = Some(SnailfishNumber::from_str(&s_trimmed[i + 1..]));
//                    }
//                    _ => (),
//                }
//            }
//
//            let node = Rc::new(SnailfishNumber::Node {
//                left_child: RefCell::new(left_child.unwrap()),
//                right_child: RefCell::new(right_child.unwrap()),
//                parent: RefCell::new(Weak::new()),
//            });
//
//            *node.left_child().borrow().parent().borrow_mut() = Rc::downgrade(&node);
//            *node.right_child().borrow().parent().borrow_mut() = Rc::downgrade(&node);
//
//            node
//        }
//    }
//}
