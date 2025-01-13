use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};
use std::fmt;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Node(usize);

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted = format!("n{}", self.0);
        serializer.serialize_str(&formatted)
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "n{}", self.0)
    }
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NodeVisitor;

        impl<'de> Visitor<'de> for NodeVisitor {
            type Value = Node;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(r#"a string in the format "n{int}"#)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v.chars().next() {
                    Some('n') => {
                        let content = &v[1..];
                        content.parse::<usize>().map(Node).map_err(|_| {
                            de::Error::invalid_value(de::Unexpected::Str(v), &"a valid usize value")
                        })
                    }
                    _ => Err(de::Error::invalid_value(
                        de::Unexpected::Str(v),
                        &r#"a string in the format "n{int}"#,
                    )),
                }
            }
        }

        deserializer.deserialize_str(NodeVisitor)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Client(usize);

impl Serialize for Client {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted = format!("c{}", self.0);
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for Client {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NodeVisitor;

        impl<'de> Visitor<'de> for NodeVisitor {
            type Value = Client;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(r#"a string in the format "c{int}"#)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v.chars().next() {
                    Some('c') => {
                        let content = &v[1..];
                        content.parse::<usize>().map(Client).map_err(|_| {
                            de::Error::invalid_value(de::Unexpected::Str(v), &"a valid usize value")
                        })
                    }
                    _ => Err(de::Error::invalid_value(
                        de::Unexpected::Str(v),
                        &r#"a string in the format "c{int}"#,
                    )),
                }
            }
        }

        deserializer.deserialize_str(NodeVisitor)
    }
}
