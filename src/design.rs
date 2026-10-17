/// Data structure for automated creation and tracking of fab designs
use std::fmt;

use chrono::{DateTime,Local};
use monstertruck::modeling::Solid;

use crate::shapes::cube;


fn default_cube_spec() -> Solid {

    // Just unwrapping bc we don't expect problems with a simple shape
    // like a cube. More complicated shapes may encounter unexpected problems.
    // Use standard, robust, idiomatic error handling instead of this.
    cube(1.0).unwrap()
}


pub struct Design {
    title: String,
    description: String,
    date: DateTime<Local>,
    spec: fn() -> Solid
}


impl Design {

    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>
    )
    -> Design {

        Self {
            title: title.into(),
            description: description.into(),
            date: Local::now(),
            spec: default_cube_spec
        }
    }

    pub fn set_spec(mut self, spec: fn() -> Solid)  -> Design {
        self.spec = spec;
        self
    }
}


impl fmt::Display for Design {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(
            f,
            "=== {}\nLocation: {}\n\nDescription:\n{}\n\n",
            self.title, self.description, file!()
        )
    }
}


impl fmt::Debug for Design {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let loc = file!();
        println!("Debug for design at {loc}");

        f.debug_struct("Design")
            .field("title", &self.title)
            .field("date", &self.date)
            .finish()
    }
}

