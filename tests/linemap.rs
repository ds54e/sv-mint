use sv_mint::core::linemap::{LineMap, SpanBytes};

#[test]
fn linemap_basic_mapping() {
    let s = "a\nbc\n";
    let lm = LineMap::new(s);

    // Start of text
    let sp = SpanBytes { start: 0, end: 0 };
    let ln = lm.to_lines(sp);
    assert_eq!((ln.line, ln.col, ln.end_line, ln.end_col), (1, 1, 1, 1));

    // 'b'..'c'
    let sp2 = SpanBytes { start: 2, end: 3 };
    let ln2 = lm.to_lines(sp2);
    assert_eq!((ln2.line, ln2.col, ln2.end_line, ln2.end_col), (2, 1, 2, 2));
}
