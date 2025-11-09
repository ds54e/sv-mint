pub fn strip_bom(mut s: String) -> String {
    if s.as_bytes().starts_with(&[0xEF, 0xBB, 0xBF]) {
        s.drain(..3);
    }
    s
}

pub fn normalize_lf(s: String) -> String {
    s.replace("\r\n", "\n").replace('\r', "\n")
}
