use std::cmp::min;

pub fn substr_edit_distance(s1: &[u8], s2: &[u8]) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        dp[i][0] = i;
    }
    for j in 0..=len2 {
        dp[0][j] = j;
    }

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1[i - 1] == s2[j - 1] { 0 } else { 1 };
            dp[i][j] = min(
                min(dp[i - 1][j] + 1, dp[i][j - 1] + 1),
                dp[i - 1][j - 1] + cost,
            );
        }
    }

    dp[len1][len2]
}

pub fn get_closest_string(candidates: &[String], s: &str) -> Option<String> {
    let mut min_dist = usize::MAX;
    let mut result = None;

    for candidate in candidates.iter() {
        let dist = substr_edit_distance(candidate.as_bytes(), s.as_bytes());
        if dist < min_dist {
            min_dist = dist;
            result = Some(candidate.clone());
        }
    }

    result
}
