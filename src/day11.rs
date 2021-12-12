use crate::parsing;
use std::collections::HashSet;

pub fn run() {
    let octopi = Octopi::new(parse_energies(
        r"
        7313511551
        3724855867
        2374331571
        4438213437
        6511566287
        6727245532
        3736868662
        2348138263
        2417483121
        8812617112
    ",
    ));
    println!(
        "Day 11: After 100 days there have been {} flashes",
        octopi.take(100).sum::<usize>()
    );
}

pub struct Octopi {
    energies: Vec<Vec<u8>>,
}

impl Octopi {
    pub fn new(energies: Vec<Vec<u8>>) -> Octopi {
        Self { energies }
    }
}

impl Iterator for Octopi {
    type Item = usize; // number of flashes in this iteration
    fn next(&mut self) -> Option<Self::Item> {
        self.energies
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|e| *e += 1);
        let mut who_flashed = HashSet::new();
        loop {
            let mut bump_neighbors = Vec::new();
            let mut num_flashed_in_step = 0;
            self.energies
                .iter()
                .enumerate()
                .flat_map(|(row, entries)| {
                    entries
                        .iter()
                        .enumerate()
                        .map(move |(col, e)| (row, col, e))
                })
                .filter(|(_row, _col, e)| **e >= 10)
                .for_each(|(row, col, _e)| {
                    if who_flashed.contains(&(row, col)) {
                        return;
                    }
                    who_flashed.insert((row, col));
                    num_flashed_in_step += 1;
                    let row = row as i32;
                    let col = col as i32;
                    let mut neighbors = Vec::new();
                    neighbors.push((row - 1, col - 1));
                    neighbors.push((row - 1, col));
                    neighbors.push((row - 1, col + 1));
                    neighbors.push((row, col - 1));
                    neighbors.push((row, col));
                    neighbors.push((row, col + 1));
                    neighbors.push((row + 1, col - 1));
                    neighbors.push((row + 1, col));
                    neighbors.push((row + 1, col + 1));
                    neighbors
                        .iter()
                        .filter(|(r, c)| {
                            *r >= 0
                                && *r < self.energies.len() as i32
                                && *c >= 0
                                && *c < self.energies[*r as usize].len() as i32
                        })
                        .for_each(|(r, c)| bump_neighbors.push((*r as usize, *c as usize)));
                });
            bump_neighbors
                .iter()
                .for_each(|(row, col)| self.energies[*row][*col] += 1);
            if num_flashed_in_step <= 0 {
                break;
            }
        }

        self.energies
            .iter_mut()
            .flat_map(|entries| entries.iter_mut())
            .filter(|e| **e > 9)
            .for_each(|e| *e = 0);
        Some(who_flashed.len())
    }
}

fn parse_energies(s: &str) -> Vec<Vec<u8>> {
    let mut rows = Vec::new();
    parsing::parse_input(s, |parts| {
        rows.push(
            parts[0]
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect(),
        );
    });
    rows
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_octopi_flashes_small() {
        let mut octopi = Octopi::new(parse_energies(
            r"
        11111
        19991
        19191
        19991
        11111
        ",
        ));

        assert_eq!(octopi.next(), Some(9));
        assert_eq!(octopi.next(), Some(0));
    }

    #[test]
    fn test_count_octopi_flashes_large() {
        let mut octopi = Octopi::new(parse_energies(
            r"
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        ",
        ));
        assert_eq!(octopi.next(), Some(0));
        assert_eq!(octopi.next(), Some(35));
        assert_eq!(octopi.next(), Some(45));
        assert_eq!(octopi.take(97).sum::<usize>(), 1656 - 45 - 35);
    }
}
