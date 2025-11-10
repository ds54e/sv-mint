pub fn strip_bom(mut s: String) -> String {
    if s.as_bytes().starts_with(&[0xEF, 0xBB, 0xBF]) {
        s.drain(..3);
    }
    s
}

pub fn normalize_lf(s: String) -> String {
    s.replace("\r\n", "\n").replace('\r', "\n")
}

pub fn line_starts(s: &str) -> Vec<usize> {
    let mut v = Vec::with_capacity(1024);
    v.push(0);
    for (i, ch) in s.char_indices() {
        if ch == '\n' && i < s.len() {
            v.push(i + 1);
        }
    }
    v
}

pub fn linecol_at(starts: &[usize], byte_idx: usize) -> (u32, u32) {
    if starts.is_empty() {
        return (1, 1);
    }
    let mut lo = 0usize;
    let mut hi = starts.len();
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if starts[mid] <= byte_idx {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let line = (lo + 1) as u32;
    let col = (byte_idx.saturating_sub(starts[lo]) + 1) as u32;
    (line, col)
}

pub fn truncate_preview_utf8(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_owned();
    }
    let mut end = max.min(s.len());
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    if end == 0 {
        return String::new();
    }
    let mut t = s[..end].to_owned();
    t.push_str(" ...");
    t
}
