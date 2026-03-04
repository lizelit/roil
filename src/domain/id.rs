use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EntryId(u64);

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

impl EntryId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn generate() -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        Self(id)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Debug for EntryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EntryId({})", self.0)
    }
}
