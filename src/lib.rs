//! Keys that serve as a means of accessing an object in a map.
use std::{hash::Hash, marker::PhantomData, sync::Arc};

/// A handle key.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum HandleKey {
    Str(&'static str),
    String(String),
    Number(usize),
}

impl From<String> for HandleKey {
    fn from(s: String) -> Self {
        HandleKey::String(s)
    }
}

impl From<&String> for HandleKey {
    fn from(s: &String) -> Self {
        HandleKey::String(s.clone())
    }
}

impl From<usize> for HandleKey {
    fn from(k: usize) -> Self {
        HandleKey::Number(k)
    }
}

impl From<&str> for HandleKey {
    fn from(s: &str) -> Self {
        HandleKey::from(s.to_string())
    }
}

/// A typed asset handle.
pub struct Handle<T> {
    // Underlying key used for comparison
    pub key: HandleKey,
    // Used to count how many things own a clone of the handle.
    pub count: Option<Arc<()>>,
    _phantom: PhantomData<T>,
}

impl<T> std::fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Handle<{}>", std::any::type_name::<T>()))
            .field("key", &self.key)
            .field(
                "references",
                &format!("{:?}", self.count.as_ref().map(|c| Arc::strong_count(c))),
            )
            .finish()
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle {
            key: self.key.clone(),
            count: self.count.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Hash for Handle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for Handle<T> {
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T> Handle<T> {
    pub fn new<K>(k: K) -> Self
    where
        HandleKey: From<K>,
    {
        Handle {
            key: HandleKey::from(k),
            count: Some(Arc::new(())),
            _phantom: PhantomData,
        }
    }

    pub const fn from_static(key: &'static str) -> Handle<T> {
        Handle {
            key: HandleKey::Str(key),
            count: None,
            _phantom: PhantomData,
        }
    }
}
