
mod measurements;

fn increases(d: &Vec<u32>) -> usize { 
    d.iter().enumerate()
        .skip(1)
        .filter(|x| &d[x.0-1]  < x.1)
        .count()
}

fn sliding_window_increases(d: &Vec<u32>) -> usize {
    d.iter().enumerate()
        .skip(3)
        .filter(|x| (&d[x.0-3] + &d[x.0-2] + &d[x.0-1])  < (&d[x.0-2] + &d[x.0-1] + &d[x.0]))
        .count()
}

fn main() {
    let d = measurements::get();
    println!("Increases={}", increases(&d));
    println!("Sliding Window Increases={}", sliding_window_increases(&d));
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_d() -> Vec<u32> {
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }

    #[test]
    fn test_increases() {
        assert_eq!(increases(&test_d()), 7)
    }

    #[test]
    fn test_sliding_window_increases() {
        assert_eq!(sliding_window_increases(&test_d()), 5)
    }
}