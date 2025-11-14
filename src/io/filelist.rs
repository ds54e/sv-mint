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
    processed: HashSet<PathBuf>,
    processing: HashSet<PathBuf>,
}

impl FilelistLoader {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            incdirs: Vec::new(),
            defines: Vec::new(),
            processed: HashSet::new(),
            processing: HashSet::new(),
        }
    }

    fn finish(self) -> FilelistLoad {
        FilelistLoad {
            files: self.files,
            incdirs: self.incdirs,
            defines: self.defines,
        }
    }

    fn process(&mut self, path: &Path) -> Result<(), ConfigError> {
        let canon = fs::canonicalize(path).map_err(|_| ConfigError::NotFound {
            path: path.display().to_string(),
        })?;
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
        if trimmed.starts_with("-f") {
            let include = self.parse_filelist_include(file, line_no, trimmed)?;
            let path_buf = resolve_path(base, include);
            self.process(&path_buf)?;
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
        let path = resolve_path(base, trimmed);
        self.files.push(path);
        Ok(())
    }

    fn parse_filelist_include<'a>(
        &self,
        file: &Path,
        line_no: usize,
        trimmed: &'a str,
    ) -> Result<&'a str, ConfigError> {
        let mut parts = trimmed.split_whitespace();
        let flag = parts.next().unwrap();
        if flag == "-f" {
            let target = parts.next().ok_or_else(|| ConfigError::InvalidValue {
                detail: format!("filelist {}:{}: -f missing path", file.display(), line_no),
            })?;
            if parts.next().is_some() {
                return Err(ConfigError::InvalidValue {
                    detail: format!("filelist {}:{}: extra tokens after -f", file.display(), line_no),
                });
            }
            return Ok(target);
        }
        if let Some(rest) = flag.strip_prefix("-f") {
            if !rest.is_empty() {
                if parts.next().is_some() {
                    return Err(ConfigError::InvalidValue {
                        detail: format!("filelist {}:{}: extra tokens after {}", file.display(), line_no, flag),
                    });
                }
                return Ok(rest);
            }
        }
        Err(ConfigError::InvalidValue {
            detail: format!(
                "filelist {}:{}: malformed -f directive {}",
                file.display(),
                line_no,
                trimmed
            ),
        })
    }

    fn push_incdirs(&mut self, base: &Path, rest: &str) {
        for entry in rest.split('+') {
            let val = entry.trim();
            if val.is_empty() {
                continue;
            }
            self.incdirs.push(resolve_path(base, val));
        }
    }

    fn push_defines(&mut self, rest: &str) {
        for entry in rest.split('+') {
            let val = entry.trim();
            if val.is_empty() {
                continue;
            }
            self.defines.push(val.to_string());
        }
    }
}

fn resolve_path(base: &Path, raw: &str) -> PathBuf {
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        path
    } else if is_windows_absolute(raw) {
        PathBuf::from(raw)
    } else {
        base.join(path)
    }
}

fn is_windows_absolute(raw: &str) -> bool {
    let bytes = raw.as_bytes();
    if bytes.len() >= 3 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
        return bytes[2] == b'\\' || bytes[2] == b'/';
    }
    raw.starts_with("\\\\")
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
        let load = load_filelists(&[root_path.clone()]).unwrap();
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
        std::env::set_var("TOP_FILE", target.to_string_lossy().to_string());
        std::env::set_var("INC_ROOT", inc.to_string_lossy().to_string());
        std::env::set_var("DEFVAL", "42");
        let body = "\
$TOP_FILE
+incdir+${INC_ROOT}
+define+FOO=$DEFVAL\\
+BAR=BAZ
";
        fs::write(&list, body).unwrap();
        let load = load_filelists(&[list.clone()]).unwrap();
        assert!(load.files.iter().any(|p| p == &target));
        assert_eq!(load.incdirs.len(), 1);
        assert!(load
            .incdirs
            .iter()
            .any(|p| p.to_string_lossy() == inc.to_string_lossy()));
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
