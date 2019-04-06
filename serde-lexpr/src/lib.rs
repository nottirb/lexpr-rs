#![deny(missing_docs)]

//! This crate provides [Serde]-based serialization and
//! deserialization from statically-typed Rust data structures to the
//! dynamically typed S-expression values, using the [`lexpr::Value`]
//! type and to their text representation.
//!
//! # About representations
//!
//! ## Sequences
//!
//! The serializer will represent serde sequences as lists. Note that `Vec` is
//! considered a sequence, so it will (counterintuitevely) be represented by an
//! S-expression list. While it would be possible to serialize all sequences as
//! S-expression vectors instead, this would lead to unideomatic (noisy)
//! S-expressions. When deserializing, both vectors and (proper) lists are when
//! a serde sequence is expected.
//!
//! ```
//! use serde_lexpr::{from_str, to_string};
//!
//! let v1: Vec<u32> = from_str("(1 2 3)").unwrap();
//! assert_eq!(v1, vec![1, 2, 3]);
//! assert_eq!(to_string(&v1).unwrap(), "(1 2 3)".to_string());
//! let v2: Vec<u32> = from_str("#(1 2 3)").unwrap();
//! assert_eq!(v1, v2);
//! assert_eq!(to_string(&v2).unwrap(), "(1 2 3)".to_string());
//! ```
//!
//! # Option
//!
//! The two variants of the `Option` type are represented as empty list (`None`)
//! or a single-element list (`Some(x)`). This representation is chosen for
//! unambiguity over using a special "missing" value (such as `#nil`) for `None`
//! and a plain value for `Some`. It also combines nicely with struct fields
//! containing option values.
//!
//! ```
//! use serde_lexpr::{from_str, to_string};
//!
//! let answer: Option<u32> = from_str("(42)").unwrap();
//! assert_eq!(answer, Some(42));
//! let no_answer: Option<u32> = from_str("()").unwrap();
//! assert_eq!(no_answer, None);
//! ```
//!
//! ## Tuples and tuple structs
//!
//! Tuples and tuple structs are serialized as S-expression vectors.
//!
//! ```
//! use serde_lexpr::{from_str, to_string};
//! use serde_derive::{Serialize, Deserialize};
//!
//! assert_eq!(to_string(&(1, "two", 3)).unwrap(), "#(1 \"two\" 3)".to_string());
//! let tuple: (u8, String, u64) = from_str("(1 \"two\" 3)").unwrap();
//! assert_eq!(tuple, (1, "two".to_string(), 3));
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
//! struct Person(String, u8);
//!
//! assert_eq!(to_string(&Person("Billy".into(), 42)).unwrap(), "#(\"Billy\" 42)".to_string());
//! let joanne: Person = from_str("#(\"Joanne\" 23)").unwrap();
//! assert_eq!(joanne, Person("Joanne".into(), 23));
//! ```
//!
//! ## Structs
//!
//! Structs are serialized as association lists, i.e. a list consisting of cons
//! cells, where each cell's `car` is the name of the struct field (as a
//! symbol), and the `cdr` containing the field's value.
//!
//! ```
//! use serde_lexpr::{from_str, to_string};
//! use serde_derive::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
//! struct Person {
//!   name: String,
//!   age: u8,
//! };
//!
//! let billy = Person { name: "Billy".into(), age: 42 };
//! assert_eq!(to_string(&billy).unwrap(), "((name . \"Billy\") (age . 42))".to_string());
//! let joanne: Person = from_str("((name . \"Joanne\") (age . 23))").unwrap();
//! assert_eq!(joanne, Person { name: "Joanne".into(), age: 23 });
//! ```
//!
//! ## Enums
//!
//! Enum variants without data are serialized as plain symbols. Tuple variants
//! are serialized as a list starting with the variant name as a symbol,
//! followed by the values. This representation is chosen over using a vector to
//! keep the emitted S-expressions less noisy and hopefully a bit more
//! ideomatic. Struct variants are serialized like structs (i.e. as association
//! lists), but have the variant name prepended as a symbol.
//!
//! ```
//! use serde_lexpr::{from_str, to_string};
//! use serde_derive::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
//! #[serde(rename_all = "kebab-case")]
//! enum Choice {
//!   Yes,
//!   No,
//!   Other(String),
//! }
//!
//! let choices: Vec<Choice> = from_str("(yes no (other . \"foo\"))").unwrap();
//! assert_eq!(choices, vec![Choice::Yes, Choice::No, Choice::Other("foo".into())]);
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
//! #[serde(rename_all = "kebab-case")]
//! enum Example {
//!   Unit,
//!   Newtype(u32),
//!   NewtypeOption(Option<u32>),
//!   Tuple(u32, u32),
//!   Struct { foo: bool, bar: u32 },
//! }
//!
//! let unit: Example = from_str("unit").unwrap();
//! assert_eq!(unit, Example::Unit);
//!
//! let newtype: Example = from_str("(newtype . 42)").unwrap();
//! assert_eq!(newtype, Example::Newtype(42));
//!
//! let newtype_some: Example = from_str("(newtype-option 23)").unwrap();
//! assert_eq!(newtype_some, Example::NewtypeOption(Some(23)));
//!
//! let newtype_none: Example = from_str("(newtype-option)").unwrap();
//! assert_eq!(newtype_none, Example::NewtypeOption(None));
//!
//! let tuple: Example = from_str("(tuple 1 2)").unwrap();
//! assert_eq!(tuple, Example::Tuple(1, 2));
//!
//! let struct_variant: Example = from_str("(struct (foo . #t) (bar . 3))").unwrap();
//! assert_eq!(struct_variant, Example::Struct { foo: true, bar: 3 });
//! ```
//!
//! [Serde]: https://crates.io/crates/serde
//! [`lexpr::Value`]: https://docs.rs/lexpr/*/lexpr/enum.Value.html

mod de;
mod error;
mod ser;
mod value;

pub use de::from_str;
pub use error::{Error, Result};
pub use ser::to_string;
pub use value::{from_value, to_value, Cons, Value};

// This is exposed for convenience (allowing importing via `serde_lexpr`) and so
// that links from the `Value` documentation work.
pub use lexpr::parse;