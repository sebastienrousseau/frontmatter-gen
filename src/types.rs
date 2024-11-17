// types.rs

//! This module defines the core types used throughout the frontmatter-gen crate.
//! It includes the `Format` enum for representing different frontmatter formats, the `Value` enum for representing various data types that can be stored in frontmatter, and the `Frontmatter` struct which is the main container for frontmatter data.

use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fmt,
    str::FromStr,
};

/// Represents the different formats supported for frontmatter serialization/deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Format {
    /// YAML format.
    Yaml,
    /// TOML format.
    Toml,
    /// JSON format.
    Json,
    /// Unsupported format.
    Unsupported,
}

impl Default for Format {
    fn default() -> Self {
        Format::Json
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let format_str = match self {
            Format::Yaml => "YAML",
            Format::Toml => "TOML",
            Format::Json => "JSON",
            Format::Unsupported => "Unsupported",
        };
        write!(f, "{}", format_str)
    }
}

/// A flexible value type that can hold various types of data found in frontmatter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Value {
    /// Represents a null value.
    Null,
    /// Represents a string value.
    String(String),
    /// Represents a numeric value.
    Number(f64),
    /// Represents a boolean value.
    Boolean(bool),
    /// Represents an array of values.
    Array(Vec<Value>),
    /// Represents an object (frontmatter).
    Object(Box<Frontmatter>),
    /// Represents a tagged value, containing a tag and a value.
    Tagged(String, Box<Value>),
}

impl Value {
    /// Returns the value as a string slice, if it is of type `String`.
    ///
    /// # Returns
    ///
    /// - `Some(&str)` if the value is a `String`.
    /// - `None` if the value is not a `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let string_value = Value::String("Hello".to_string());
    /// assert_eq!(string_value.as_str(), Some("Hello"));
    ///
    /// let number_value = Value::Number(42.0);
    /// assert_eq!(number_value.as_str(), None);
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        if let Value::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Returns the value as a float, if it is of type `Number`.
    ///
    /// # Returns
    ///
    /// - `Some(f64)` if the value is a `Number`.
    /// - `None` if the value is not a `Number`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let number_value = Value::Number(3.14);
    /// assert_eq!(number_value.as_f64(), Some(3.14));
    ///
    /// let string_value = Value::String("Not a number".to_string());
    /// assert_eq!(string_value.as_f64(), None);
    /// ```
    pub fn as_f64(&self) -> Option<f64> {
        if let Value::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    /// Returns the value as a boolean, if it is of type `Boolean`.
    ///
    /// # Returns
    ///
    /// - `Some(bool)` if the value is a `Boolean`.
    /// - `None` if the value is not a `Boolean`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let bool_value = Value::Boolean(true);
    /// assert_eq!(bool_value.as_bool(), Some(true));
    ///
    /// let string_value = Value::String("Not a boolean".to_string());
    /// assert_eq!(string_value.as_bool(), None);
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Boolean(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    /// Returns the value as an array, if it is of type `Array`.
    ///
    /// # Returns
    ///
    /// - `Some(&Vec<Value>)` if the value is an `Array`.
    /// - `None` if the value is not an `Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let array_value = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    /// assert!(array_value.as_array().is_some());
    /// assert_eq!(array_value.as_array().unwrap().len(), 2);
    ///
    /// let string_value = Value::String("Not an array".to_string());
    /// assert!(string_value.as_array().is_none());
    /// ```
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        if let Value::Array(arr) = self {
            Some(arr)
        } else {
            None
        }
    }

    /// Returns the value as an object (frontmatter), if it is of type `Object`.
    ///
    /// # Returns
    ///
    /// - `Some(&Frontmatter)` if the value is an `Object`.
    /// - `None` if the value is not an `Object`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Value, Frontmatter};
    ///
    /// let mut fm = Frontmatter::new();
    /// fm.insert("key".to_string(), Value::String("value".to_string()));
    /// let object_value = Value::Object(Box::new(fm));
    /// assert!(object_value.as_object().is_some());
    ///
    /// let string_value = Value::String("Not an object".to_string());
    /// assert!(string_value.as_object().is_none());
    /// ```
    pub fn as_object(&self) -> Option<&Frontmatter> {
        if let Value::Object(obj) = self {
            Some(obj)
        } else {
            None
        }
    }

    /// Returns the value as a tagged value, if it is of type `Tagged`.
    ///
    /// # Returns
    ///
    /// - `Some((&str, &Value))` if the value is `Tagged`.
    /// - `None` if the value is not `Tagged`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let tagged_value = Value::Tagged("tag".to_string(), Box::new(Value::Number(42.0)));
    /// assert_eq!(tagged_value.as_tagged(), Some(("tag", &Value::Number(42.0))));
    ///
    /// let string_value = Value::String("Not tagged".to_string());
    /// assert_eq!(string_value.as_tagged(), None);
    /// ```
    pub fn as_tagged(&self) -> Option<(&str, &Value)> {
        if let Value::Tagged(tag, val) = self {
            Some((tag, val))
        } else {
            None
        }
    }

    /// Checks if the value is of type `Null`.
    ///
    /// # Returns
    ///
    /// `true` if the value is `Null`, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let null_value = Value::Null;
    /// assert!(null_value.is_null());
    ///
    /// let string_value = Value::String("Not null".to_string());
    /// assert!(!string_value.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Checks if the value is of type `String`.
    ///
    /// # Returns
    ///
    /// `true` if the value is a `String`, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let string_value = Value::String("Hello".to_string());
    /// assert!(string_value.is_string());
    ///
    /// let number_value = Value::Number(42.0);
    /// assert!(!number_value.is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Checks if the value is of type `Number`.
    ///
    /// # Returns
    ///
    /// `true` if the value is a `Number`, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let number_value = Value::Number(3.14);
    /// assert!(number_value.is_number());
    ///
    /// let string_value = Value::String("Not a number".to_string());
    /// assert!(!string_value.is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    /// Checks if the value is of type `Boolean`.
    ///
    /// # Returns
    ///
    /// `true` if the value is a `Boolean`, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let bool_value = Value::Boolean(true);
    /// assert!(bool_value.is_boolean());
    ///
    /// let string_value = Value::String("Not a boolean".to_string());
    /// assert!(!string_value.is_boolean());
    /// ```
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    /// Checks if the value is of type `Array`.
    ///
    /// # Returns
    ///
    /// `true` if the value is an `Array`, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let array_value = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    /// assert!(array_value.is_array());
    ///
    /// let string_value = Value::String("Not an array".to_string());
    /// assert!(!string_value.is_array());
    /// ```
    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    /// Checks if the value is of type `Object`.
    ///
    /// # Returns
    ///
    /// `true` if the value is an `Object`, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Value, Frontmatter};
    ///
    /// let object_value = Value::Object(Box::new(Frontmatter::new()));
    /// assert!(object_value.is_object());
    ///
    /// let string_value = Value::String("Not an object".to_string());
    /// assert!(!string_value.is_object());
    /// ```
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    /// Checks if the value is of type `Tagged`.
    ///
    /// # Returns
    ///
    /// `true` if the value is `Tagged`, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let tagged_value = Value::Tagged("tag".to_string(), Box::new(Value::Number(42.0)));
    /// assert!(tagged_value.is_tagged());
    ///
    /// let string_value = Value::String("Not tagged".to_string());
    /// assert!(!string_value.is_tagged());
    /// ```
    pub fn is_tagged(&self) -> bool {
        matches!(self, Value::Tagged(_, _))
    }

    /// Returns the length of the array if the value is an array, otherwise returns `None`.
    ///
    /// # Returns
    ///
    /// - `Some(usize)` with the length of the array if the value is an `Array`.
    /// - `None` if the value is not an `Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let array_value = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    /// assert_eq!(array_value.array_len(), Some(2));
    ///
    /// let string_value = Value::String("Not an array".to_string());
    /// assert_eq!(string_value.array_len(), None);
    /// ```
    pub fn array_len(&self) -> Option<usize> {
        if let Value::Array(arr) = self {
            Some(arr.len())
        } else {
            None
        }
    }

    /// Attempts to convert the value into a `Frontmatter`.
    ///
    /// # Returns
    ///
    /// - `Ok(Frontmatter)` if the value is an `Object`.
    /// - `Err(String)` with an error message if the value is not an `Object`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Value, Frontmatter};
    ///
    /// let object_value = Value::Object(Box::new(Frontmatter::new()));
    /// assert!(object_value.to_object().is_ok());
    ///
    /// let string_value = Value::String("Not an object".to_string());
    /// assert!(string_value.to_object().is_err());
    /// ```
    pub fn to_object(self) -> Result<Frontmatter, String> {
        if let Value::Object(obj) = self {
            Ok(*obj)
        } else {
            Err("Value is not an object".into())
        }
    }

    /// Converts the value to a string representation regardless of its type.
    ///
    /// # Returns
    ///
    /// A `String` representation of the value.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let number_value = Value::Number(3.14);
    /// assert_eq!(number_value.to_string_representation(), "3.14");
    ///
    /// let string_value = Value::String("Hello".to_string());
    /// assert_eq!(string_value.to_string_representation(), "\"Hello\"");
    /// ```
    pub fn to_string_representation(&self) -> String {
        format!("{}", self)
    }

    /// Attempts to convert the value into a `String`.
    ///
    /// # Returns
    ///
    /// - `Ok(String)` if the value is a `String`.
    /// - `Err(String)` with an error message if the value is not a `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let string_value = Value::String("Hello".to_string());
    /// assert_eq!(string_value.into_string(), Ok("Hello".to_string()));
    ///
    /// let number_value = Value::Number(42.0);
    /// assert!(number_value.into_string().is_err());
    /// ```
    pub fn into_string(self) -> Result<String, String> {
        if let Value::String(s) = self {
            Ok(s)
        } else {
            Err("Value is not a string".into())
        }
    }

    /// Attempts to convert the value into an `f64`.
    ///
    /// # Returns
    ///
    /// - `Ok(f64)` if the value is a `Number`.
    /// - `Err(String)` with an error message if the value is not a `Number`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let number_value = Value::Number(3.14);
    /// assert_eq!(number_value.into_f64(), Ok(3.14));
    ///
    /// let string_value = Value::String("Not a number".to_string());
    /// assert!(string_value.into_f64().is_err());
    /// ```
    pub fn into_f64(self) -> Result<f64, String> {
        if let Value::Number(n) = self {
            Ok(n)
        } else {
            Err("Value is not a number".into())
        }
    }

    /// Attempts to convert the value into a `bool`.
    ///
    /// # Returns
    ///
    /// - `Ok(bool)` if the value is a `Boolean`.
    /// - `Err(String)` with an error message if the value is not a `Boolean`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let bool_value = Value::Boolean(true);
    /// assert_eq!(bool_value.into_bool(), Ok(true));
    ///
    /// let string_value = Value::String("Not a boolean".to_string());
    /// assert!(string_value.into_bool().is_err());
    /// ```
    pub fn into_bool(self) -> Result<bool, String> {
        if let Value::Boolean(b) = self {
            Ok(b)
        } else {
            Err("Value is not a boolean".into())
        }
    }

    /// Attempts to get a mutable reference to the array if the value is an array.
    ///
    /// # Returns
    ///
    /// - `Some(&mut Vec<Value>)` if the value is an `Array`.
    /// - `None` if the value is not an `Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Value;
    ///
    /// let mut array_value = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    /// if let Some(arr) = array_value.get_mut_array() {
    ///     arr.push(Value::Number(3.0));
    /// }
    /// assert_eq!(array_value.array_len(), Some(3));
    ///
    /// let mut string_value = Value::String("Not an array".to_string());
    /// assert!(string_value.get_mut_array().is_none());
    /// ```
    pub fn get_mut_array(&mut self) -> Option<&mut Vec<Value>> {
        if let Value::Array(arr) = self {
            Some(arr)
        } else {
            None
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

impl FromIterator<Value> for Value {
    fn from_iter<I: IntoIterator<Item = Value>>(iter: I) -> Self {
        Value::Array(iter.into_iter().collect())
    }
}

impl FromStr for Value {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("null") {
            Ok(Value::Null)
        } else if s.eq_ignore_ascii_case("true") {
            Ok(Value::Boolean(true))
        } else if s.eq_ignore_ascii_case("false") {
            Ok(Value::Boolean(false))
        } else if let Ok(n) = s.parse::<f64>() {
            Ok(Value::Number(n))
        } else {
            Ok(Value::String(s.to_string()))
        }
    }
}

/// Represents the frontmatter, a collection of key-value pairs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Frontmatter(pub HashMap<String, Value>);

impl Frontmatter {
    /// Creates a new, empty frontmatter.
    ///
    /// # Returns
    ///
    /// A new `Frontmatter` instance with no key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::Frontmatter;
    ///
    /// let fm = Frontmatter::new();
    /// assert!(fm.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Frontmatter(HashMap::new())
    }

    /// Inserts a key-value pair into the frontmatter.
    ///
    /// # Arguments
    ///
    /// * `key` - The key for the entry.
    /// * `value` - The value associated with the key.
    ///
    /// # Returns
    ///
    /// An option containing the old value if it was replaced.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Frontmatter, Value};
    ///
    /// let mut fm = Frontmatter::new();
    /// assert_eq!(fm.insert("key".to_string(), Value::String("value".to_string())), None);
    /// assert_eq!(fm.insert("key".to_string(), Value::Number(42.0)), Some(Value::String("value".to_string())));
    /// ```
    pub fn insert(
        &mut self,
        key: String,
        value: Value,
    ) -> Option<Value> {
        self.0.insert(key, value)
    }

    /// Retrieves a reference to a value associated with a key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the value if the key exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Frontmatter, Value};
    ///
    /// let mut fm = Frontmatter::new();
    /// fm.insert("key".to_string(), Value::String("value".to_string()));
    /// assert_eq!(fm.get("key"), Some(&Value::String("value".to_string())));
    /// assert_eq!(fm.get("nonexistent"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    /// Retrieves a mutable reference to a value associated with a key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up.
    ///
    /// # Returns
    ///
    /// An option containing a mutable reference to the value if the key exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Frontmatter, Value};
    ///
    /// let mut fm = Frontmatter::new();
    /// fm.insert("key".to_string(), Value::String("value".to_string()));
    /// if let Some(value) = fm.get_mut("key") {
    ///     *value = Value::Number(42.0);
    /// }
    /// assert_eq!(fm.get("key"), Some(&Value::Number(42.0)));
    /// ```
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.0.get_mut(key)
    }

    /// Removes a key-value pair from the frontmatter.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove.
    ///
    /// # Returns
    ///
    /// An option containing the removed value if the key existed.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Frontmatter, Value};
    ///
    /// let mut fm = Frontmatter::new();
    /// fm.insert("key".to_string(), Value::String("value".to_string()));
    /// assert_eq!(fm.remove("key"), Some(Value::String("value".to_string())));
    /// assert_eq!(fm.remove("key"), None);
    /// ```
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.0.remove(key)
    }

    /// Checks if the frontmatter contains a given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check for.
    ///
    /// # Returns
    ///
    /// `true` if the key exists in the frontmatter, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Frontmatter, Value};
    ///
    /// let mut fm = Frontmatter::new();
    /// fm.insert("key".to_string(), Value::String("value".to_string()));
    /// assert!(fm.contains_key("key"));
    /// assert!(!fm.contains_key("nonexistent"));
    /// ```
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Returns the number of entries in the frontmatter.
    ///
    /// # Returns
    ///
    /// The number of key-value pairs in the frontmatter.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Frontmatter, Value};
    ///
    /// let mut fm = Frontmatter::new();
    /// assert_eq!(fm.len(), 0);
    /// fm.insert("key".to_string(), Value::String("value".to_string()));
    /// assert_eq!(fm.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Checks if the frontmatter is empty.
    ///
    /// # Returns
    ///
    /// `true` if the frontmatter contains no key-value pairs, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Frontmatter, Value};
    ///
    /// let mut fm = Frontmatter::new();
    /// assert!(fm.is_empty());
    /// fm.insert("key".to_string(), Value::String("value".to_string()));
    /// assert!(!fm.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over the key-value pairs of the frontmatter.
    ///
    /// # Returns
    ///
    /// An iterator over references to the key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Frontmatter, Value};
    ///
    /// let mut fm = Frontmatter::new();
    /// fm.insert("key1".to_string(), Value::String("value1".to_string()));
    /// fm.insert("key2".to_string(), Value::Number(42.0));
    ///
    /// for (key, value) in fm.iter() {
    ///     println!("{}: {:?}", key, value);
    /// }
    /// ```
    pub fn iter(
        &self,
    ) -> std::collections::hash_map::Iter<String, Value> {
        self.0.iter()
    }

    /// Returns a mutable iterator over the key-value pairs of the frontmatter.
    ///
    /// # Returns
    ///
    /// A mutable iterator over references to the key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Frontmatter, Value};
    ///
    /// let mut fm = Frontmatter::new();
    /// fm.insert("key1".to_string(), Value::String("value1".to_string()));
    /// fm.insert("key2".to_string(), Value::Number(42.0));
    ///
    /// for (_, value) in fm.iter_mut() {
    ///     if let Value::Number(n) = value {
    ///         *n += 1.0;
    ///     }
    /// }
    ///
    /// assert_eq!(fm.get("key2"), Some(&Value::Number(43.0)));
    /// ```
    pub fn iter_mut(
        &mut self,
    ) -> std::collections::hash_map::IterMut<String, Value> {
        self.0.iter_mut()
    }

    /// Merges another frontmatter into this one. If a key exists, it will be overwritten.
    ///
    /// # Arguments
    ///
    /// * `other` - The frontmatter to merge into this one.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Frontmatter, Value};
    ///
    /// let mut fm1 = Frontmatter::new();
    /// fm1.insert("key1".to_string(), Value::String("value1".to_string()));
    ///
    /// let mut fm2 = Frontmatter::new();
    /// fm2.insert("key2".to_string(), Value::Number(42.0));
    ///
    /// fm1.merge(fm2);
    /// assert_eq!(fm1.len(), 2);
    /// assert_eq!(fm1.get("key2"), Some(&Value::Number(42.0)));
    /// ```
    pub fn merge(&mut self, other: Frontmatter) {
        self.0.extend(other.0);
    }

    /// Checks if a given key exists and its value is `null`.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check.
    ///
    /// # Returns
    ///
    /// `true` if the key exists and its value is `null`, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Frontmatter, Value};
    ///
    /// let mut fm = Frontmatter::new();
    /// fm.insert("null_key".to_string(), Value::Null);
    /// fm.insert("non_null_key".to_string(), Value::String("value".to_string()));
    ///
    /// assert!(fm.is_null("null_key"));
    /// assert!(!fm.is_null("non_null_key"));
    /// assert!(!fm.is_null("nonexistent_key"));
    /// ```
    pub fn is_null(&self, key: &str) -> bool {
        matches!(self.get(key), Some(Value::Null))
    }

    /// Clears the frontmatter while preserving allocated capacity
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns the current capacity of the underlying HashMap
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Reserves capacity for at least `additional` more elements
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }
}

impl Default for Frontmatter {
    fn default() -> Self {
        Self(HashMap::with_capacity(8))
    }
}

/// Implement `IntoIterator` for `Frontmatter` to allow idiomatic iteration.
impl IntoIterator for Frontmatter {
    type Item = (String, Value);
    type IntoIter = std::collections::hash_map::IntoIter<String, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Implement `FromIterator` for `Frontmatter` to create a frontmatter from an iterator.
impl FromIterator<(String, Value)> for Frontmatter {
    /// Creates a `Frontmatter` from an iterator of key-value pairs.
    ///
    /// # Arguments
    ///
    /// * `iter` - An iterator of key-value pairs where the key is a `String` and the value is a `Value`.
    ///
    /// # Returns
    ///
    /// A `Frontmatter` containing the key-value pairs from the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::{Frontmatter, Value};
    /// use std::iter::FromIterator;
    ///
    /// let pairs = vec![
    ///     ("key1".to_string(), Value::String("value1".to_string())),
    ///     ("key2".to_string(), Value::Number(42.0)),
    /// ];
    ///
    /// let fm = Frontmatter::from_iter(pairs);
    /// assert_eq!(fm.len(), 2);
    /// assert_eq!(fm.get("key1"), Some(&Value::String("value1".to_string())));
    /// assert_eq!(fm.get("key2"), Some(&Value::Number(42.0)));
    /// ```
    fn from_iter<I: IntoIterator<Item = (String, Value)>>(
        iter: I,
    ) -> Self {
        let mut fm = Frontmatter::new();
        for (key, value) in iter {
            let _ = fm.insert(key, value);
        }
        fm
    }
}

/// Implement `Display` for `Frontmatter` to allow easy printing with escaped characters.
impl fmt::Display for Frontmatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        // Use a BTreeMap to ensure consistent key order (sorted by key)
        let mut sorted_map = BTreeMap::new();
        for (key, value) in &self.0 {
            let _ = sorted_map.insert(key, value);
        }

        for (i, (key, value)) in sorted_map.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "\"{}\": {}", escape_str(key), value)?;
        }

        write!(f, "}}")
    }
}

/// Implement `Display` for `Value` to allow easy printing with escaped characters.
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::String(s) => write!(f, "\"{}\"", escape_str(s)),
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{:.0}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Object(obj) => write!(f, "{}", obj),
            Value::Tagged(tag, val) => {
                write!(f, "\"{}\": {}", escape_str(tag), val)
            }
        }
    }
}

/// Escapes special characters in a string (e.g., backslashes and quotes).
///
/// # Arguments
///
/// * `s` - The input string to escape.
///
/// # Returns
///
/// A new `String` with special characters escaped.
///
/// # Examples
///
/// ```
/// use frontmatter_gen::types::escape_str;
///
/// assert_eq!(escape_str(r#"Hello "World""#), r#"Hello \"World\""#);
/// assert_eq!(escape_str(r#"C:\path\to\file"#), r#"C:\\path\\to\\file"#);
/// ```
pub fn escape_str(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            _ => escaped.push(c),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    mod format_tests {
        use super::*;

        #[test]
        fn test_format_default() {
            assert_eq!(Format::default(), Format::Json);
        }
    }

    mod value_tests {
        use super::*;

        #[test]
        fn test_value_default() {
            assert_eq!(Value::default(), Value::Null);
        }

        #[test]
        fn test_value_as_str() {
            let value = Value::String("Hello".to_string());
            assert_eq!(value.as_str(), Some("Hello"));

            let value = Value::Number(42.0);
            assert_eq!(value.as_str(), None);
        }

        #[test]
        fn test_value_as_f64() {
            let value = Value::Number(42.0);
            assert_eq!(value.as_f64(), Some(42.0));

            let value = Value::String("Not a number".to_string());
            assert_eq!(value.as_f64(), None);
        }

        #[test]
        fn test_value_as_bool() {
            let value = Value::Boolean(true);
            assert_eq!(value.as_bool(), Some(true));

            let value = Value::String("Not a boolean".to_string());
            assert_eq!(value.as_bool(), None);
        }

        #[test]
        fn test_value_is_null() {
            assert!(Value::Null.is_null());
            assert!(!Value::String("Not null".to_string()).is_null());
        }

        #[test]
        fn test_value_is_string() {
            assert!(Value::String("test".to_string()).is_string());
            assert!(!Value::Number(42.0).is_string());
        }

        #[test]
        fn test_value_is_number() {
            assert!(Value::Number(42.0).is_number());
            assert!(!Value::String("42".to_string()).is_number());
        }

        #[test]
        fn test_value_is_boolean() {
            assert!(Value::Boolean(true).is_boolean());
            assert!(!Value::String("true".to_string()).is_boolean());
        }

        #[test]
        fn test_value_as_array() {
            let value =
                Value::Array(vec![Value::Null, Value::Boolean(false)]);
            assert!(value.as_array().is_some());
            assert_eq!(value.as_array().unwrap().len(), 2);

            assert!(Value::String("Not an array".to_string())
                .as_array()
                .is_none());
        }

        #[test]
        fn test_value_as_object() {
            let mut fm = Frontmatter::new();
            let _ = fm.insert(
                "key".to_string(),
                Value::String("value".to_string()),
            );
            let value = Value::Object(Box::new(fm.clone()));
            assert_eq!(value.as_object().unwrap(), &fm);

            assert!(Value::String("Not an object".to_string())
                .as_object()
                .is_none());
        }

        #[test]
        fn test_value_to_object() {
            let fm = Frontmatter::new();
            let obj = Value::Object(Box::new(fm.clone()));
            assert_eq!(obj.to_object().unwrap(), fm);

            assert!(Value::String("Not an object".to_string())
                .to_object()
                .is_err());
        }

        #[test]
        fn test_value_to_string_representation() {
            assert_eq!(
                Value::String("test".to_string())
                    .to_string_representation(),
                "\"test\""
            );
            assert_eq!(
                Value::Number(42.0).to_string_representation(),
                "42"
            );
            assert_eq!(
                Value::Boolean(true).to_string_representation(),
                "true"
            );
        }

        #[test]
        fn test_value_display() {
            assert_eq!(format!("{}", Value::Null), "null");
            assert_eq!(
                format!("{}", Value::String("test".to_string())),
                "\"test\""
            );
            assert_eq!(
                format!("{}", Value::Number(PI)),
                format!("{}", PI)
            );
            assert_eq!(format!("{}", Value::Boolean(true)), "true");
        }
    }

    mod frontmatter_tests {
        use super::*;

        #[test]
        fn test_frontmatter_new() {
            let fm = Frontmatter::new();
            assert!(fm.is_empty());
            assert_eq!(fm.len(), 0);
        }

        #[test]
        fn test_frontmatter_insert_and_get() {
            let mut fm = Frontmatter::new();
            let _ = fm.insert(
                "title".to_string(),
                Value::String("Hello World".to_string()),
            );

            assert_eq!(
                fm.get("title"),
                Some(&Value::String("Hello World".to_string()))
            );
        }

        #[test]
        fn test_frontmatter_len_and_is_empty() {
            let mut fm = Frontmatter::new();
            assert!(fm.is_empty());

            let _ = fm.insert("key1".to_string(), Value::Null);
            assert_eq!(fm.len(), 1);
            assert!(!fm.is_empty());
        }

        #[test]
        fn test_frontmatter_merge() {
            let mut fm1 = Frontmatter::new();
            let _ = fm1.insert(
                "key1".to_string(),
                Value::String("value1".to_string()),
            );

            let mut fm2 = Frontmatter::new();
            let _ = fm2.insert("key2".to_string(), Value::Number(42.0));

            fm1.merge(fm2);
            assert_eq!(fm1.len(), 2);
            assert_eq!(fm1.get("key2"), Some(&Value::Number(42.0)));
        }

        #[test]
        fn test_frontmatter_display() {
            let mut fm = Frontmatter::new();
            let _ = fm.insert(
                "key1".to_string(),
                Value::String("value1".to_string()),
            );
            let _ = fm.insert("key2".to_string(), Value::Number(42.0));
            let display = format!("{}", fm);

            assert!(display.contains("\"key1\": \"value1\""));
            assert!(display.contains("\"key2\": 42"));
        }

        #[test]
        fn test_frontmatter_is_null() {
            let mut fm = Frontmatter::new();
            let _ = fm.insert("key".to_string(), Value::Null);

            assert!(fm.is_null("key"));
            assert!(!fm.is_null("nonexistent_key"));
        }
    }

    mod utility_tests {
        use super::*;

        #[test]
        fn test_escape_str() {
            assert_eq!(
                escape_str(r#"Hello "World""#),
                r#"Hello \"World\""#
            );
            assert_eq!(
                escape_str(r#"C:\path\to\file"#),
                r#"C:\\path\\to\\file"#
            );
        }

        #[test]
        fn test_escape_str_empty() {
            assert_eq!(escape_str(""), "");
        }
    }

    mod additional_tests {
        use super::*;

        #[test]
        fn test_frontmatter_clear() {
            let mut fm = Frontmatter::new();
            let _ = fm.insert(
                "key1".to_string(),
                Value::String("value1".to_string()),
            );
            let _ = fm.insert("key2".to_string(), Value::Number(42.0));

            fm.clear();
            assert!(fm.is_empty());
            assert_eq!(fm.len(), 0);
        }

        #[test]
        fn test_frontmatter_capacity_and_reserve() {
            let mut fm = Frontmatter::new();
            let initial_capacity = fm.capacity();

            fm.reserve(10);
            assert!(fm.capacity() >= initial_capacity + 10);
        }

        #[test]
        fn test_value_tagged() {
            let tagged_value = Value::Tagged(
                "tag".to_string(),
                Box::new(Value::Number(42.0)),
            );

            if let Value::Tagged(tag, value) = tagged_value {
                assert_eq!(tag, "tag");
                assert_eq!(*value, Value::Number(42.0));
            } else {
                panic!("Expected Value::Tagged");
            }
        }

        #[test]
        fn test_value_array_mutation() {
            let mut value = Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
            ]);

            if let Some(array) = value.get_mut_array() {
                array.push(Value::Number(3.0));
            }

            assert_eq!(value.array_len(), Some(3));
            assert!(value
                .as_array()
                .unwrap()
                .contains(&Value::Number(3.0)));
        }

        #[test]
        fn test_value_conversion_errors() {
            let value = Value::Boolean(true);
            assert!(value.clone().into_f64().is_err());
            assert!(value.into_string().is_err());

            let value = Value::Number(42.0);
            assert!(value.into_bool().is_err());
        }

        #[test]
        fn test_value_from_str_error_handling() {
            assert_eq!("null".parse::<Value>().unwrap(), Value::Null);
            assert_eq!(
                "true".parse::<Value>().unwrap(),
                Value::Boolean(true)
            );
            assert_eq!(
                "false".parse::<Value>().unwrap(),
                Value::Boolean(false)
            );

            let invalid_number = "abc123".parse::<Value>();
            assert!(invalid_number.is_ok()); // Treated as a string.
            assert_eq!(
                invalid_number.unwrap(),
                Value::String("abc123".to_string())
            );
        }

        #[test]
        fn test_frontmatter_empty_iterator() {
            let fm = Frontmatter::new();
            let mut iter = fm.iter();

            assert!(iter.next().is_none());
        }

        #[test]
        fn test_frontmatter_duplicate_merge() {
            let mut fm1 = Frontmatter::new();
            let _ = fm1.insert(
                "key1".to_string(),
                Value::String("value1".to_string()),
            );

            let mut fm2 = Frontmatter::new();
            let _ = fm2.insert(
                "key1".to_string(),
                Value::String("new_value".to_string()),
            );

            fm1.merge(fm2);

            assert_eq!(
                fm1.get("key1"),
                Some(&Value::String("new_value".to_string()))
            );
        }

        #[test]
        fn test_display_for_empty_frontmatter() {
            let fm = Frontmatter::new();
            let display = format!("{}", fm);
            assert_eq!(display, "{}");
        }

        #[test]
        fn test_value_from_iterator_empty() {
            let vec: Vec<Value> = vec![];
            let array_value: Value = vec.into_iter().collect();
            assert_eq!(array_value, Value::Array(vec![]));
        }

        #[test]
        fn test_escape_str_edge_cases() {
            let special_chars = r#"Special \chars\n\t"#;
            assert_eq!(
                escape_str(special_chars),
                r#"Special \\chars\\n\\t"#
            );
        }
    }
}
