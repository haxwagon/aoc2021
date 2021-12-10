use crate::parsing;

pub fn run() {
    let (_, num_lanternfish) = simulate_lanternfish(&get_lanternfish(), 80);
    println!("Day  6: There will be {} lanternfish after 80 days", num_lanternfish);
}

/// Returns (number of fish by days until new gen, total number of fish)
pub fn simulate_lanternfish(fish : &[u64;9], num_days: u32) -> ([u64;9], u64) {
    let mut new_fish = [0;9];
    for i in 0..9 { new_fish[i] = fish[i] }

    for _ in 0..num_days {
        let num_reproducing = new_fish[0];
        for i in 0..8 { new_fish[i] = new_fish[i+1]; }
        new_fish[8] = num_reproducing;
        new_fish[6] += num_reproducing;
    }

    let mut num_fish = 0;
    for i in 0..9 { num_fish += new_fish[i] }
    (new_fish, num_fish)
}

fn parse_lanternfish(s: &str) -> [u64;9] {
    let mut fish = [0;9];
    parsing::parse_input(s, |parts| {
        parts[0].split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .for_each(|x| {
                if x > 8 { panic!("Invalid fish! timer={}", x) }
                fish[x as usize] += 1;
            })
    });
    fish
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lanternfish() {
        let fish = parse_lanternfish("3,4,3,1,2");
        {
            let (_, num_fish) = simulate_lanternfish(&fish, 18);
            assert_eq!(num_fish, 26);
        }
        {
            let (_, num_fish) = simulate_lanternfish(&fish, 80);
            assert_eq!(num_fish, 5934);
        }
    }
}

fn get_lanternfish() -> [u64;9] {
    parse_lanternfish(r"
1,1,3,5,3,1,1,4,1,1,5,2,4,3,1,1,3,1,1,5,5,1,3,2,5,4,1,1,5,1,4,2,1,4,2,1,4,4,1,5,1,4,4,1,1,5,1,5,1,5,1,1,1,5,1,2,5,1,1,3,2,2,2,1,4,1,1,2,4,1,3,1,2,1,3,5,2,3,5,1,1,4,3,3,5,1,5,3,1,2,3,4,1,1,5,4,1,3,4,4,1,2,4,4,1,1,3,5,3,1,2,2,5,1,4,1,3,3,3,3,1,1,2,1,5,3,4,5,1,5,2,5,3,2,1,4,2,1,1,1,4,1,2,1,2,2,4,5,5,5,4,1,4,1,4,2,3,2,3,1,1,2,3,1,1,1,5,2,2,5,3,1,4,1,2,1,1,5,3,1,4,5,1,4,2,1,1,5,1,5,4,1,5,5,2,3,1,3,5,1,1,1,1,3,1,1,4,1,5,2,1,1,3,5,1,1,4,2,1,2,5,2,5,1,1,1,2,3,5,5,1,4,3,2,2,3,2,1,1,4,1,3,5,2,3,1,1,5,1,3,5,1,1,5,5,3,1,3,3,1,2,3,1,5,1,3,2,1,3,1,1,2,3,5,3,5,5,4,3,1,5,1,1,2,3,2,2,1,1,2,1,4,1,2,3,3,3,1,3,5
    ")
}