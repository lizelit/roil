use std::collections::HashSet;

use crate::buffer::BufferLine;
use crate::domain::EntryId;

#[derive(Debug)]
pub enum ValidationError {
    EmptyName { id: EntryId },
    DuplicateName { name: String },
}

pub fn validate(lines: &[BufferLine]) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    let mut seen = HashSet::new();

    for line in lines {
        let trimmed = line.name.trim();

        if trimmed.is_empty() {
            errors.push(ValidationError::EmptyName { id: line.id });
            continue;
        }

        if !seen.insert(trimmed.to_string()) {
            errors.push(ValidationError::DuplicateName {
                name: trimmed.to_string(),
            });
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
