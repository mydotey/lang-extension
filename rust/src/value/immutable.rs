use std::sync::Arc;

use super::object::*;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct ImmutableObject {
    value: Arc<Box<dyn Object>>
}

impl ImmutableObject {
    pub fn new<T: Object>(value: T) -> ImmutableObject {
        ImmutableObject {
            value: Arc::new(Box::new(value))
        }
    }

    pub fn value(&self) -> Box<dyn Object> {
        self.value.as_ref().clone()
    }
}

unsafe impl Send for ImmutableObject {

}

unsafe impl Sync for ImmutableObject {

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::thread;

    #[test]
    fn object() {
        assert_eq!(ImmutableObject::new("key"), ImmutableObject::new("key"));
        assert_ne!(ImmutableObject::new("key"), ImmutableObject::new("value"));
    }

    #[test]
    fn generic_map() {
        let mut map = HashMap::<ImmutableObject, ImmutableObject>::new();
        map.insert(ImmutableObject::new("key"), ImmutableObject::new("value"));
        println!("map: {:?}", map.get(&ImmutableObject::new("key")).unwrap());
    }

    #[test]
    fn test() {
        let object = ImmutableObject::new("data");
        println!("data: {:?}", object);
        let clone = object.clone();
        let handle = thread::spawn(move || {
            println!("data: {:?}", clone);
        });
        handle.join().unwrap();
    }

}