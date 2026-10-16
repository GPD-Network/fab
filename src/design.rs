// A struct with some trait implementations...will have:
// 1. Metadata
// 2. One trait function will be the construct()
use std::fmt;
use std::path::Path;


struct Design<'a> {
    title: String,
    date: String,
    description: String,
    path: &'a Path,  // lifetime marker indicating shares lifetime with Design
}


impl<'a> Design<'a> {
    fn new(title: impl Into<String>,
        description: impl Into<String>,
        date: impl Into<String>,
        path: &'a Path) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            date: date.into(),
            path: path,
        }
    }
}


impl fmt::Display for Design<'_> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(
            f,
            "{} ({})\nLocation: {}",
            self.title, self.date, self.path.display()
        )
    }
}


impl fmt::Debug for Design<'_> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        f.debug_struct("Design")
            .field("title", &self.title)
            .field("date", &self.date)
            .finish()
    }
}


