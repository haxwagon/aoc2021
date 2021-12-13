use crate::parsing;
use std::collections::HashMap;

pub fn run() {
    let caves = Caves::new(parse_caves(
        r"
        re-js
        qx-CG
        start-js
        start-bj
        qx-ak
        js-bj
        ak-re
        CG-ak
        js-CG
        bj-re
        ak-lg
        lg-CG
        qx-re
        WP-ak
        WP-end
        re-lg
        end-ak
        WP-re
        bj-CG
        qx-start
        bj-WP
        JG-lg
        end-lg
        lg-iw
    ",
    ));

    println!(
        "Day 12: There are {} ways through the cave",
        caves.walk("start", "end").len()
    );
    println!(
        "      : There are {} ways to walk through the cave with up to 1 small cave revisited",
        caves.walk_leisurely("start", "end").len()
    );
}

pub struct Caves {
    connections: HashMap<String, Vec<String>>,
}

impl Caves {
    pub fn new(d: Vec<(String, String)>) -> Self {
        let mut connections = HashMap::<String, Vec<String>>::new();
        d.iter()
            .flat_map(|(l, r)| vec![(l, r), (r, l)])
            .for_each(|(from, to)| {
                connections
                    .entry(from.clone())
                    .or_default()
                    .push(to.clone());
            });
        Self { connections }
    }

    pub fn can_visit(
        node: &str,
        crumbs: &Vec<String>,
        start: &str,
        end: &str,
        max_small_revisits: u32,
    ) -> bool {
        let mut num_visits = HashMap::<String, u32>::new();
        crumbs.iter().for_each(|crumb| {
            *num_visits.entry(crumb.to_string()).or_default() += 1;
        });
        let node_visits = *num_visits.entry(node.to_string()).or_default();
        let num_revisited = num_visits
            .iter()
            .filter(|(_, count)| **count > 1)
            .filter(|(x, _)| x.chars().nth(0).unwrap().is_lowercase())
            .count() as u32;
        if node == start {
            false
        } else if node == end {
            node_visits <= 0
        } else if node.chars().nth(0).unwrap().is_uppercase() {
            true
        } else if num_revisited >= max_small_revisits && node_visits > 0 {
            false
        } else {
            true
        }
    }

    pub fn walk(self: &Self, start: &str, end: &str) -> Vec<Vec<String>> {
        self.walk_internal(&vec![start.to_string()], start, end, 0)
    }

    pub fn walk_leisurely(self: &Self, start: &str, end: &str) -> Vec<Vec<String>> {
        self.walk_internal(&vec![start.to_string()], start, end, 1)
    }

    pub fn walk_internal(
        self: &Self,
        crumbs: &Vec<String>,
        start: &str,
        end: &str,
        max_small_revisits: u32,
    ) -> Vec<Vec<String>> {
        let last = crumbs.last().unwrap();
        if last == end {
            return vec![crumbs.clone()];
        }
        match self.connections.get(last) {
            Some(nexts) => nexts
                .iter()
                .filter(|next| Caves::can_visit(next, crumbs, start, end, max_small_revisits))
                .flat_map(|next| {
                    let mut new_crumbs = crumbs.clone();
                    new_crumbs.push(next.to_string());
                    self.walk_internal(&new_crumbs, start, end, max_small_revisits)
                })
                .collect(),
            None => Vec::new(),
        }
    }
}

fn parse_caves(s: &str) -> Vec<(String, String)> {
    parsing::parse_lines(s)
        .iter()
        .map(|s| {
            let mut parts = s.split("-");
            (
                parts.next().unwrap().to_string(),
                parts.next().unwrap().to_string(),
            )
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_walk_small_cave() {
        let caves = Caves::new(parse_caves(
            r"
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        ",
        ));
        assert_eq!(caves.walk("start", "end").len(), 10);
    }

    #[test]
    fn test_walk_medium_cave() {
        let caves = Caves::new(parse_caves(
            r"
            dc-end
            HN-start
            start-kj
            dc-start
            dc-HN
            LN-dc
            HN-end
            kj-sa
            kj-HN
            kj-dc
        ",
        ));
        assert_eq!(caves.walk("start", "end").len(), 19);
    }

    #[test]
    fn test_walk_large_cave() {
        let caves = Caves::new(parse_caves(
            r"
            fs-end
            he-DX
            fs-he
            start-DX
            pj-DX
            end-zg
            zg-sl
            zg-pj
            pj-he
            RW-he
            fs-DX
            pj-RW
            zg-RW
            start-pj
            he-WI
            zg-he
            pj-fs
            start-RW
        ",
        ));
        assert_eq!(caves.walk("start", "end").len(), 226);
    }

    #[test]
    fn test_walk_leisurely_small_cave() {
        let caves = Caves::new(parse_caves(
            r"
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        ",
        ));
        let walks = caves.walk_leisurely("start", "end");
        walks.iter().for_each(|walk| println!("{:?}", walk));
        assert_eq!(caves.walk_leisurely("start", "end").len(), 36);
    }

    #[test]
    fn test_walk_leisurely_medium_cave() {
        let caves = Caves::new(parse_caves(
            r"
            dc-end
            HN-start
            start-kj
            dc-start
            dc-HN
            LN-dc
            HN-end
            kj-sa
            kj-HN
            kj-dc
        ",
        ));
        assert_eq!(caves.walk_leisurely("start", "end").len(), 103);
    }

    #[test]
    fn test_walk_leisurely_large_cave() {
        let caves = Caves::new(parse_caves(
            r"
            fs-end
            he-DX
            fs-he
            start-DX
            pj-DX
            end-zg
            zg-sl
            zg-pj
            pj-he
            RW-he
            fs-DX
            pj-RW
            zg-RW
            start-pj
            he-WI
            zg-he
            pj-fs
            start-RW
        ",
        ));
        assert_eq!(caves.walk_leisurely("start", "end").len(), 3509);
    }
}
