//! Generic fixture walk, preserving source member order without another dependency.
//! All numeric data come unchanged from labeled fixture ledgers.
use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::{fmt, fs, path::Path};

enum Node {
    Object(Vec<(String, Node)>),
    Array(Vec<Node>),
    Scalar(Value),
}
impl<'de> Deserialize<'de> for Node {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Walk;
        impl<'de> Visitor<'de> for Walk {
            type Value = Node;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("JSON value")
            }
            fn visit_map<M: MapAccess<'de>>(self, mut m: M) -> Result<Node, M::Error> {
                let mut fields = Vec::new();
                while let Some(pair) = m.next_entry()? {
                    fields.push(pair);
                }
                Ok(Node::Object(fields))
            }
            fn visit_seq<S: SeqAccess<'de>>(self, mut s: S) -> Result<Node, S::Error> {
                let mut items = Vec::new();
                while let Some(v) = s.next_element()? {
                    items.push(v);
                }
                Ok(Node::Array(items))
            }
            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Node, E> {
                Ok(Node::Scalar(v.into()))
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Node, E> {
                Ok(Node::Scalar(v.into()))
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Node, E> {
                Ok(Node::Scalar(v.into()))
            }
            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Node, E> {
                Ok(Node::Scalar(v.into()))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<Node, E> {
                Ok(Node::Scalar(v.into()))
            }
            fn visit_unit<E: de::Error>(self) -> Result<Node, E> {
                Ok(Node::Scalar(Value::Null))
            }
        }
        d.deserialize_any(Walk)
    }
}
impl Node {
    fn value(&self) -> Value {
        match self {
            Self::Object(fields) => {
                Value::Object(fields.iter().map(|(k, v)| (k.clone(), v.value())).collect())
            }
            Self::Array(items) => Value::Array(items.iter().map(Self::value).collect()),
            Self::Scalar(v) => v.clone(),
        }
    }
    fn objects(&self, path: &str, out: &mut Vec<(String, Value)>) {
        match self {
            Self::Object(fields) => {
                out.push((
                    path.into(),
                    Value::Array(fields.iter().map(|(_, v)| v.value()).collect()),
                ));
                for (key, value) in fields {
                    let key = key.replace('~', "~0").replace('/', "~1");
                    value.objects(&format!("{path}/{key}"), out);
                }
            }
            Self::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    item.objects(&format!("{path}/{i}"), out);
                }
            }
            Self::Scalar(_) => {}
        }
    }
}
pub fn check_directory(directory: &Path, load: impl Fn(&str) -> Result<(), String>) {
    let mut files: Vec<_> = fs::read_dir(directory)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    files.sort();
    assert!(!files.is_empty());
    let mut accepted = Vec::new();
    for file in files {
        let text = fs::read_to_string(&file).unwrap();
        load(&text).unwrap();
        let node: Node = serde_json::from_str(&text).unwrap();
        let original = node.value();
        let mut objects = Vec::new();
        node.objects("", &mut objects);
        assert!(!objects.is_empty());
        println!("{}: discovered {} objects", file.display(), objects.len());
        for (path, array) in objects {
            let mut changed = original.clone();
            *changed.pointer_mut(&path).unwrap() = array;
            match load(&changed.to_string()) {
                Ok(()) => accepted.push(format!("{}:{path}", file.display())),
                Err(e) => assert!(
                    e.contains("object"),
                    "{}:{path}: unclear rejection: {e}",
                    file.display()
                ),
            }
        }
    }
    assert!(
        accepted.is_empty(),
        "accepted positional objects: {accepted:?}"
    );
}
