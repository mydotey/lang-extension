use std::sync::Arc;

use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct ImmutableValue {
    value: Arc<Box<dyn Value>>,
}

impl ImmutableValue {
    pub fn new<T: Value>(value: T) -> ImmutableValue {
        Self::wrap(Value::to_boxed(value))
    }

    pub fn wrap(value: Box<dyn Value>) -> Self {
        ImmutableValue {
            value: Arc::new(value),
        }
    }

    pub fn raw_boxed(&self) -> Box<dyn Value> {
        self.value.as_ref().clone()
    }
}

unsafe impl Send for ImmutableValue {}

unsafe impl Sync for ImmutableValue {}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct ImmutableKey {
    key: Arc<Box<dyn Key>>,
}

impl ImmutableKey {
    pub fn new<T: Key>(value: T) -> ImmutableKey {
        Self::wrap(Key::to_boxed(value))
    }

    pub fn wrap(value: Box<dyn Key>) -> Self {
        ImmutableKey {
            key: Arc::new(value),
        }
    }

    pub fn raw_boxed(&self) -> Box<dyn Key> {
        self.key.as_ref().clone()
    }
}

unsafe impl Send for ImmutableKey {}

unsafe impl Sync for ImmutableKey {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::thread;

    #[test]
    fn immutable_value() {
        assert_eq!(ImmutableValue::new("value"), ImmutableValue::new("value"));
        assert_ne!(ImmutableValue::new("value"), ImmutableValue::new("value2"));

        println!("{:?}", ImmutableValue::new("value"));

        let v = ImmutableValue::new("value");
        assert_eq!(&v, &v.clone());

        let b: Box<dyn Value> = Box::new("value");
        assert_eq!(
            ImmutableValue::new("value"),
            ImmutableValue::wrap(b.clone())
        );

        assert_eq!(&b, &v.raw_boxed());

        let clone = v.clone();
        let handle = thread::spawn(move || {
            println!("data: {:?}", clone);
        });
        handle.join().unwrap();
    }

    #[test]
    fn immutable_key() {
        assert_eq!(ImmutableKey::new("key"), ImmutableKey::new("key"));
        assert_ne!(ImmutableKey::new("key"), ImmutableKey::new("key2"));

        println!("{:?}", ImmutableKey::new("key"));

        let k = ImmutableKey::new("key");
        assert_eq!(&k, &k.clone());

        let b: Box<dyn Key> = Box::new("key");
        assert_eq!(ImmutableKey::new("key"), ImmutableKey::wrap(b.clone()));

        assert_eq!(&b, &k.raw_boxed());

        let clone = k.clone();
        let handle = thread::spawn(move || {
            println!("data: {:?}", clone);
        });
        handle.join().unwrap();

        let mut hasher = DefaultHasher::new();
        k.hash(&mut hasher);
        let _ = hasher.finish();
    }
}
