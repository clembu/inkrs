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
        if s == "^" {
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
            Comp::Up => write!(f, "^"),
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
