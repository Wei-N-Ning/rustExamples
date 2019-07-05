use std::cmp::{self, Ord};

pub fn levenshtein_distance(source: &str, target: &str) -> usize {
    if source.is_empty() {
        return target.len();
    }

    if target.is_empty() {
        return source.len();
    }

    let mut cache: Vec<usize> = (0..=target.len()).collect();

    for (i, source_char) in source.chars().enumerate() {
        let mut next_dist = i + 1;

        for (j, target_char) in target.chars().enumerate() {
            let current_dist = next_dist;

            let mut dist_if_substitute = cache[j];
            if source_char != target_char {
                dist_if_substitute += 1;
            }

            let dist_if_insert = current_dist + 1;
            let dist_if_delete = cache[j + 1] + 1;

            next_dist = min_3(dist_if_substitute, dist_if_insert, dist_if_delete);

            cache[j] = current_dist;
        }

        cache[target.len()] = next_dist;
    }

    cache[target.len()]
}

fn min_3<T: Ord>(a: T, b: T, c: T) -> T {
    cmp::min(a, cmp::min(b, c))
}

fn main() {
    let lines: Vec<&str> = include_str!("../sample.txt")
        .split('\n')
        .collect();

    let benchmark = || {
        for _ in 0..10000 {
            let mut last_value = "";
            for line in &lines {
                levenshtein_distance(last_value, line);
                last_value = line;
            }
        }
    };

    use std::time::Instant;
    let now = Instant::now();

    {
        benchmark();
    }

    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Seconds: {}", sec);
}

