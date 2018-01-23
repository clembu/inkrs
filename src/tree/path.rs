const PARENT_ID: &'static str = "^";

#[derive(PartialEq, Debug)]
pub(crate) enum Comp {
    Up,
    Name(String),
    Index(usize),
}

impl<T> From<T> for Comp
where
    T: Into<String>,
{
    fn from(s: T) -> Self {
        let s = s.into();
        if s == PARENT_ID {
            Comp::Up
        } else if let Ok(i) = s.parse::<usize>() {
            Comp::Index(i)
        } else {
            Comp::Name(s)
        }
    }
}
#[derive(PartialEq, Debug)]
pub(crate) struct Path {
    rel: bool,
    comps: Vec<Comp>,
}

impl Path {
    pub fn new() -> Path {
        Path {
            rel: false,
            comps: Vec::new(),
        }
    }
}

impl<T> From<T> for Path
where
    T: Into<String>,
{
    fn from(_s: T) -> Self {
        let s = _s.into();
        if s.is_empty() {
            Path::new()
        } else {
            let rel = s.starts_with('.');
            Path {
                rel,
                comps: {
                    if rel {
                        &s[1..]
                    } else {
                        &s[..]
                    }
                }.split('.')
                    .map(Comp::from)
                    .collect(),
            }
        }
    }
}

use std::fmt;
impl fmt::Display for Comp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Comp::Up => write!(f, "{}", PARENT_ID),
            Comp::Name(ref s) => s.fmt(f),
            Comp::Index(i) => i.fmt(f),
        }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.comps
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(".");
        if self.rel {
            write!(f, ".{}", s)
        } else {
            s.fmt(f)
        }
    }
}

struct PathVisitor;
use serde::de::{self, Visitor};
impl<'de> Visitor<'de> for PathVisitor {
    type Value = Path;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a string representation of an ink path")
    }
    fn visit_str<E>(self, v: &str) -> Result<Path, E>
    where
        E: de::Error,
    {
        Ok(Path::from(v))
    }
}

use serde::Deserialize;
impl<'de> Deserialize<'de> for Path {
    fn deserialize<D>(d: D) -> Result<Path, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(PathVisitor)
    }
}
