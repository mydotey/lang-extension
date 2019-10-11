use std::hash::{ Hash, Hasher };
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::any::Any;

use crate::any::*;

pub trait ObjectConstraits: 'static + Hash + PartialEq + Eq + Debug + Clone { }

impl<T: 'static + Hash + PartialEq + Eq + Debug + Clone> ObjectConstraits for T { }

pub trait Object: 'static + AnyExtension + AsAny + AsAnyMut {

    fn hashcode(&self) -> u64;

    fn equals(&self, other: &dyn Any) -> bool;

    fn to_debug_string(&self) -> String;

    fn clone_boxed(&self) -> Box<dyn Object>;

    fn as_object(&self) -> &dyn Object;

}

impl<T: ObjectConstraits> Object for T {
    fn hashcode(&self) -> u64 {
        let mut hasher = DefaultHasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn equals(&self, other: &dyn Any) -> bool {
        match other.downcast_ref::<T>() {
            Some(r) => *self == *r,
            None => false
        }
    }
 
    fn to_debug_string(&self) -> String {
        format!("{:?}", self)
    }

    fn clone_boxed(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }

    fn as_object(&self) -> &dyn Object {
        self
    }

}

#[allow(unused_macros)]
#[macro_export]
macro_rules! boxed_trait_object {
    ($type: tt) => {
impl std::hash::Hash for Box<dyn $type> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.as_ref().hashcode());
    }
}

impl PartialEq for Box<dyn $type> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().equals(other.as_ref().as_any())
    }
}

impl Eq for Box<dyn $type> { }

impl std::fmt::Debug for Box<dyn $type> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_ref().to_debug_string())
    }
}

impl Clone for Box<dyn $type> {
    fn clone(&self) -> Self {
        $type::clone_boxed(self.as_ref())
    }
}
    };
}

boxed_trait_object!(Object);

pub fn downcast_raw<T: 'static + Clone>(obj: Box<dyn Object>) -> Option<T> {
    match obj.as_ref().as_any().downcast_ref::<T>() {
        Some(r) => Some(r.clone()),
        None => None
    }
}

pub fn downcast_ref<T: 'static>(obj: &dyn Object) -> Option<&T> {
    obj.as_any().downcast_ref::<T>()
}

pub fn downcast_mut<T: 'static>(obj: &mut dyn Object) -> Option<&mut T> {
    obj.as_any_mut().downcast_mut::<T>()
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    #[derive(Hash, PartialEq, Eq, Debug, Clone)]
    struct S1 {
        a: i32,
        b: u32
    }

    #[test]
    fn hash() {
        let s = S1 {
            a: 1,
            b: 2
        };
        println!("hashcode: {}", s.hashcode());

        let bs: Box<dyn Object> = Box::new(s);
        println!("hashcode: {}", bs.hashcode());
    }

    #[test]
    fn eq() {
        let s = S1 {
            a: 1,
            b: 2
        };
        let t: Box<dyn Object> = Box::new(s.clone());
        let t2: Box<dyn Object> = Box::new(s.clone());
        assert!(t == t2);
    }

    #[test]
    fn to_debug_string() {
        let s = S1 {
            a: 1,
            b: 2
        };
        let t: Box<dyn Object> = Box::new(s);
        println!("to_debug_string: {:?}", t);
    }

    #[test]
    fn type_name() {
        let s = S1 {
            a: 1,
            b: 2
        };
        let t: Box<dyn Object> = Box::new(s);
        println!("Box<Object>: {}", t.type_name());
        println!("Object: {}", t.as_ref().type_name());
    }

    #[test]
    fn type_id() {
        let s = S1 {
            a: 1,
            b: 2
        };
        let t: Box<dyn Object> = Box::new(s);
        println!("Box<Object>: {:?}", t.type_id());
        println!("Object: {:?}", t.as_ref().type_id());
    }

    #[test]
    fn as_any() {
        let s = S1 {
            a: 1,
            b: 2
        };
        let t: Box<dyn Object> = Box::new(s);
        println!("s: {:?}", t.as_any().downcast_ref::<Box<dyn Object>>().unwrap());
        println!("s: {:?}", t.as_ref().as_any().downcast_ref::<S1>().unwrap());
    }

    #[test]
    fn as_any_mut() {
        let s = S1 {
            a: 1,
            b: 2
        };
        let mut t: Box<dyn Object> = Box::new(s);
        t.as_mut().as_any_mut().downcast_mut::<S1>().unwrap().a = 11;
        println!("s: {:?}", t.as_any_mut().downcast_mut::<Box<dyn Object>>().unwrap());
        println!("s: {:?}", t.as_ref().as_any().downcast_ref::<S1>().unwrap());
    }

    #[test]
    fn downcast_raw() {
        let s = S1 {
            a: 1,
            b: 2
        };
        let obj: Box<dyn Object> = Box::new(s.clone());
        let s2 = super::downcast_raw::<S1>(obj).unwrap();
        println!("s: {:?}", s);
        println!("s2: {:?}", s2);
        assert_eq!(s, s2);
    }

    #[test]
    fn downcast_ref() {
        let s = S1 {
            a: 1,
            b: 2
        };
        let obj: Box<dyn Object> = Box::new(s.clone());
        let s2 = super::downcast_ref::<S1>(obj.as_ref()).unwrap();
        println!("s: {:?}", s);
        println!("s2: {:?}", s2);
        assert_eq!(s, *s2);
    }

    #[test]
    fn downcast_mut() {
        let s = S1 {
            a: 1,
            b: 2
        };
        let mut obj: Box<dyn Object> = Box::new(s.clone());
        {
                let s2 = super::downcast_mut::<S1>(obj.as_mut()).unwrap();
                s2.a = 10;
                println!("s2: {:?}", s2);
        }
        println!("s: {:?}", s);
        println!("s2: {:?}", obj);
        assert_eq!(10, super::downcast_ref::<S1>(obj.as_ref()).unwrap().a);
    }

    #[test]
    fn hashmap() {
        let k = S1 {
            a: 1,
            b: 2
        };
        let v = S1 {
            a: 11,
            b: 22
        };
        let key: Box<dyn Object> = Box::new(k.clone());
        let value: Box<dyn Object> = Box::new(v.clone());
        let mut map = HashMap::<Box<dyn Object>, Box<dyn Object>>::new();
        map.insert(key.clone(), value.clone());
        assert_eq!(&value, map.get(&key).unwrap());
    }

}
