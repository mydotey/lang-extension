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

    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn type_id(&self) -> TypeId {
        self.as_any().type_id()
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
