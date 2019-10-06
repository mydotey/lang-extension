use std::any::{ Any, type_name };
use std::string::*;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait AsAnyMut {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> AsAnyMut for T {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait AnyExtension {
    fn type_name(&self) -> &'static str;
    fn address(&self) -> usize;
    fn string(&self) -> String;
    fn reference_equals(&self, other: &dyn Any) -> bool;
}

impl<T: ?Sized> AnyExtension for T {
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn address(&self) -> usize {
        self as *const T as *const () as usize
    }

    fn string(&self) -> String {
        format!("{{ type: {}, address: {} }}", self.type_name(), self.address())
    }

    fn reference_equals(&self, other: &dyn Any) -> bool {
        self.address() == other.address()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_extension() {
        let a: Box<dyn AnyExtension> = Box::new(10);
        println!("string: {}, type_name: {}, address: {:?}",
            a.string(), a.as_ref().type_name(), a.as_ref().address());

        let b: Box<dyn AnyExtension> = Box::new(10);
        println!("string: {}, type_name: {}, address: {:?}",
            b.string(), b.as_ref().type_name(), b.as_ref().address());

        println!("a == b: {}", a.as_ref().reference_equals(b.as_any()));
        assert!(!a.as_ref().reference_equals(b.as_any()));
    }
}