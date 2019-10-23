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
    ($trait:tt) => {
as_boxed!(impl Hash for $trait);
boxed_value_trait!($trait);
    };

    ($trait:tt<$($param:tt), *>) => {
as_boxed!(impl Hash for $trait<$($param), *>);
boxed_value_trait!($trait<$($param), *>);
    };

    ($trait:tt<$($param:tt: $constraint:tt), *>) => {
as_boxed!(impl Hash for $trait<$($param: $constraint), *>);
boxed_value_trait!($trait<$($param: $constraint), *>);
    };
}

boxed_key_trait!(Key);

#[cfg(test)]
mod tests {

    use crate::*;
    use super::*;
    use std::collections::HashMap;

    trait K1<K: KeyConstraint>: Key {
        fn say(&self, _k: K) { }

    as_boxed!(K1<K>);
    }

    #[derive(Hash, PartialEq, Eq, Debug, Clone)]
    struct S1 {
        a: i32,
        b: u32
    }

    impl<K: KeyConstraint> K1<K> for S1 {
    as_boxed!(impl K1<K>);
    }

    boxed_key_trait!(K1<K: KeyConstraint>);

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
