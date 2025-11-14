use crate::core::errors::ConfigError;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FilelistLoad {
    pub files: Vec<PathBuf>,
    pub incdirs: Vec<PathBuf>,
    pub defines: Vec<String>,
    pub lib_dirs: Vec<PathBuf>,
    pub libexts: Vec<String>,
}

pub fn load_filelists(paths: &[PathBuf]) -> Result<FilelistLoad, ConfigError> {
    let mut loader = FilelistLoader::new();
    for path in paths {
        loader.process(path)?;
    }
    Ok(loader.finish())
}

struct FilelistLoader {
    files: Vec<PathBuf>,
    incdirs: Vec<PathBuf>,
    defines: Vec<String>,
    lib_dirs: Vec<PathBuf>,
    libexts: Vec<String>,
    processed: HashSet<PathBuf>,
    processing: HashSet<PathBuf>,
}

impl FilelistLoader {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            incdirs: Vec::new(),
            defines: Vec::new(),
            lib_dirs: Vec::new(),
            libexts: Vec::new(),
            processed: HashSet::new(),
            processing: HashSet::new(),
        }
    }

    fn finish(self) -> FilelistLoad {
        FilelistLoad {
            files: self.files,
            incdirs: self.incdirs,
            defines: self.defines,
            lib_dirs: self.lib_dirs,
            libexts: self.libexts,
        }
    }

    fn process(&mut self, path: &Path) -> Result<(), ConfigError> {
        let canon = normalize_path(fs::canonicalize(path).map_err(|_| ConfigError::NotFound {
            path: path.display().to_string(),
        })?);
        if self.processed.contains(&canon) {
            return Ok(());
        }
        if !self.processing.insert(canon.clone()) {
            return Err(ConfigError::InvalidValue {
                detail: format!("filelist cycle detected at {}", canon.display()),
            });
        }
        let bytes = fs::read(&canon).map_err(|_| ConfigError::NotFound {
            path: canon.display().to_string(),
        })?;
        let text = String::from_utf8(bytes).map_err(|_| ConfigError::InvalidUtf8 {
            path: canon.display().to_string(),
            source: None,
        })?;
        let base = canon
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));
        let lines = preprocess_lines(&text, &canon)?;
        for (line_no, line) in lines {
            self.handle_line(&canon, &base, &line, line_no)?;
        }
        self.processing.remove(&canon);
        self.processed.insert(canon);
        Ok(())
    }

    fn handle_line(&mut self, file: &Path, base: &Path, line: &str, line_no: usize) -> Result<(), ConfigError> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("#") {
            return Ok(());
        }
        if let Some(rest) = trimmed.strip_prefix("+incdir+") {
            self.push_incdirs(base, rest);
            return Ok(());
        }
        if let Some(rest) = trimmed.strip_prefix("+define+") {
            self.push_defines(rest);
            return Ok(());
        }
        if let Some(rest) = trimmed.strip_prefix("+libext+") {
            self.push_libexts(rest);
            return Ok(());
        }
        if trimmed.starts_with("-f") {
            let include = self.parse_filelist_include(file, line_no, trimmed)?;
            let path_buf = resolve_path(base, &include);
            self.process(&path_buf)?;
            return Ok(());
        }
        if trimmed.starts_with("-y") {
            let dir = self.parse_flag_value(file, line_no, trimmed, "-y")?;
            let path = resolve_path(base, &dir);
            self.incdirs.push(path);
            self.lib_dirs.push(resolve_path(base, &dir));
            return Ok(());
        }
        if trimmed.starts_with("+") {
            return Err(ConfigError::InvalidValue {
                detail: format!(
                    "filelist {}:{}: unsupported directive {}",
                    file.display(),
                    line_no,
                    trimmed
                ),
            });
        }
        let token = strip_outer_quotes(trimmed);
        let path = resolve_path(base, &token);
        self.files.push(path);
        Ok(())
    }

    fn parse_filelist_include(&self, file: &Path, line_no: usize, trimmed: &str) -> Result<String, ConfigError> {
        self.parse_flag_value(file, line_no, trimmed, "-f")
    }

    fn push_incdirs(&mut self, base: &Path, rest: &str) {
        for entry in split_plus_entries(rest) {
            let token = strip_outer_quotes(&entry);
            if token.is_empty() {
                continue;
            }
            self.incdirs.push(resolve_path(base, &token));
        }
    }

    fn push_defines(&mut self, rest: &str) {
        for entry in split_plus_entries(rest) {
            let val = entry.trim();
            if val.is_empty() {
                continue;
            }
            let token = strip_outer_quotes(val);
            self.defines.push(token);
        }
    }

    fn push_libexts(&mut self, rest: &str) {
        for entry in split_plus_entries(rest) {
            let token = strip_outer_quotes(&entry);
            if token.is_empty() {
                continue;
            }
            let ext = token.trim();
            if ext.is_empty() {
                continue;
            }
            let normalized = if ext.starts_with('.') {
                ext.to_string()
            } else {
                format!(".{}", ext)
            };
            self.libexts.push(normalized);
        }
    }

    fn parse_flag_value(&self, file: &Path, line_no: usize, trimmed: &str, flag: &str) -> Result<String, ConfigError> {
        if !trimmed.starts_with(flag) {
            return Err(ConfigError::InvalidValue {
                detail: format!(
                    "filelist {}:{}: malformed {} directive {}",
                    file.display(),
                    line_no,
                    flag,
                    trimmed
                ),
            });
        }
        let rest = &trimmed[flag.len()..];
        if rest.is_empty() {
            return Err(ConfigError::InvalidValue {
                detail: format!("filelist {}:{}: {} missing path", file.display(), line_no, flag),
            });
        }
        let trimmed_rest = rest.trim_start();
        if trimmed_rest.is_empty() {
            return Err(ConfigError::InvalidValue {
                detail: format!("filelist {}:{}: {} missing path", file.display(), line_no, flag),
            });
        }
        let (token, remainder) = take_token(trimmed_rest).map_err(|msg| ConfigError::InvalidValue {
            detail: format!("filelist {}:{}: {}", file.display(), line_no, msg),
        })?;
        if !remainder.trim().is_empty() {
            return Err(ConfigError::InvalidValue {
                detail: format!("filelist {}:{}: extra tokens after {}", file.display(), line_no, flag),
            });
        }
        Ok(strip_outer_quotes(token))
    }
}

fn resolve_path(base: &Path, raw: &str) -> PathBuf {
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        normalize_path(path)
    } else if is_windows_absolute(raw) {
        normalize_path(PathBuf::from(raw))
    } else {
        normalize_path(base.join(path))
    }
}

fn is_windows_absolute(raw: &str) -> bool {
    let bytes = raw.as_bytes();
    if bytes.len() >= 3 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
        return bytes[2] == b'\\' || bytes[2] == b'/';
    }
    raw.starts_with("\\\\")
}

#[cfg(windows)]
fn normalize_path(path: PathBuf) -> PathBuf {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};

    const VERBATIM_PREFIX: [u16; 4] = [b'\\' as u16, b'\\' as u16, b'?' as u16, b'\\' as u16];
    const UNC_MARKER: [u16; 4] = [b'U' as u16, b'N' as u16, b'C' as u16, b'\\' as u16];
    let wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    if wide.len() >= VERBATIM_PREFIX.len() && wide[..VERBATIM_PREFIX.len()] == VERBATIM_PREFIX {
        let remainder = &wide[VERBATIM_PREFIX.len()..];
        if remainder.len() >= UNC_MARKER.len() && remainder[..UNC_MARKER.len()] == UNC_MARKER {
            let mut rebuilt = Vec::with_capacity(remainder.len() - UNC_MARKER.len() + 2);
            rebuilt.extend_from_slice(&[b'\\' as u16, b'\\' as u16]);
            rebuilt.extend_from_slice(&remainder[UNC_MARKER.len()..]);
            return PathBuf::from(OsString::from_wide(&rebuilt));
        }
        if remainder.is_empty() {
            return PathBuf::new();
        }
        return PathBuf::from(OsString::from_wide(remainder));
    }
    path
}

#[cfg(not(windows))]
fn normalize_path(path: PathBuf) -> PathBuf {
    path
}

fn preprocess_lines(text: &str, file: &Path) -> Result<Vec<(usize, String)>, ConfigError> {
    let mut out = Vec::new();
    let mut buffer = String::new();
    let mut start_line = 0;
    for (idx, raw_line) in text.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed_end = raw_line.trim_end_matches(['\r', '\n']);
        let mut segment = trimmed_end.to_string();
        let continued = segment.trim_end().ends_with('\\');
        if continued {
            let trimmed = segment.trim_end();
            let without = trimmed[..trimmed.len() - 1].trim_end();
            segment = without.to_string();
        }
        if buffer.is_empty() {
            start_line = line_no;
        }
        buffer.push_str(&segment);
        if continued {
            continue;
        }
        let expanded = expand_env(&buffer, file, start_line)?;
        out.push((start_line, expanded));
        buffer.clear();
    }
    if !buffer.is_empty() {
        return Err(ConfigError::InvalidValue {
            detail: format!(
                "filelist {}:{}: trailing line continuation without content",
                file.display(),
                start_line
            ),
        });
    }
    Ok(out)
}

fn expand_env(line: &str, file: &Path, line_no: usize) -> Result<String, ConfigError> {
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    let mut out = String::with_capacity(line.len());
    while i < chars.len() {
        let ch = chars[i];
        if ch != '$' {
            out.push(ch);
            i += 1;
            continue;
        }
        if i + 1 >= chars.len() {
            out.push('$');
            i += 1;
            continue;
        }
        let next = chars[i + 1];
        if next == '$' {
            out.push('$');
            i += 2;
            continue;
        }
        if next == '{' || next == '(' {
            let closing = if next == '{' { '}' } else { ')' };
            let mut j = i + 2;
            let mut name = String::new();
            while j < chars.len() && chars[j] != closing {
                name.push(chars[j]);
                j += 1;
            }
            if j == chars.len() {
                return Err(ConfigError::InvalidValue {
                    detail: format!(
                        "filelist {}:{}: unterminated ${} variable",
                        file.display(),
                        line_no,
                        next
                    ),
                });
            }
            let value = lookup_env(&name, file, line_no)?;
            out.push_str(&value);
            i = j + 1;
            continue;
        }
        let mut j = i + 1;
        let mut name = String::new();
        while j < chars.len() && is_env_ident(chars[j]) {
            name.push(chars[j]);
            j += 1;
        }
        if name.is_empty() {
            out.push('$');
            i += 1;
            continue;
        }
        let value = lookup_env(&name, file, line_no)?;
        out.push_str(&value);
        i = j;
    }
    Ok(out)
}

fn lookup_env(name: &str, file: &Path, line_no: usize) -> Result<String, ConfigError> {
    env::var(name).map_err(|_| ConfigError::InvalidValue {
        detail: format!(
            "filelist {}:{}: environment variable ${} not set",
            file.display(),
            line_no,
            name
        ),
    })
}

fn is_env_ident(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

fn strip_outer_quotes(token: &str) -> String {
    let trimmed = token.trim();
    if trimmed.len() >= 2 {
        let bytes = trimmed.as_bytes();
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        if (first == last) && (first == b'"' || first == b'\'') {
            return trimmed[1..trimmed.len() - 1].to_string();
        }
    }
    trimmed.to_string()
}

fn split_plus_entries(rest: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    for ch in rest.chars() {
        if ch == '\'' || ch == '"' {
            if let Some(q) = quote {
                if q == ch {
                    quote = None;
                }
            } else {
                quote = Some(ch);
            }
            current.push(ch);
            continue;
        }
        if ch == '+' && quote.is_none() {
            parts.push(std::mem::take(&mut current));
        } else {
            current.push(ch);
        }
    }
    parts.push(current);
    parts
}

fn take_token(input: &str) -> Result<(&str, &str), &'static str> {
    if input.is_empty() {
        return Err("missing token");
    }
    let trimmed = input.trim_start();
    if trimmed.is_empty() {
        return Err("missing token");
    }
    let first = trimmed.as_bytes()[0] as char;
    if first == '"' || first == '\'' {
        if let Some(pos) = trimmed[1..].find(first) {
            let end = 1 + pos;
            let token = &trimmed[..=end];
            let rest = &trimmed[end + 1..];
            return Ok((token, rest));
        } else {
            return Err("unterminated quoted token");
        }
    }
    if let Some(pos) = trimmed.find(char::is_whitespace) {
        let token = &trimmed[..pos];
        let rest = &trimmed[pos..];
        Ok((token, rest))
    } else {
        Ok((trimmed, ""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn parses_nested_lists() {
        let dir = tempdir().unwrap();
        let nested_dir = dir.path().join("nested");
        fs::create_dir_all(&nested_dir).unwrap();
        let nested_path = nested_dir.join("child.f");
        fs::write(&nested_path, "leaf.sv\n").unwrap();
        let root_path = dir.path().join("root.f");
        let root_body = format!(
            "\
-f {}
+incdir+inc+more
+define+FOO=1+BAR
top.sv
",
            nested_path.strip_prefix(dir.path()).unwrap().display()
        );
        fs::write(&root_path, root_body).unwrap();
        let load = load_filelists(std::slice::from_ref(&root_path)).unwrap();
        assert_eq!(load.files.len(), 2);
        assert!(load.files.iter().any(|p| p.ends_with(Path::new("top.sv"))));
        assert!(load.files.iter().any(|p| p.ends_with(Path::new("leaf.sv"))));
        assert_eq!(load.incdirs.len(), 2);
        assert!(load.incdirs.iter().all(|p| p.is_absolute()));
        assert_eq!(load.defines, vec!["FOO=1".to_string(), "BAR".to_string()]);
    }

    #[test]
    fn detects_cycles() {
        let dir = tempdir().unwrap();
        let a = dir.path().join("a.f");
        let b = dir.path().join("b.f");
        fs::write(&a, "-f b.f\n").unwrap();
        fs::write(&b, "-f a.f\n").unwrap();
        let err = load_filelists(&[a]).unwrap_err();
        match err {
            ConfigError::InvalidValue { detail } => {
                assert!(detail.contains("cycle"));
            }
            _ => panic!("expected ConfigError::InvalidValue"),
        }
    }

    #[test]
    fn expands_env_and_continuations() {
        let dir = tempdir().unwrap();
        let target = dir.path().join("foo.sv");
        fs::write(&target, "").unwrap();
        let list = dir.path().join("env.f");
        let inc = dir.path().join("incs");
        fs::create_dir_all(&inc).unwrap();
        let inc_spaced = dir.path().join("inc spaced");
        fs::create_dir_all(&inc_spaced).unwrap();
        let libdir = dir.path().join("lib path");
        fs::create_dir_all(&libdir).unwrap();
        std::env::set_var("TOP_FILE", target.to_string_lossy().to_string());
        std::env::set_var("INC_ROOT", inc.to_string_lossy().to_string());
        std::env::set_var("DEFVAL", "42");
        let body = "\
-y \"lib path\"
+libext+.sv+.svh
$TOP_FILE
+incdir+${INC_ROOT}
+incdir+\"inc spaced\"
+define+FOO=$DEFVAL\\
+BAR=BAZ
";
        fs::write(&list, body).unwrap();
        let load = load_filelists(std::slice::from_ref(&list)).unwrap();
        assert!(load.files.iter().any(|p| p == &target));
        assert!(load.incdirs.iter().any(|p| p == &inc));
        assert!(load.incdirs.iter().any(|p| p == &inc_spaced));
        assert!(load.incdirs.iter().any(|p| p == &libdir));
        assert!(load.defines.contains(&"FOO=42".to_string()));
        assert!(load.defines.contains(&"BAR=BAZ".to_string()));
        std::env::remove_var("TOP_FILE");
        std::env::remove_var("INC_ROOT");
        std::env::remove_var("DEFVAL");
    }

    #[test]
    fn windows_absolute_paths() {
        let base = Path::new("/tmp/project");
        let p = resolve_path(base, "C:\\proj\\foo.sv");
        assert_eq!(p.to_string_lossy(), "C:\\proj\\foo.sv");
        let unc = resolve_path(base, "\\\\server\\share\\bar.sv");
        assert_eq!(unc.to_string_lossy(), "\\\\server\\share\\bar.sv");
    }
}
