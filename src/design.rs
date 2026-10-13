// A struct with some trait implementations...will have:
// 1. Metadata
// 2. One trait function will be the construct()


use std::fmt;

struct Design {
    title: String,
    date: String,
}

impl Design {
    fn new(title: impl Into<String>, date: impl Into<String>, path: &Path) -> Self {
        Self {
            title: title.into(),
            date: date.into(),
            path: path,
        }
    }
}

impl fmt::Display for Design {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.date)
    }
}

impl fmt::Debug for Design {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Design")
            .field("title", &self.title)
            .field("date", &self.date)
            .finish()
    }
}
