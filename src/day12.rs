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

    pub fn can_revisit(node: &str) -> bool {
        node.chars().nth(0).unwrap().is_uppercase()
    }

    pub fn walk(self: &Self, start: &str, end: &str) -> Vec<Vec<String>> {
        self.walk_internal(&vec![start.to_string()], end)
    }

    pub fn walk_internal(self: &Self, crumbs: &Vec<String>, end: &str) -> Vec<Vec<String>> {
        let last = crumbs.last().unwrap();
        if last == end {
            return vec![crumbs.clone()];
        }
        match self.connections.get(last) {
            Some(nexts) => nexts
                .iter()
                .filter(|next| !crumbs.contains(next) || Caves::can_revisit(next))
                .flat_map(|next| {
                    let mut new_crumbs = crumbs.clone();
                    new_crumbs.push(next.to_string());
                    self.walk_internal(&new_crumbs, end)
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
}
