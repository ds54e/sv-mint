#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpanBytes {
    pub start: usize,
    pub end: usize,
}

impl SpanBytes {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpanLines {
    pub line: u32,
    pub col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

#[derive(Clone, Debug)]
pub struct LineMap {
    starts: Vec<usize>,
    len: usize,
}

impl LineMap {
    pub fn new(s: &str) -> Self {
        let mut starts = Vec::with_capacity(1024);
        starts.push(0);
        for (i, &b) in s.as_bytes().iter().enumerate() {
            if b == b'\n' {
                starts.push(i + 1);
            }
        }
        Self { starts, len: s.len() }
    }

    pub fn to_lines(&self, span: SpanBytes) -> SpanLines {
        let s = span.start.min(self.len);
        let e = span.end.min(self.len);
        let s_line_idx = self.lower_bound_line(s);
        let e_line_idx = self.lower_bound_line(e);
        let s_col = (s.saturating_sub(self.starts[s_line_idx]) + 1) as u32;
        let e_col = (e.saturating_sub(self.starts[e_line_idx]) + 1) as u32;
        SpanLines {
            line: (s_line_idx as u32) + 1,
            col: s_col,
            end_line: (e_line_idx as u32) + 1,
            end_col: e_col,
        }
    }

    fn lower_bound_line(&self, pos: usize) -> usize {
        let mut lo = 0usize;
        let mut hi = self.starts.len();
        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            if self.starts[mid] <= pos {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        lo
    }

    pub fn starts(&self) -> &Vec<usize> {
        &self.starts
    }
}
