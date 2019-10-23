use std::fmt::Debug;
use std::any::Any;

use super::*;

pub trait ValueConstraint: 'static + PartialEq + Eq + Debug + Clone { }

impl<T: 'static + ?Sized + PartialEq + Eq + Debug + Clone> ValueConstraint for T { }

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
            None => false
        }
    }
 
as_trait!(impl Value);
as_boxed!(impl Value);
}

#[macro_export]
macro_rules! boxed_value_trait {
    ($trait:tt) => {
as_boxed!(impl PartialEq for $trait);
as_boxed!(impl Eq for $trait);
as_boxed!(impl Clone for $trait);
    };

    ($trait:tt<$($param:tt), *>) => {
as_boxed!(impl PartialEq for $trait<$($param), *>);
as_boxed!(impl Eq for $trait<$($param), *>);
as_boxed!(impl Clone for $trait<$($param), *>);
    };
}

boxed_value_trait!(Value);

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(PartialEq, Eq, Debug, Clone)]
    struct S1 {
        a: i32,
        b: u32
    }

    #[test]
    fn eq() {
        let s = S1 {
            a: 1,
            b: 2
        };
        let t: Box<dyn Value> = Box::new(s.clone());
        let t2: Box<dyn Value> = Box::new(s.clone());
        assert!(t == t2);
    }

    #[test]
    fn debug() {
        let s = S1 {
            a: 1,
            b: 2
        };
        let t: Box<dyn Value> = Box::new(s);
        println!("{:?}", t);
    }

    #[test]
    fn clone() {
        let s = S1 {
            a: 1,
            b: 2
        };
        let b = s.clone_boxed();
        assert_eq!(type_name::<S1>(), b.as_ref().type_name());
        assert_eq!(type_name::<Box<dyn Value>>(), b.type_name());
        assert!(s.equals(b.as_ref().as_any_ref()));
    }

    #[test]
    fn as_trait_as_boxed() {
        let s = S1 {
            a: 1,
            b: 2
        };
        s.as_trait_ref();

        println!("{:p}", s.as_trait_ref() as &dyn Value);

        let mut s = s;
        s.as_trait_mut();

        assert_eq!(&s.clone_boxed(), &s.to_boxed());
    }

}
