
mod measurements;

fn depth_increases(d: &Vec<u32>) -> usize { 
    d.iter().enumerate()
        .skip(1)
        .filter(|x| &d[x.0-1]  < x.1)
        .count()
}

fn depth_sliding_window_increases(d: &Vec<u32>) -> usize {
    d.iter().enumerate()
        .skip(3)
        .filter(|x| (&d[x.0-3] + &d[x.0-2] + &d[x.0-1])  < (&d[x.0-2] + &d[x.0-1] + &d[x.0]))
        .count()
}

fn diagnostics_freq(d: &Vec<u64>) -> (u64, u64) { // (gamma, epsilon) aka (most common bits, least common bits)
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


fn main() {
    let depths = measurements::get_depths();
    println!("Depth Increases={}, Depth Sliding Window Increases={}", depth_increases(&depths), depth_sliding_window_increases(&depths));
    let diagnostics = measurements::get_diagnostics();
    let (gamma, epsilon) = diagnostics_freq(&diagnostics);
    println!("Gamma={}, Epsilon={}, Power Consumption={}", gamma, epsilon, gamma * &epsilon);
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_depths() -> Vec<u32> {
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }

    fn get_diagnostics() -> Vec<u64> {
        vec![0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001, 0b00010, 0b01010]
    }

    #[test]
    fn test_depth_increases() {
        assert_eq!(depth_increases(&get_depths()), 7);
    }

    #[test]
    fn test_sliding_window_increases() {
        assert_eq!(depth_sliding_window_increases(&get_depths()), 5);
    }

    #[test] 
    fn test_diagnostics_freq() {
        let (gamma, epsilon) = diagnostics_freq(&get_diagnostics());
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
        assert_eq!(gamma * epsilon, 198);
    }
}