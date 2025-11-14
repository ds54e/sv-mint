use crate::core::errors::ConfigError;
use std::collections::HashSet;
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
        for (idx, line) in text.lines().enumerate() {
            self.handle_line(&canon, &base, line, idx + 1)?;
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
    } else {
        base.join(path)
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
}
