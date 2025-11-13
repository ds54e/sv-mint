use crate::core::errors::ParseError;
use crate::core::linemap::LineMap;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub(crate) struct SourceFile {
    pub display: String,
    pub text: String,
    pub line_map: LineMap,
}

pub(crate) struct SourceCache {
    files: HashMap<PathBuf, Arc<SourceFile>>,
}

impl SourceCache {
    pub(crate) fn new(path: &Path, text: String) -> Self {
        let key = normalize_path(path);
        let display = path.to_string_lossy().to_string();
        let line_map = LineMap::new(&text);
        let entry = Arc::new(SourceFile {
            display,
            text,
            line_map,
        });
        let mut files = HashMap::new();
        files.insert(key, entry);
        Self { files }
    }

    pub(crate) fn get_or_load(&mut self, path: &Path) -> Result<Arc<SourceFile>, ParseError> {
        let key = normalize_path(path);
        if !self.files.contains_key(&key) {
            let text = read_with_fallback(&key, path)?;
            let display = path.to_string_lossy().to_string();
            let line_map = LineMap::new(&text);
            let entry = Arc::new(SourceFile {
                display,
                text,
                line_map,
            });
            self.files.insert(key.clone(), entry);
        }
        Ok(self.files.get(&key).unwrap().clone())
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn read_with_fallback(primary: &Path, fallback: &Path) -> Result<String, ParseError> {
    match fs::read_to_string(primary) {
        Ok(text) => Ok(text),
        Err(primary_err) => {
            if primary == fallback {
                return Err(ParseError::ParseFailed {
                    detail: format!("{}: {}", primary.display(), primary_err),
                });
            }
            fs::read_to_string(fallback).map_err(|e| ParseError::ParseFailed {
                detail: format!("{}: {}", fallback.display(), e),
            })
        }
    }
}
