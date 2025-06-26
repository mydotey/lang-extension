use std::any::Any;
use std::fmt::Debug;

use super::*;

pub trait ValueConstraint: PartialEq + Debug + Clone + Any {}

impl<T: ?Sized + PartialEq + Debug + Clone + Any> ValueConstraint for T {}

pub trait Value: Any + AnyExtension + AsAny + Debug {
    fn equals(&self, other: &dyn Any) -> bool {
        self.reference_equals(other)
    }

    as_trait!(Value);
    as_boxed!(Value);
}

impl<T: ?Sized + ValueConstraint> Value for T {
    fn equals(&self, other: &dyn Any) -> bool {
        match other.downcast_ref::<T>() {
            Some(r) => *self == *r,
            None => false,
        }
    }

    as_trait!(impl Value);
    as_boxed!(impl Value);
}

#[macro_export]
macro_rules! boxed_value_trait {
    ($trait:tt) => {
as_boxed!(impl PartialEq for $trait);
as_boxed!(impl Clone for $trait);
    };

    ($trait:tt<$($param:tt), *>) => {
as_boxed!(impl PartialEq for $trait<$($param), *>);
as_boxed!(impl Clone for $trait<$($param), *>);
    };

    ($trait:tt<$($param:tt: $constraint0:tt $(+ $constraint:tt)*), *>) => {
as_boxed!(impl PartialEq for $trait<$($param: $constraint0 $(+ $constraint)*), *>);
as_boxed!(impl Clone for $trait<$($param: $constraint0 $(+ $constraint)*), *>);
    };

    ($trait:tt<$($param:tt: ?Sized + $constraint0:tt $(+ $constraint:tt)*), *>) => {
as_boxed!(impl PartialEq for $trait<$($param: ?Sized + $constraint0 $(+ $constraint)*), *>);
as_boxed!(impl Clone for $trait<$($param: ?Sized + $constraint0 $(+ $constraint)*), *>);
    };

    ($trait:tt<$($param:tt: 'static + $constraint0:tt $(+ $constraint:tt)*), *>) => {
as_boxed!(impl PartialEq for $trait<$($param: 'static + $constraint0 $(+ $constraint)*), *>);
as_boxed!(impl Clone for $trait<$($param: 'static + $constraint0 $(+ $constraint)*), *>);
    };

    ($trait:tt<$($param:tt: 'static + ?Sized + $constraint0:tt $(+ $constraint:tt)*), *>) => {
as_boxed!(impl PartialEq for $trait<$($param: 'static + ?Sized + $constraint0 $(+ $constraint)*), *>);
as_boxed!(impl Clone for $trait<$($param: 'static + ?Sized + $constraint0 $(+ $constraint)*), *>);
    };
}

boxed_value_trait!(Value);

#[allow(dead_code)]
#[cfg(test)]
mod tests {

    use super::*;

    #[derive(PartialEq, Eq, Debug, Clone)]
    struct S1 {
        a: i32,
        b: u32,
    }

    #[test]
    fn eq() {
        let s = S1 { a: 1, b: 2 };
        let t: Box<dyn Value> = Box::new(s.clone());
        let t2: Box<dyn Value> = Box::new(s.clone());
        assert!(t == t2);
    }

    #[test]
    fn debug() {
        let s = S1 { a: 1, b: 2 };
        let t: Box<dyn Value> = Box::new(s);
        println!("{:?}", t);
    }

    #[test]
    fn clone() {
        let s = S1 { a: 1, b: 2 };
        let b = s.clone_boxed();
        assert_eq!(type_name::<S1>(), b.as_ref().type_name());
        assert_eq!(type_name::<Box<dyn Value>>(), b.type_name());
        assert!(s.equals(b.as_ref().as_any_ref()));
    }

    #[test]
    fn as_trait_as_boxed() {
        let s = S1 { a: 1, b: 2 };
        s.as_trait_ref();

        println!("{:p}", s.as_trait_ref() as &dyn Value);

        let mut s = s;
        s.as_trait_mut();

        assert_eq!(&s.clone_boxed(), &s.to_boxed());
    }

    trait SomeType0<K: KeyConstraint, V: ValueConstraint>: Value {
        fn say(&self, k: K, v: V);

        as_boxed!(SomeType0<K, V>);
        as_trait!(SomeType0<K, V>);
    }

    boxed_value_trait!(SomeType0<K: KeyConstraint, V: ValueConstraint>);

    trait SomeType1<K: KeyConstraint + Debug, V: ValueConstraint>: Value {
        fn say(&self, k: K, v: V);

        as_boxed!(SomeType1<K, V>);
        as_trait!(SomeType1<K, V>);
    }

    boxed_value_trait!(SomeType1<K: KeyConstraint + Debug, V: ValueConstraint>);

    trait SomeType2<K: ?Sized + KeyConstraint, V: ?Sized + ValueConstraint>: Value {
        fn say(&self, k: K, v: V);

        as_boxed!(SomeType2<K, V>);
        as_trait!(SomeType2<K, V>);
    }

    boxed_value_trait!(SomeType2<K: ?Sized + KeyConstraint, V: ?Sized + ValueConstraint>);

    trait SomeType3<K: 'static + KeyConstraint, V: 'static + ValueConstraint>: Value {
        fn say(&self, k: K, v: V);

        as_boxed!(SomeType3<K, V>);
        as_trait!(SomeType3<K, V>);
    }

    boxed_value_trait!(SomeType3<K: 'static + KeyConstraint, V: 'static + ValueConstraint>);

    trait SomeType4<K: 'static + ?Sized + KeyConstraint, V: 'static + ?Sized + ValueConstraint>:
        Value
    {
        fn say(&self, k: K, v: V);

        as_boxed!(SomeType4<K, V>);
        as_trait!(SomeType4<K, V>);
    }

    boxed_value_trait!(SomeType4<K: 'static + ?Sized + KeyConstraint, V: 'static + ?Sized + ValueConstraint>);
}
