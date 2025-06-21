use std::any::{Any, type_name};

#[macro_use]
mod as_macro;

#[macro_use]
mod value;
pub use value::*;

#[macro_use]
mod key;
pub use key::*;

mod immutable;
pub use immutable::*;

pub trait AsAny {
    fn as_any_ref(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> AsAny for T {
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait AnyExtension {
    fn type_name(&self) -> &'static str;
    fn memory_address(&self) -> usize;
    fn reference_equals(&self, other: &dyn Any) -> bool;
}

impl<T: ?Sized> AnyExtension for T {
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn memory_address(&self) -> usize {
        self as *const T as *const () as usize
    }

    fn reference_equals(&self, other: &dyn Any) -> bool {
        self.memory_address() == other.memory_address()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::TypeId;

    #[test]
    fn as_any() {
        let d = 1;
        assert_eq!(d.as_any_ref().type_id(), TypeId::of::<i32>());

        let mut d = d;
        *d.as_any_mut().downcast_mut::<i32>().unwrap() = 2;
        assert_eq!(2, d);
    }

    #[test]
    fn any_extension() {
        let a: Box<dyn Any> = Box::new(10);
        println!(
            "type_name: {}, memory_address: {:?}",
            a.as_ref().type_name(),
            a.as_ref().memory_address()
        );

        assert_eq!(type_name::<i32>(), 10.type_name());
        assert_eq!(type_name::<dyn Any>(), a.as_ref().type_name());
        assert_eq!(type_name::<Box<dyn Any>>(), a.type_name());

        assert_eq!(
            a.as_ref().downcast_ref::<i32>().unwrap() as *const i32 as usize,
            a.as_ref().memory_address()
        );
        assert_eq!(
            &a as *const Box::<dyn Any> as *const () as usize,
            a.memory_address()
        );

        let b: Box<dyn Any> = Box::new(10);
        assert!(!a.reference_equals(b.as_any_ref()));

        let x = Box::new(10);
        assert_eq!(x, x);
        assert!(x.reference_equals(x.as_any_ref()));

        let y = Box::new(10);
        assert_eq!(x, y);
        assert!(!x.reference_equals(y.as_any_ref()));
        assert!(!x.as_ref().reference_equals(y.as_ref().as_any_ref()));
    }
}
