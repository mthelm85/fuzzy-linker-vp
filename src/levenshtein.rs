pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    let s_chars: Vec<_> = s.chars().collect();
    let t_chars: Vec<_> = t.chars().collect();
    let n = s_chars.len();
    let m = t_chars.len();

    if n == 0 {
        return m;
    }

    if m == 0 {
        return n;
    }

    let mut d = vec![vec![0; m + 1]; n + 1];

    for i in 0..=n {
        d[i][0] = i;
    }

    for j in 0..=m {
        d[0][j] = j;
    }

    for i in 0..n {
        for j in 0..m {
            let cost = if s_chars[i] == t_chars[j] { 0 } else { 1 };

            d[i + 1][j + 1] = (d[i][j + 1] + 1)
                .min(d[i + 1][j] + 1)
                .min(d[i][j] + cost);
        }
    }

    d[n][m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("book", "back"), 2);
        assert_eq!(levenshtein_distance("rust", "rust"), 0);
        assert_eq!(levenshtein_distance("rust", ""), 4);
        assert_eq!(levenshtein_distance("", "rust"), 4);
    }
}