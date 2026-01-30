use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct ValidationResult {
    valid: bool,
    line: Option<u32>,
    column: Option<u32>,
    end_line: Option<u32>,
    end_column: Option<u32>,
    message: Option<String>,
}

#[wasm_bindgen]
pub fn validate_toml(content: &str) -> String {
    let result = toml::from_str::<toml::Value>(content);
    let index = LineIndex::new(content);

    let validation = match result {
        Ok(_) => ValidationResult {
            valid: true,
            line: None,
            column: None,
            end_line: None,
            end_column: None,
            message: None,
        },
        Err(error) => {
            let (start_line, start_col, end_line, end_col) = if let Some(range) = error.span() {
                let mut start_offset = range.start;
                let mut end_offset = range.end;

                // Siempre intentamos expandir a los límites del "token" conflictivo
                // Hacia atrás hasta un delimitador
                while start_offset > 0 {
                    let prev = content[..start_offset]
                        .char_indices()
                        .last()
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    let c = content[prev..].chars().next().unwrap();
                    if c.is_whitespace()
                        || c == '='
                        || c == '['
                        || c == '{'
                        || c == ','
                        || c == '"'
                        || c == '\''
                    {
                        break;
                    }
                    start_offset = prev;
                }
                // Hacia adelante hasta un delimitador
                while end_offset < content.len() {
                    let c = content[end_offset..].chars().next().unwrap();
                    if c.is_whitespace()
                        || c == '#'
                        || c == ']'
                        || c == '}'
                        || c == ','
                        || c == '"'
                        || c == '\''
                    {
                        break;
                    }
                    end_offset += c.len_utf8();
                }

                let start = index.coords(start_offset, content);
                let end = index.coords(end_offset, content);
                (Some(start.0), Some(start.1), Some(end.0), Some(end.1))
            } else {
                (None, None, None, None)
            };

            ValidationResult {
                valid: false,
                line: start_line,
                column: start_col,
                end_line,
                end_column: end_col,
                message: Some(error.to_string()),
            }
        }
    };

    serde_json::to_string(&validation).unwrap()
}

struct LineIndex {
    line_starts: Vec<usize>,
}

impl LineIndex {
    fn new(text: &str) -> Self {
        let mut line_starts = vec![0];
        for (i, c) in text.char_indices() {
            if c == '\n' {
                line_starts.push(i + 1);
            }
        }
        Self { line_starts }
    }

    fn coords(&self, offset: usize, content: &str) -> (u32, u32) {
        let line = match self.line_starts.binary_search(&offset) {
            Ok(idx) => idx,
            Err(idx) => idx - 1,
        };
        let line_start = self.line_starts[line];
        // Convertimos el offset de bytes a conteo de caracteres UTF-16 (que es lo que VS Code espera)
        let col = content[line_start..offset].chars().count();
        (line as u32, col as u32)
    }
}

#[cfg(test)]
mod tests {}
