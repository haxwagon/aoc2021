use crate::parsing;

pub fn run() {
    let cmds = get_cmds();
    let (distance, depth) = pilot(&cmds);
    println!("Day  2: Piloted forward {} and at a depth of {} for a total of {}", distance, depth, distance * depth);
}

/// Returns (distance, depth) after piloting the moves
pub fn pilot(cmds: &Vec<(String, u32)>) -> (u32, u32) {
    let mut depth = 0;
    let mut distance = 0;
    cmds.iter().for_each(|cmd| {
        match cmd.0.as_str() {
            "forward" => distance += cmd.1,
            "down" => depth += cmd.1,
            "up" => depth -= cmd.1,
            _ => {},
        }
    });

    (distance, depth)
}

fn parse_cmds(s: &str) -> Vec<(String, u32)> {
    parsing::parse_input(s, |parts| (parts[0].to_string(), parts[1].parse::<u32>().unwrap()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pilot() {
        let cmds = parse_cmds(r"
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        ");
        let (distance, depth) = pilot(&cmds);
        assert_eq!(distance, 15);
        assert_eq!(depth, 10);
    }
}

fn get_cmds() -> Vec<(String, u32)> {
    parse_cmds(r"
forward 5
forward 2
forward 9
down 2
forward 9
forward 3
forward 2
down 6
forward 3
forward 3
down 3
down 3
forward 8
down 5
forward 7
forward 9
forward 9
forward 6
forward 9
forward 3
forward 3
forward 1
forward 7
down 6
forward 7
forward 4
down 3
down 1
forward 4
down 7
down 2
down 8
forward 9
down 5
down 2
forward 6
up 4
down 3
down 1
down 6
down 7
forward 6
up 2
forward 2
down 9
down 3
forward 7
up 3
up 7
forward 8
forward 7
down 4
up 8
up 1
forward 4
down 6
forward 9
forward 3
down 1
down 1
forward 2
forward 4
forward 3
up 2
forward 1
down 8
forward 4
down 5
forward 9
up 1
forward 3
forward 6
up 8
forward 1
forward 7
up 9
down 3
up 7
down 1
forward 5
forward 3
forward 7
down 5
down 1
down 2
down 4
down 5
down 5
down 8
up 9
down 7
down 7
down 6
forward 3
forward 5
forward 1
forward 8
up 4
down 8
down 3
down 1
down 9
down 3
down 9
down 8
down 2
forward 1
forward 7
forward 1
down 3
down 1
up 3
down 6
forward 6
forward 6
down 8
forward 3
down 1
forward 2
down 4
down 7
up 8
forward 4
down 4
up 1
forward 6
down 6
forward 5
forward 9
up 5
down 3
up 9
down 6
up 3
down 9
down 4
down 2
forward 3
down 6
down 7
down 9
forward 7
forward 2
forward 5
up 6
down 8
forward 1
down 2
forward 8
down 5
down 2
up 4
forward 9
up 4
down 4
down 2
forward 3
up 2
down 5
down 9
up 4
forward 9
down 5
down 2
down 2
forward 9
up 9
forward 6
up 7
down 2
forward 8
down 1
forward 2
down 3
up 6
down 4
forward 5
up 4
forward 5
forward 9
forward 9
forward 5
down 1
down 7
forward 6
forward 9
forward 3
forward 4
up 8
down 1
up 1
down 3
up 1
down 5
forward 6
up 5
up 6
down 9
forward 7
down 7
forward 1
forward 2
up 4
forward 4
down 9
up 4
down 6
forward 1
up 9
forward 2
down 8
forward 6
forward 6
down 6
forward 5
forward 9
up 8
down 4
forward 8
up 6
down 4
up 9
forward 2
down 8
down 6
forward 2
down 3
forward 1
forward 3
forward 2
up 4
down 6
up 6
down 4
down 2
forward 1
up 7
forward 9
forward 9
forward 7
down 1
down 9
forward 5
forward 4
forward 1
forward 4
down 5
forward 6
forward 8
down 5
down 7
up 2
up 1
forward 5
down 5
down 1
down 5
down 8
down 8
down 5
forward 7
down 6
down 2
forward 9
forward 1
forward 3
forward 4
up 3
down 4
up 7
forward 8
forward 7
up 4
down 9
forward 7
forward 6
up 1
down 1
up 6
down 5
up 1
forward 2
down 2
forward 3
down 6
up 5
up 4
down 8
down 5
down 3
down 4
up 3
down 3
down 2
down 7
up 2
down 8
forward 5
up 1
forward 9
down 6
down 6
down 8
up 4
forward 9
forward 8
up 7
down 9
down 4
forward 9
forward 9
up 2
down 1
forward 1
forward 4
forward 2
forward 9
down 1
down 3
down 1
down 3
up 5
down 2
forward 4
down 2
forward 1
down 6
up 9
down 3
forward 1
forward 5
forward 8
down 5
down 6
down 9
forward 4
down 7
up 8
forward 8
down 9
forward 6
down 8
up 3
forward 4
up 9
down 7
up 7
forward 6
forward 1
up 9
down 7
up 7
down 5
forward 6
up 7
down 8
down 8
forward 4
up 4
forward 1
forward 6
down 4
up 9
forward 4
up 1
up 8
up 6
forward 9
forward 4
forward 7
up 1
down 5
up 5
up 5
forward 4
down 9
up 8
down 6
down 3
down 6
forward 2
up 1
forward 3
up 8
down 1
forward 5
down 9
forward 4
up 5
forward 3
forward 2
down 8
down 9
up 6
down 9
down 7
forward 5
forward 4
forward 9
up 8
forward 3
down 7
forward 9
down 8
forward 4
forward 8
up 9
up 9
down 6
forward 5
forward 5
forward 5
up 2
up 2
up 1
down 6
forward 2
forward 2
down 8
down 6
up 2
forward 1
down 1
up 5
forward 7
down 2
forward 1
forward 3
down 5
down 7
forward 8
forward 4
forward 9
up 1
up 7
up 9
forward 3
up 1
forward 6
forward 3
forward 9
up 9
down 6
forward 8
up 6
down 9
forward 3
forward 7
down 9
forward 4
forward 5
forward 7
down 1
down 4
down 3
forward 6
down 3
forward 7
forward 8
down 1
forward 3
down 4
up 7
forward 2
forward 8
down 6
up 3
down 2
forward 9
forward 5
forward 7
up 2
up 6
down 9
forward 1
up 5
forward 1
up 6
up 2
up 1
forward 6
down 8
forward 7
down 5
forward 3
down 9
down 4
forward 3
down 1
up 1
up 7
forward 4
down 6
forward 3
forward 2
down 8
forward 9
forward 6
up 3
down 3
down 1
down 7
up 8
up 2
up 8
forward 6
forward 8
forward 6
forward 4
down 6
forward 6
forward 6
forward 1
down 2
forward 2
forward 6
down 1
up 6
forward 3
forward 9
forward 6
down 2
forward 2
up 4
down 2
up 4
forward 2
forward 2
forward 3
up 1
forward 8
forward 3
forward 3
forward 1
down 5
down 9
forward 4
down 1
forward 5
forward 2
down 5
forward 6
forward 3
up 3
forward 6
forward 9
forward 5
down 2
down 2
down 7
forward 8
down 1
down 5
down 9
up 3
up 5
up 4
forward 3
down 9
down 2
down 8
down 5
down 2
forward 4
up 3
down 5
up 3
down 8
down 7
up 1
forward 2
forward 1
down 2
up 1
up 5
down 8
down 3
up 9
forward 2
down 8
down 4
down 3
forward 3
forward 7
up 1
down 9
forward 1
down 6
up 3
up 5
down 6
up 4
forward 7
up 4
forward 9
up 4
forward 4
down 7
down 2
down 8
up 3
down 7
down 4
up 5
forward 8
down 8
down 1
forward 7
up 9
down 5
up 8
down 1
up 3
forward 8
up 4
down 1
up 1
up 7
forward 3
forward 6
forward 5
forward 2
down 4
forward 2
down 7
up 7
up 6
down 4
forward 4
forward 9
forward 7
down 4
down 6
forward 5
down 2
down 6
down 2
down 2
forward 9
up 5
forward 9
down 3
down 6
down 1
forward 5
down 5
forward 7
forward 9
up 8
forward 6
down 6
down 2
forward 8
forward 5
up 9
up 4
forward 1
forward 2
forward 2
up 9
down 6
forward 6
forward 8
up 5
up 7
forward 4
down 3
forward 1
up 6
up 9
forward 6
up 8
forward 5
down 4
forward 1
down 3
forward 1
forward 6
forward 2
up 4
down 6
forward 9
down 2
forward 4
down 4
forward 3
down 5
down 2
forward 2
forward 2
up 1
forward 1
down 7
down 8
up 6
forward 8
forward 5
forward 8
down 6
forward 5
down 3
up 3
forward 4
up 9
forward 8
forward 4
down 3
forward 6
up 6
down 2
down 7
down 3
down 2
down 8
forward 5
down 9
up 6
down 6
forward 8
down 1
forward 8
down 1
down 1
forward 3
forward 9
down 6
forward 7
down 4
forward 1
forward 4
forward 3
down 6
forward 5
down 5
forward 1
forward 3
forward 8
down 9
up 8
up 6
up 6
forward 2
forward 9
down 4
down 8
forward 6
up 1
down 2
down 6
forward 6
forward 2
up 8
forward 6
down 9
down 1
forward 7
forward 6
forward 5
forward 6
down 6
up 7
down 9
forward 9
forward 3
forward 5
down 4
down 1
down 7
up 3
up 7
forward 6
forward 8
down 7
down 4
forward 7
down 6
up 1
forward 4
down 2
forward 4
forward 3
forward 4
forward 4
up 3
down 8
down 4
down 1
down 8
down 3
up 9
down 4
forward 7
down 6
up 2
down 8
up 9
down 6
forward 1
down 3
forward 9
down 9
forward 1
down 5
up 5
up 1
forward 8
down 8
down 9
down 5
down 2
down 5
forward 3
down 9
forward 4
forward 4
up 2
forward 8
forward 4
forward 1
down 5
forward 5
down 5
forward 6
forward 4
up 5
down 9
up 3
up 8
forward 5
forward 9
up 6
forward 6
down 5
forward 7
down 8
down 7
down 9
forward 4
down 8
forward 4
down 5
forward 6
forward 4
down 7
down 5
forward 4
down 3
up 4
forward 3
up 9
down 8
forward 9
forward 6
forward 9
down 1
forward 2
up 5
down 9
down 2
down 9
up 8
forward 7
forward 8
forward 4
down 1
up 8
forward 8
down 8
down 4
forward 9
down 3
forward 7
forward 9
down 5
forward 7
forward 1
forward 5
forward 2
down 4
forward 7
down 6
forward 3
down 9
forward 3
down 5
up 6
up 3
forward 1
up 9
down 1
forward 2
down 8
down 7
up 9
up 2
down 5
up 9
forward 9
forward 9
down 1
forward 5
up 5
forward 1
up 1
down 3
forward 3
down 3
forward 3
up 5
up 4
down 7
down 7
down 4
forward 7
down 6
forward 1
up 1
down 8
forward 4
down 3
forward 9
up 6
forward 6
forward 3
up 9
down 9
forward 4
up 5
down 3
down 8
down 3
down 3
forward 4
forward 6
forward 4
up 2
up 3
up 5
down 5
down 6
forward 5
forward 4
down 1
down 2
up 8
down 2
down 1
up 4
forward 5
forward 8
forward 8
forward 5
down 3
forward 4
up 8
forward 7
forward 4
down 9
down 6
forward 2
down 7
forward 6
up 7
up 1
up 4
forward 2
forward 9
forward 7
up 5
forward 2
up 5
forward 1
forward 2
forward 4
down 6
forward 2
up 6
up 7
forward 3
forward 2
forward 6
forward 4
forward 9
forward 6
up 6
forward 5
up 7
up 5
down 6
down 2
down 1
forward 7
down 5
down 1
down 7
forward 8
forward 8
forward 5
down 9
forward 6
               ")
}