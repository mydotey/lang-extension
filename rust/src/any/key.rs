use std::hash::{ Hash, Hasher };
use std::collections::hash_map::DefaultHasher;

use super::*;

pub trait KeyConstraint: ValueConstraint + Hash { }

impl<T: ?Sized + ValueConstraint + Hash> KeyConstraint for T { }

pub trait Key: Value {
    fn hashcode(&self) -> u64 {
        self.memory_address() as u64
    }

as_trait!(Key);
as_boxed!(Key);
}

impl<T: ?Sized + KeyConstraint> Key for T {
    fn hashcode(&self) -> u64 {
        let mut hasher = DefaultHasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }

as_trait!(impl Key);
as_boxed!(impl Key);
}

#[macro_export]
macro_rules! boxed_key_trait {
    ($trait: tt) => {
as_boxed!(impl Hash for $trait);
boxed_value_trait!($trait);
    };
}

boxed_key_trait!(Key);

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
    fn hashcode() {
        let s = S1 {
            a: 1,
            b: 2
        };
        assert_eq!(s.hashcode(), s.clone().hashcode());

        let bs: Box<dyn Key> = Box::new(s);
        assert_eq!(bs.hashcode(), bs.clone().hashcode());

        let mut hasher = DefaultHasher::new();
        bs.hash(&mut hasher);
        hasher.finish();
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
        let key: Box<dyn Key> = Box::new(k.clone());
        let value: Box<dyn Value> = Box::new(v.clone());
        let mut map = HashMap::<Box<dyn Key>, Box<dyn Value>>::new();
        map.insert(key.clone(), value.clone());
        assert_eq!(&value, map.get(&key).unwrap());
    }

}
