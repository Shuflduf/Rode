use std::collections::HashSet;

/// Structure for info related to autocomplete
#[derive(Clone, Debug)]
pub struct Suggestion {
    pub text: String,
    pub kind: SuggestionKind,
}

/// TODO: write docs
#[derive(Clone, Debug, PartialEq)]
pub enum SuggestionKind {
    Variable,
    Function,
    Keyword,
    Type,
}

/// Structure for the actual autocomplete with detailed info params
pub struct Autocomplete {
    pub active: bool,
    pub suggestion: Vec<Suggestion>, // system can give more than 1, they are ordered
    pub selected_index: usize,
    pub trigger_position: usize,
    pub prefix: String,
}

impl Default for Autocomplete {
    /// Default values for autocomplete 
    fn default() -> Self {
        Self {
            active: false,
            suggestion: Vec::new(),
            selected_index: 0,
            trigger_position: 0,
            prefix: String::new(),
        }
    }
}

impl Autocomplete {
    /// Function to extract keywords from the current text 
    fn extract_keywords(text: &str) -> HashSet<String> {
        let mut keywords = HashSet::new();

        // common keywords 
        // TODO: for later think of a better way to extract keywords 
        // instead of just hardcoding common ones across languages
        let common_keywords = vec![
            // Rust
            "fn", "let", "mut", "const", "static", "if", "else", "match", "for", "while", "loop", "return", "break", "continue", "pub", "use", "mod", "struct", "enum", "trait", "impl", "type", "where", "unsafe", "async", "await", "move",
            // JS/TS 
            "function", "var", "const", "class", "interface", "extends", "implements", "import", "export", "default", "async", "await", "try", "catch", "finally",
            // Python
            "def", "class", "import", "from", "as", "lamba", "yield", "with", "raise", "try", "except", "finally", "assert", "pass", "del", "global", "nonlocal",
            // Common types
            "String", "Vec", "HashMap", "Option", "Result", "bool", "i32", "i64", "u32", "u64", "f32", "f64", "char", "str", "usize", "isize",
        ];

        for keyword in common_keywords {
            keywords.insert(keyword.to_string());    
        }

        keywords
    }

    fn extract_identifiers(text: &str) -> HashSet<String> {
        let mut identifiers = HashSet::new();
        let mut current_word = String::new();

        for ch in text.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                current_word.push(ch);
            } else {
                if !current_word.is_empty() && current_word.len() > 1 {
                    // we should be able to skip single character words
                    // as well as keywords 
                    if !current_word.chars().next().unwrap().is_numeric() {
                        identifiers.insert(current_word.clone());
                    }
                }

                current_word.clear();
            }
        }

        // we must not forget the last word 
        if !current_word.is_empty() && current_word.len() > 1 {
            identifiers.insert(current_word);
        }

        identifiers
    }
    
    /// Provides the system context on where the user currently is
    fn is_function_context(text: &str, cursor_pos: usize) -> bool {
        if cursor_pos == 0 {
            return false;
        }

        let after = &text[cursor_pos..];
        after.trim_start().starts_with('(')
    }

    pub fn get_current_word(text: &str, cursor_pos: usize) -> (String, usize) {
        if cursor_pos == 0 {
            return (String::new(), 0);
        }

        let before_cursor = &text[..cursor_pos];
        let mut word_start = cursor_pos;

        for (i, ch) in before_cursor.char_indices().rev() {
            if ch.is_alphanumeric() || ch == '_' {
                word_start = i;
            } else {
                break;
            }
        }

        let current_word = text[word_start..cursor_pos].to_string();
        (current_word, word_start)
    }

    /// Trigger the autocomplete sequence at the current cursor position 
    pub fn trigger(&mut self, text: &str, cursor_pos: usize) {
        let (prefix, start_pos) = Self::get_current_word(text, cursor_pos);

        // making sure not to trigger it for very short prefixes 
        if prefix.len() < 2 {
            self.active = false;
            return;
        }

        self.prefix = prefix.clone();
        self.trigger_position = start_pos;

        let mut all_suggestions = Vec::new();

        let keywords = Self::extract_keywords(text);
        for keyword in keywords {
            if keyword.to_lowercase().starts_with(&prefix.to_lowercase()) && keyword != prefix {
                all_suggestions.push(Suggestion {
                    text: keyword,
                    kind: SuggestionKind::Keyword,
                });
            }
        }

        let identifiers = Self::extract_identifiers(text);
        let is_func = Self::is_function_context(text, cursor_pos);

        for identifier in identifiers {
            if identifier.to_lowercase().starts_with(&prefix.to_lowercase()) && identifier != prefix {
                // kind of a gamble but if its followed by '(' or contains
                // certain patterns then its very likely a function 
                let kind = if is_func || identifier.contains("_fn") || identifier.ends_with("_func") {
                    SuggestionKind::Function
                } else if identifier.chars().next().unwrap().is_uppercase() {
                    SuggestionKind::Type
                } else {
                    SuggestionKind::Variable
                };

                all_suggestions.push(Suggestion {
                    text: identifier,
                    kind,
                });
            }
        }

        all_suggestions.sort_by(|a, b| {
            let kind_order = |k: &SuggestionKind| match k {
                SuggestionKind::Keyword => 0,
                SuggestionKind::Function => 1,
                SuggestionKind::Type => 2,
                SuggestionKind::Variable => 3,
            };

            kind_order(&a.kind)
                .cmp(&kind_order(&b.kind))
                .then_with(|| a.text.cmp(&b.text))
        });

        // remove the duplicates
        all_suggestions.dedup_by(|a, b| a.text == b.text);

        self.suggestion = all_suggestions;
        self.selected_index = 0;
        self.active = !self.suggestion.is_empty();
    }

    pub fn select_next(&mut self) {
        if !self.suggestion.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.suggestion.len();
        }
    }

    pub fn select_previous(&mut self) {
        if !self.suggestion.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.suggestion.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn get_selected(&self) -> Option<&Suggestion> {
        if self.active && self.selected_index < self.suggestion.len() {
            Some(&self.suggestion[self.selected_index])
        } else {
            None
        }
    }

    pub fn apply_suggestion(&mut self, text: &mut String, cursor_pos: &mut usize) -> bool {
        if let Some(suggestion) = self.get_selected() {
            let completion = &suggestion.text;

            text.replace_range(self.trigger_position..*cursor_pos, completion);
            *cursor_pos = self.trigger_position + completion.len();

            self.active = false;
            true
        } else {
            false
        }
    }

    pub fn cancel(&mut self) {
        self.active = false;
        self.suggestion.clear();
        self.selected_index = 0;
    }
}
