use crate::domain::entry::Entry;
use crate::domain::id::EntryId;
use std::collections::HashMap;
use std::path::PathBuf;

pub enum Diff {
    Create { entry: Entry },
    Delete { entry: Entry },
    Rename { from: PathBuf, to: PathBuf },
}

pub fn diff(original: &[Entry], current: &[Entry]) -> Vec<Diff> {
    let mut result = Vec::new();

    let original_map: HashMap<EntryId, &Entry> = original.iter().map(|e| (*e.id(), e)).collect();
    let current_map: HashMap<EntryId, &Entry> = current.iter().map(|e| (*e.id(), e)).collect();

    // detect deleted or renamed
    for (id, orig) in &original_map {
        match current_map.get(id) {
            None => {
                result.push(Diff::Delete {
                    entry: (*orig).clone(),
                });
            }
            Some(curr) => {
                if orig.path() != curr.path() {
                    result.push(Diff::Rename {
                        from: orig.path().to_path_buf(),
                        to: curr.path().to_path_buf(),
                    });
                }
            }
        }
    }

    //detect created
    for (id, curr) in &current_map {
        if !original_map.contains_key(id) {
            result.push(Diff::Create {
                entry: (*curr).clone(),
            });
        }
    }

    result
}
