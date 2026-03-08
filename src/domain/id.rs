use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EntryId(u64);

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

impl EntryId {
    pub fn generate() -> Self {
        EntryId(NEXT_ID.fetch_add(1, Ordering::SeqCst))
    }
}

impl fmt::Debug for EntryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EntryId({})", self.0)
    }
}
