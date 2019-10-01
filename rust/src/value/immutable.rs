use std::sync::Arc;
use std::any::TypeId;

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

    pub fn raw_object(&self) -> Box<dyn Object> {
        self.value.as_ref().clone()
    }

    pub fn downcast_raw<T: 'static + Clone>(&self) -> Option<T> {
        if self.value.as_ref().as_ref().raw_type_id() == TypeId::of::<T>() {
            let r =  unsafe { &*(self.value.as_ref().as_ref() as *const dyn Object as *const T) };
            Some(r.clone())
        } else {
            None
        }
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
    fn downcast() {
        let s = "key";
        let obj = ImmutableObject::new(s);
        let s2: &str = obj.downcast_raw().unwrap();
        println!("s: {}, s2: {}", s, s2);
        assert_eq!(s, s2);
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