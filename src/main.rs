use std::collections::HashMap;
use std::cmp::{max, min};

mod data;

fn count_digit_segments(d: &Vec<(Vec<String>, Vec<String>)>) -> [u32;10] {
    let mut digits = [0;10];
    d.iter()
        .flat_map(|input_output| input_output.1.iter())
        .map(|entry| {
            match entry.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                _ => 0,
            }
        })
        .for_each(|num| digits[num] += 1);
    digits
}

fn decode_segments(d: &Vec<(Vec<String>, Vec<String>)>) -> Vec<u32> {
    //  0
    // 1 2
    //  3
    // 4 5
    //  6
    d.iter()
        .map(|input_output| {
            let mut opts = [127;7];
            let is_found = |x: u32| -> bool { x == 1 || x == 2 || x == 4 || x == 8 || x == 16 || x == 32 || x == 64 };
            let map_segments = |x: &String| -> u32 {
                x.chars()
                    .map(|c| {
                         match c {
                             'a' => 1,
                             'b' => 2,
                             'c' => 4,
                             'd' => 8,
                             'e' => 16,
                             'f' => 32,
                             'g' => 64,
                             _ => 0,
                         }
                     })
                    .sum()
            };
            let mut update_p = |x: &String| {
                let segments = map_segments(x);
                match x.len() {
                    2 => (0..7).for_each(|i| {
                        if i == 2 || i == 5 { opts[i] &= segments; } else { opts[i] &= !segments; }
                    }),
                    3 => (0..7).for_each(|i| {
                        if i == 0 || i == 2 || i == 5 { opts[i] &= segments; } else { opts[i] &= !segments; }
                    }),
                    4 => (0..7).for_each(|i| {
                        if i == 1 || i == 2 || i == 3 || i == 5 { opts[i] &= segments; } else { opts[i] &= !segments; }
                    }),
                    7 => {},
                    _ => {},
                }
            };
            input_output.0.iter().chain(input_output.1.iter())
                .for_each(|s| update_p(&s));

            let mut changed = true;
            while changed {
                changed = false;

                (0..7).for_each(|i| {
                    if is_found(opts[i]) {
                        (0..7).for_each(|j| {
                            if i != j && opts[i] & opts[j] > 0 {
                                opts[j] &= !opts[i];
                                changed = true;
                            }
                        });
                    }
                });
            }

            if opts[3] == opts[1] { // disambiguate from 0 and invalid
                let mut i = 0;
                while i < 8 {
                    let check_digit = u32::pow(2, i);
                    if opts[3] & check_digit > 0 {
                        let check_0 = 127 & !check_digit;
                        if input_output.0.iter().chain(input_output.1.iter())
                            .map(map_segments)
                            .filter(|x| *x == check_0)
                            .count() > 0 {
                            // we know that check_0 is a valid digit, thus check_digit is not a
                            // possibility for opts[3]
                            (0..7).for_each(|i| { if i == 3 { opts[i] &= check_digit; } else { opts[i] &= !check_digit; } });
                        }
                    }
                    i += 1;
                }
            }
            if opts[2] == opts[5] { // disambiguate from 6 and invalid
                let mut i = 0;
                while i < 8 {
                    let check_digit = u32::pow(2, i);
                    if opts[2] & check_digit > 0 {
                        let check_6 = 127 & !check_digit;
                        if input_output.0.iter().chain(input_output.1.iter())
                            .map(map_segments)
                            .filter(|x| *x == check_6)
                            .count() > 0 {
                            // we know that check_6 is a valid digit, thus check_digit is not a
                            // possibility for opts[2]
                            (0..7).for_each(|i| { if i == 2 { opts[i] &= check_digit; } else { opts[i] &= !check_digit; } });
                        }
                    }
                    i += 1;
                }
            }
            if opts[4] == opts[6] { // disambiguate from 9 and invalid
                let mut i = 0;
                while i < 8 {
                    let check_digit = u32::pow(2, i);
                    if opts[4] & check_digit > 0 {
                        let check_9 = 127 & !check_digit;
                        if input_output.0.iter().chain(input_output.1.iter())
                            .map(map_segments)
                            .filter(|x| *x == check_9)
                            .count() > 0 {
                            // we know that check_9 is a valid digit, thus check_digit is not a
                            // possibility for opts[4]
                            (0..7).for_each(|i| { if i == 4 { opts[i] &= check_digit; } else { opts[i] &= !check_digit; } });
                        }
                    }
                    i += 1;
                }
            }

            let mut mapping = HashMap::<u32,u32>::new();
            mapping.insert(opts[0] | opts[1] | opts[2] | opts[4] | opts[5] | opts[6], 0);
            mapping.insert(opts[2] | opts[5], 1);
            mapping.insert(opts[0] | opts[2] | opts[3] | opts[4] | opts[6], 2);
            mapping.insert(opts[0] | opts[2] | opts[3] | opts[5] | opts[6], 3);
            mapping.insert(opts[1] | opts[2] | opts[3] | opts[5], 4);
            mapping.insert(opts[0] | opts[1] | opts[3] | opts[5] | opts[6], 5);
            mapping.insert(opts[0] | opts[1] | opts[3] | opts[4] | opts[5] | opts[6], 6);
            mapping.insert(opts[0] | opts[2] | opts[5], 7);
            mapping.insert(opts[0] | opts[1] | opts[2] | opts[3] | opts[4] | opts[5] | opts[6], 8);
            mapping.insert(opts[0] | opts[1] | opts[2] | opts[3] | opts[5] | opts[6], 9);

            let mut output_value : u32 = 0;
            input_output.1.iter()
                .map(map_segments)
                .map(|x| mapping[&x])
                .for_each(|x| {
                    output_value *= 10;
                    output_value += x;
                });
            output_value
        })
        .collect::<Vec<u32>>()
}

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

fn find_sliding_window_increases(d: &Vec<u32>, window: u32) -> usize {
    d.iter().enumerate()
        .skip(window as usize)
        .filter(|x| (1..=window).map(|i| &d[x.0-(i as usize)]).sum::<u32>() < (0..window).map(|i| &d[x.0-(i as usize)]).sum::<u32>())
        .count()
}

/// Finds the cheapest position which would require the least number of moves by the swarm to align
fn find_cheapest_alignment(d: &Vec<u32>) -> (u32, u32) {
    let pos_cost = |pos : u32| -> u32 { d.iter().map(|n| (pos as i32 - *n as i32).abs() as u32).sum() };
    let mut best_pos = d.iter().sum::<u32>() / d.len() as u32;
    let mut best_cost = pos_cost(best_pos);
    let mut search = |dir : i32| {
        let mut pos = (best_pos as i32 + dir) as u32;
        loop {
            let cost = pos_cost(pos);
            if cost < best_cost {
                best_cost = cost;
                best_pos = pos;
            } else if cost > best_cost {
                break
            }
            pos = (pos as i32 + dir) as u32;
        }
    };
    search(1);
    search(-1);
    (best_pos, best_cost)
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

/// Returns (number of fish by days until new gen, total number of fish)
fn simulate_lanternfish(fish : &[u64;9], num_days: u32) -> ([u64;9], u64) {
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

fn main() {
    let depths = data::get_depths();
    println!("Day  1: Depth Increases={}", find_increases(&depths));
    println!("      : Depth Sliding Window Increases={}", find_sliding_window_increases(&depths, 3));
    let cmds = data::get_cmds();
    let (distance, depth) = pilot(&cmds);
    println!("Day  2: Piloted forward {} and at a depth of {} for a total of {}", distance, depth, distance * depth);
    let diagnostics = data::get_diagnostics();
    let (gamma, epsilon) = find_bit_frequencies(&diagnostics);
    println!("Day  3: Gamma={}, Epsilon={}, Power Consumption={}", gamma, epsilon, gamma * &epsilon);
    let bingo_data = data::get_bingo();
    match play_bingo(&bingo_data.0, &bingo_data.1) {
        Some(results) => println!("Day  4: Board {} wins bingo with a score of {}", results.0, results.1),
        None => println!("Day  4: No one wins bingo!"),
    }
    let (_, num_vent_intersections) = find_intersections(&data::get_vents());
    println!("Day  5: Number of vent intersections={}", num_vent_intersections);
    let (_, num_lanternfish) = simulate_lanternfish(&data::get_lanternfish(), 80);
    println!("Day  6: There will be {} lanternfish after 80 days", num_lanternfish);
    let (alignment_pos, alignment_cost) = find_cheapest_alignment(&data::get_crab_positions());
    println!("Day  7: The crabs would best align at position {} with a total cost of {}", alignment_pos, alignment_cost);
    let digit_counts = count_digit_segments(&data::get_digit_segments());
    println!("Day  8: There will be {} 1s, {} 4s, {} 7s, {} 8s for a total of {}", digit_counts[1], digit_counts[4], digit_counts[7], digit_counts[8], digit_counts[1] + digit_counts[4] + digit_counts[7] + digit_counts[8]);
    println!("      : Total of outputs={}", decode_segments(&data::get_digit_segments()).iter().sum::<u32>());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_digit_segments() {
        let digit_segments = data::parse_digit_segments(r"
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        ");
        let digits = count_digit_segments(&digit_segments);
        assert_eq!(digits[1], 8);
        assert_eq!(digits[4], 6);
        assert_eq!(digits[7], 5);
        assert_eq!(digits[8], 7);
    }

    #[test]
    fn test_decode_segments() {
        let digit_segments = data::parse_digit_segments(r"
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        ");
        assert_eq!(decode_segments(&digit_segments).iter().sum::<u32>(), 61229);
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
    fn test_find_cheapest_alignment() {
        let positions = data::parse_positions("16,1,2,0,4,2,7,1,2,14");
        let (pos, cost) = find_cheapest_alignment(&positions);
        assert_eq!(pos, 2);
        assert_eq!(cost, 37);
    }

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
        assert_eq!(find_sliding_window_increases(&depths, 1), 7);
        assert_eq!(find_sliding_window_increases(&depths, 3), 5);
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
        let (calls, boards) = data::parse_bingo(r"
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
            8  2 23  4 24
            21  9 14 16  7
            6 10  3 18  5
            1 12 20 15 19

            3 15  0  2 22
            9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
            2  0 12  3  7
        ");
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

    #[test]
    fn test_lanternfish() {
        let fish = data::parse_lanternfish("3,4,3,1,2");
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
