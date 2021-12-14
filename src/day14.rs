use crate::parsing;
use std::collections::HashMap;

pub fn run() {
    let mut polymer = get_polymer();
    {
        let ((min_c, min_count),(max_c, max_count)) = polymer.calc_min_max(10);
        println!(
            "Day 14: After 10 steps most common is {} ({}), least is {} ({}), Result={}",
            max_c, max_count, min_c, min_count, max_count - min_count
        );
    }
    {
        let ((min_c, min_count),(max_c, max_count)) = polymer.calc_min_max(40);
        println!(
            "      : After 40 steps most common is {} ({}), least is {} ({}), Result={}",
            max_c, max_count, min_c, min_count, max_count - min_count
        );
    }
}

type Rules = HashMap<(char,char),char>;
type FreqsCache = HashMap<(char, char, u32), HashMap<char, usize>>;
pub struct Polymer {
    start: Vec<char>,
    rules: Rules,
    freqs_cache: FreqsCache,
}

impl Polymer {
    pub fn new(start: &Vec<char>, rules: &Rules) -> Self {
        Self {
            start: start.clone(),
            rules: rules.clone(),
            freqs_cache: HashMap::new(),
        }
    }

    pub fn new_from_string(s: &str) -> Self {
        let mut start = Vec::<char>::new();
        let mut rules = HashMap::new();
        parsing::parse_input(s, |parts| {
            if start.is_empty() {
                start = parts[0].chars().collect();
            } else {
                let (left, right) = (parts[0], parts[2]);
                rules.insert((left.chars().nth(0).unwrap(), left.chars().nth(1).unwrap()), right.chars().nth(0).unwrap());
            }
        });
        Self::new(&start, &rules)
    }

    pub fn calc_freqs(&mut self, steps: u32) -> HashMap<char, usize> {
        let mut freqs_cache = HashMap::new();
        let freqs = Self::calc_freqs_internal(&self.rules, &mut freqs_cache, &self.start, steps);
        self.freqs_cache.extend(freqs_cache);
        freqs
    }

    fn calc_freqs_internal(rules: &Rules, freqs_cache: &mut FreqsCache, seq: &Vec<char>, steps: u32) -> HashMap<char, usize> {
        let mut freqs = HashMap::<char, usize>::new();
        seq.iter().take(seq.len() - 1)
            .zip(seq.iter().skip(1)).for_each(|(left, right)| {
                Self::calc_freqs_between(rules, freqs_cache, &(*left, *right), steps).iter()
                    .for_each(|(c, count)| *freqs.entry(*c).or_default() += count);
                *freqs.entry(*right).or_default() -= 1; // will count right side later as next left
        });
        if let Some(last) = seq.last() {
            *freqs.entry(*last).or_default() += 1;
        }
        freqs
    }

    fn calc_freqs_between(rules: &Rules, freqs_cache: &mut FreqsCache, sub: &(char, char), steps: u32) -> HashMap<char, usize> {
        if let Some(freqs) = freqs_cache.get(&(sub.0, sub.1, steps)) {
            return freqs.clone();
        }
        let mut freqs = HashMap::new();
        *freqs.entry(sub.0).or_default() += 1;
        *freqs.entry(sub.1).or_default() += 1;
        if steps >= 1 {
            if let Some(inserted) = rules.get(sub) {
                *freqs.entry(*inserted).or_default() += 1;

                if steps > 1 {
                    Self::calc_freqs_between(rules, freqs_cache, &(sub.0, *inserted), steps - 1).iter()
                        .for_each(|(c, count)| *freqs.entry(*c).or_default() += count);
                    Self::calc_freqs_between(rules, freqs_cache, &(*inserted, sub.1), steps - 1).iter()
                        .for_each(|(c, count)| *freqs.entry(*c).or_default() += count);
                    *freqs.entry(sub.0).or_default() -= 1;
                    *freqs.entry(*inserted).or_default() -= 2;
                    *freqs.entry(sub.1).or_default() -= 1;
                }
            }
        }
        freqs_cache.insert((sub.0, sub.1, steps), freqs.clone());
        freqs
    }

    pub fn calc_min_max(&mut self, steps: u32) -> ((char, usize), (char, usize)) {
        let mut min_count = 0 as usize;
        let mut min_c = ' ';
        let mut max_count = 0 as usize;
        let mut max_c = ' ';
        self.calc_freqs(steps).iter()
            .for_each(|(c, count)| {
                if min_count <= 0 || count < &min_count { min_c = *c;
                    min_count = *count;
                }
                if count > &max_count {
                    max_c = *c;
                    max_count = *count;
                }
            });
        ((min_c, min_count), (max_c, max_count))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_polymer_freqs() {
        let mut polymer = Polymer::new_from_string(r"
            NNNCCB
        ");
        let ((min_c, min_count),(max_c, max_count)) = polymer.calc_min_max(0);
        assert_eq!(min_c, 'B');
        assert_eq!(min_count, 1);
        assert_eq!(max_c, 'N');
        assert_eq!(max_count, 3);
    }

    #[test]
    fn test_polymer_calc_min_max() {
        let mut polymer = Polymer::new_from_string(r"
            NNCB

            CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C
        ");
        let ((min_c, min_count),(max_c, max_count)) = polymer.calc_min_max(10);
        assert_eq!(min_c, 'H');
        assert_eq!(min_count, 161);
        assert_eq!(max_c, 'B');
        assert_eq!(max_count, 1749);
    }
}

fn get_polymer() -> Polymer {
    Polymer::new_from_string(r"
CNBPHFBOPCSPKOFNHVKV

CS -> S
FB -> F
VK -> V
HO -> F
SO -> K
FK -> B
VS -> C
PS -> H
HH -> P
KH -> V
PV -> V
CB -> N
BB -> N
HB -> B
HV -> O
NC -> H
NF -> B
HP -> B
HK -> S
SF -> O
ON -> K
VN -> V
SB -> H
SK -> H
VH -> N
KN -> C
CC -> N
BF -> H
SN -> N
KP -> B
FO -> N
KO -> V
BP -> O
OK -> F
HC -> B
NH -> O
SP -> O
OO -> S
VC -> O
PC -> F
VB -> O
FF -> S
BS -> F
KS -> F
OV -> P
NB -> O
CF -> F
SS -> V
KV -> K
FP -> F
KC -> C
PF -> C
OS -> C
PN -> B
OP -> C
FN -> F
OF -> C
NP -> C
CK -> N
BN -> K
BO -> K
OH -> S
BH -> O
SH -> N
CH -> K
PO -> V
CN -> N
BV -> F
FV -> B
VP -> V
FS -> O
NV -> P
PH -> C
HN -> P
VV -> C
NK -> K
CO -> N
NS -> P
VO -> P
CP -> V
OC -> S
PK -> V
NN -> F
SC -> P
BK -> F
BC -> P
FH -> B
OB -> O
FC -> N
PB -> N
VF -> N
PP -> S
HS -> O
HF -> N
KK -> C
KB -> N
SV -> N
KF -> K
CV -> N
NO -> P
    ")
}


