use std::collections::HashMap;
use std::cmp::{max, min};

mod data;

/// Returns (gamma, epsilon) aka (most common bits, least common bits)
fn find_bit_frequencies(d: &Vec<u64>) -> (u64, u64) {
    #[derive(Debug)]
    struct Count(u32, u32);
    let mut counts = Vec::<Count>::new();
    d.iter().for_each(|num| {
        let mut mask = 1;
        let mut pos = 0;
        while &mask < num || pos < counts.len() {
            if counts.len() <= pos { counts.push(Count(0, 0)); }
            counts.get_mut(pos)
                .and_then(|count| {
                    if mask & num > 0 { (*count).1 += 1; } else { (*count).0 += 1; };
                    Some(count)
                });
            pos += 1;
            mask *= 2;
        }
    });
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut order = 1;
    counts.iter().for_each(|count| {
        gamma += order * (if count.1 > count.0 { 1 } else { 0 });
        epsilon += order * (if count.1 < count.0 { 1 } else { 0 });
        order *= 2;
    });
    (gamma, epsilon)
}

fn find_increases(d: &Vec<u32>) -> usize {
    d.iter().enumerate()
        .skip(1)
        .filter(|x| &d[x.0-1]  < x.1)
        .count()
}

/// Returns (all represented points, num points with > 1 line through it)
/// also ignores all angled lines
fn find_intersections(segments : &Vec<((u32,u32),(u32,u32))>) -> (HashMap<(u32,u32), u32>, usize) {
    let mut intersections = HashMap::new();
    segments.iter().for_each(|segment| {
        if segment.0.0 == segment.1.0 {
            for y in min(segment.0.1, segment.1.1)..=max(segment.0.1, segment.1.1) {
                *intersections.entry((segment.0.0, y)).or_insert(0) += 1;
            }
        } else if segment.0.1 == segment.1.1 {
            for x in min(segment.0.0, segment.1.0)..=max(segment.0.0, segment.1.0) {
                *intersections.entry((x, segment.0.1)).or_insert(0) += 1;
            }
        }
    });

    let num_intersections = intersections.iter()
        .filter(|x| *x.1 > 1)
        .count();

    (intersections, num_intersections)
}

fn find_sliding_window_increases(d: &Vec<u32>) -> usize {
    d.iter().enumerate()
        .skip(3)
        .filter(|x| (&d[x.0-3] + &d[x.0-2] + &d[x.0-1])  < (&d[x.0-2] + &d[x.0-1] + &d[x.0]))
        .count()
}

/// Returns (distance, depth) after piloting the moves
fn pilot(cmds: &Vec<(String, u32)>) -> (u32, u32) {
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

/// Returns index of winning board and score
type Board = [u32;25];
fn play_bingo(calls: &Vec<u32>, boards: &Vec<Board>) -> Option<(usize, u64)> {
    let mut scores = Vec::<Board>::with_capacity(boards.len());
    scores.resize(boards.len(), [0;25]);
    let board_wins = |b: &Board| -> bool {
        for row in 0..5 {
            let r = 5*row;
            if b[r] == 1 && b[r+1] == 1 && b[r+2] == 1 && b[r+3] == 1 && b[r+4] == 1 {
                return true;
            }
        }
        for c in 0..5 {
            if b[c] == 1 && b[c+5] == 1 && b[c+10] == 1 && b[c+15] == 1 && b[c+20] == 1 {
                return true;
            }
        }

        false
    };
    let board_score = |board: &Board, score: &Board| -> u32 {
        board.iter().zip(score.iter())
            .filter_map(|pair| { if pair.1 == &0 { Some(pair.0) } else { None } })
            .sum()
    };
    for call in calls.iter() {
        for score in scores.iter_mut().enumerate() {
            for pos in 0..25 {
                if call == &boards[score.0][pos] {
                    (score.1)[pos] = 1;
                }
            }
            if board_wins(score.1) {
                return Some((score.0, (call * board_score(&boards[score.0], &score.1)).into()));
            }
        }
    }
    return None
}

fn main() {
    let depths = data::get_depths();
    println!("Depth Increases={}, Depth Sliding Window Increases={}",
             find_increases(&depths), find_sliding_window_increases(&depths));
    let cmds = data::get_cmds();
    let (distance, depth) = pilot(&cmds);
    println!("Piloted forward {} and at a depth of {} for a total of {}", distance, depth, distance * depth);
    let diagnostics = data::get_diagnostics();
    let (gamma, epsilon) = find_bit_frequencies(&diagnostics);
    println!("Gamma={}, Epsilon={}, Power Consumption={}", gamma, epsilon, gamma * &epsilon);
    let bingo_data = data::get_bingo();
    match play_bingo(&bingo_data.0, &bingo_data.1) {
        Some(results) => println!("Board {} wins bingo with a score of {}", results.0, results.1),
        None => println!("No one wins bingo!"),
    }
    let (_, num_vent_intersections) = find_intersections(&data::get_vents());
    println!("Number of vent intersections={}", num_vent_intersections);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_increases() {
        let depths = data::parse_u32s(r"
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
        ");
        assert_eq!(find_increases(&depths), 7);
    }

    #[test]
    fn test_find_sliding_window_increases() {
        let depths = data::parse_u32s(r"
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
        ");
        assert_eq!(find_sliding_window_increases(&depths), 5);
    }

    #[test]
    fn test_find_bit_frequencies() {
        let numbers = data::parse_binary_u64s(r"
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010
        ");
        let (gamma, epsilon) = find_bit_frequencies(&numbers);
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
        assert_eq!(gamma * epsilon, 198);
    }

    #[test]
    fn test_pilot() {
        let cmds = data::parse_cmds(r"
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

    #[test]
    fn test_play_bingo() {
        let calls = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];
        let boards = vec![
            [
                22, 13, 17, 11,  0,
                 8,  2, 23,  4, 24,
                21,  9, 14, 16,  7,
                 6, 10,  3, 18,  5,
                 1, 12, 20, 15, 19,
            ],
            [
                 3, 15,  0,  2, 22,
                 9, 18, 13, 17,  5,
                19,  8,  7, 25, 23,
                20, 11, 10, 24,  4,
                14, 21, 16, 12,  6,
            ],
            [
                14, 21, 17, 24,  4,
                10, 16, 15,  9, 19,
                18,  8, 23, 26, 20,
                22, 11, 13,  6,  5,
                 2,  0, 12,  3,  7,
            ],
        ];
        match play_bingo(&calls, &boards) {
            Some(result) => {
                assert_eq!(result.0, 2);
                assert_eq!(result.1, 4512);
            },
            None => panic!("No winner found!"),
        }
    }

    #[test]
    fn test_find_intersections() {
        let segments = data::parse_segments(r"
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2
        ");
        let (_, num_intersections) = find_intersections(&segments);
        assert_eq!(num_intersections, 5);
    }
}
