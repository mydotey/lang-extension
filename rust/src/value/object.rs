use std::hash::{ Hash, Hasher };
use std::fmt::{ Debug, Formatter, Result };
use std::any::{ type_name, Any, TypeId };
use std::collections::hash_map::DefaultHasher;

pub trait Object: 'static {

    fn hashcode(&self) -> u64;

    fn equals(&self, other: &dyn Object) -> bool;

    fn to_debug_string(&self) -> String;

    fn clone_boxed(&self) -> Box<dyn Object>;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn raw_type_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn raw_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

}

impl<T: 'static + Hash + PartialEq + Eq + Debug + Clone> Object for T {

    fn hashcode(&self) -> u64 {
        let mut hasher = DefaultHasher::default();
        T::hash(self, &mut hasher);
        hasher.finish()
    }

    fn equals(&self, other: &dyn Object) -> bool {
        match other.as_any().downcast_ref::<T>() {
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

}

impl Hash for Box<dyn Object> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.as_ref().hashcode());
    }
}

impl PartialEq for Box<dyn Object> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().equals(other.as_ref())
    }
}

impl Eq for Box<dyn Object> { }

impl Debug for Box<dyn Object> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.as_ref().to_debug_string())
    }
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        self.as_ref().clone_boxed()
    }
}

pub fn downcast_raw<T: 'static + Clone>(obj: Box<dyn Object>) -> Option<T> {
    if obj.as_ref().raw_type_id() == TypeId::of::<T>() {
        let r =  unsafe { &*(obj.as_ref() as *const dyn Object as *const T) };
        Some(r.clone())
    } else {
        None
    }
}

pub fn downcast_ref<T: 'static>(obj: &dyn Object) -> Option<&T> {
    if obj.raw_type_id() == TypeId::of::<T>() {
        let r =  unsafe { &*(obj as *const dyn Object as *const T) };
        Some(r)
    } else {
        None
    }
}

pub fn downcast_mut<T: 'static>(obj: &mut dyn Object) -> Option<&mut T> {
    if obj.raw_type_id() == TypeId::of::<T>() {
        let r =  unsafe { &mut *(obj as *mut dyn Object as *mut T) };
        Some(r)
    } else {
        None
    }
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
        println!("Box<Object>: {}", t.raw_type_name());
        println!("Object: {}", t.as_ref().raw_type_name());
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
