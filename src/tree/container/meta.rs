use std::collections::HashMap;
use super::Container;

#[derive(PartialEq, Debug)]
pub(crate) struct Meta {
    pub content: HashMap<String, Container>,
    pub flags: u8,
    pub name: Option<String>,
}

struct MetaVisitor;
use serde::de::{self, Visitor};

use std::fmt;
impl<'de> Visitor<'de> for MetaVisitor {
    type Value = Meta;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("{\"name\":...,\"#f\":...,\"#n\":...}")
    }

    fn visit_map<M>(self, mut m: M) -> Result<Meta, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut map: HashMap<String, Container> = HashMap::new();
        let mut flags: Option<u8> = None;
        let mut name: Option<String> = None;
        while let Some(k) = m.next_key::<String>()? {
            match &k {
                s if s == "#f" => {
                    let fl = m.next_value::<u8>()?;
                    flags = Some(fl);
                }
                s if s == "#n" => {
                    let s = m.next_value::<String>()?;
                    name = Some(s);
                }
                s => {
                    let mut c = m.next_value::<Container>()?;
                    c.set_name(Some(s.clone()));
                    map.insert(s.clone(), c);
                }
            }
        }
        Ok(Meta {
            content: map,
            flags: flags.unwrap_or(0),
            name,
        })
    }
}

use serde::Deserialize;
impl<'de> Deserialize<'de> for Meta {
    fn deserialize<D>(d: D) -> Result<Meta, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_any(MetaVisitor)
    }
}
